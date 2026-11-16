mod server;

use std::{
    fs,
    io::{self, IsTerminal, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, ValueEnum};
use moli_core::{
    RendererOutputItem, RendererOutputTransportMessage, RendererOutputTransportReceiver,
    RendererOwnerAction, RendererProtocolObservation,
    page::{
        RendererDedicatedWorkerTargetEvent, RendererServiceWorkerTargetEvent,
        RendererSharedWorkerTargetEvent,
    },
    protocol_types::ScriptRunOutcome,
    runtime::{Browser, BrowserConfig, RenderedDomWaitUntil},
};
use oxc_allocator::Allocator;
use oxc_parser::Parser as OxcParser;
use oxc_span::SourceType;
use tokio::runtime::Builder;

use crate::server::{LocalServer, script_router, source_router};

#[derive(Clone, Copy, ValueEnum)]
enum InputType {
    Script,
    Module,
}

#[derive(Parser)]
#[command(version, about = "Run JavaScript with moli's browser Web APIs")]
struct Cli {
    /// JavaScript entry file, or - to read standard input.
    file: Option<PathBuf>,
    /// Run JavaScript code.
    #[arg(
        short = 'e',
        long = "eval",
        value_name = "CODE",
        conflicts_with = "file"
    )]
    eval: Option<String>,
    /// JavaScript input type (inferred if omitted).
    #[arg(long, value_enum)]
    input_type: Option<InputType>,
    /// Local server port (random if omitted).
    #[arg(long)]
    port: Option<u16>,
    /// Directory for persistent browser state.
    #[arg(long)]
    profile_dir: Option<PathBuf>,
    /// Execution timeout in milliseconds (default: 10 minutes).
    #[arg(long)]
    timeout_ms: Option<u64>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runtime = Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .context("failed to initialize tokio runtime")?;
    runtime.block_on(run(cli))
}

async fn run(cli: Cli) -> Result<()> {
    let source = match cli.eval {
        Some(source) => Some(source),
        None => match cli.file.as_deref() {
            Some(path) if path != Path::new("-") => {
                ensure!(
                    path.is_file(),
                    "JavaScript entry is not a file: {}",
                    path.display()
                );
                None
            }
            None if io::stdin().is_terminal() => {
                bail!("provide a JavaScript file, --eval, or standard input; see --help")
            }
            _ => {
                let mut source = String::new();
                io::stdin()
                    .read_to_string(&mut source)
                    .context("failed to read JavaScript from standard input")?;
                Some(source)
            }
        },
    };
    let timeout = Duration::from_millis(cli.timeout_ms.unwrap_or(600_000));
    let mut config = BrowserConfig::default();
    config.set_profile_dir(cli.profile_dir);
    let browser = Browser::new(config).context("failed to initialize moli")?;

    let (output_tx, output_rx) = moli_core::renderer_output_transport_channel();
    browser.set_renderer_output_transport_sender(output_tx);
    let console = tokio::spawn(forward_console(output_rx));

    let execution: Result<()> = tokio::time::timeout(timeout, async {
        let input_type = match cli.input_type {
            Some(input_type) => input_type,
            None => match &source {
                Some(source) => detect_input_type(source),
                None => {
                    let path = cli
                        .file
                        .as_deref()
                        .context("missing JavaScript entry file")?;
                    match path.extension().and_then(|extension| extension.to_str()) {
                        Some("mjs") => InputType::Module,
                        Some("cjs") => bail!("CommonJS (.cjs) entry files are not supported"),
                        _ => {
                            let source = fs::read_to_string(path).with_context(|| {
                                format!("failed to read JavaScript entry: {}", path.display())
                            })?;
                            detect_input_type(&source)
                        }
                    }
                }
            },
        };

        let app = if let Some(source) = source {
            source_router(source, input_type)
        } else {
            script_router(
                cli.file
                    .as_deref()
                    .context("missing JavaScript entry file")?,
                input_type,
            )?
        };
        let server = LocalServer::start(app, cli.port.unwrap_or(0)).await?;

        let page = browser
            .fetch_with_wait_until(server.base_url(), RenderedDomWaitUntil::Idle, timeout)
            .await
            .context("JavaScript execution failed")?;
        for run in page.script_execution().runs() {
            if let ScriptRunOutcome::Failed(message) = run.outcome() {
                bail!("{message}");
            }
        }
        Ok(())
    })
    .await
    .context("JavaScript execution timed out")
    .and_then(|result| result);

    // Stop all producers, then drain their queued console output before returning.
    drop(browser);
    let output = console.await.context("console forwarding task failed")?;

    execution?;
    output.context("failed to forward console output")?;
    Ok(())
}

fn detect_input_type(source: &str) -> InputType {
    let allocator = Allocator::default();
    let parsed = OxcParser::new(&allocator, source, SourceType::unambiguous()).parse();
    if parsed.program.source_type.is_module() {
        InputType::Module
    } else {
        InputType::Script
    }
}

async fn forward_console(mut output: RendererOutputTransportReceiver) -> Result<()> {
    while let Some(message) = output.recv().await {
        let RendererOutputTransportMessage::Publication(publication) = message else {
            continue;
        };
        for record in publication.into_records() {
            let message = match record.into_parts().1 {
                RendererOutputItem::Observation(RendererProtocolObservation::RuntimeConsole(
                    message,
                )) => message.message,
                RendererOutputItem::OwnerAction(
                    RendererOwnerAction::DedicatedWorkerTargetLifecycle(
                        RendererDedicatedWorkerTargetEvent::Console { message, .. },
                    ),
                ) => message.message,
                RendererOutputItem::OwnerAction(
                    RendererOwnerAction::SharedWorkerTargetLifecycle(
                        RendererSharedWorkerTargetEvent::Console { message, .. },
                    ),
                ) => message.message,
                RendererOutputItem::OwnerAction(
                    RendererOwnerAction::ServiceWorkerTargetLifecycle(
                        RendererServiceWorkerTargetEvent::Console { message, .. },
                    ),
                ) => message.message,
                _ => continue,
            };
            let text = message
                .split_once(": ")
                .map_or(message.as_str(), |(_, text)| text);
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "{text}")?;
            stdout.flush()?;
        }
    }
    if output.is_terminal() {
        bail!("renderer output transport terminated unexpectedly");
    }
    Ok(())
}
