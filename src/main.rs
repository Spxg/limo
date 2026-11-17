mod output;
mod server;

use std::{
    fs,
    io::{self, IsTerminal, Read},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, ValueEnum};
use moli_core::runtime::{Browser, BrowserConfig, RenderedDomWaitUntil};
use oxc_allocator::Allocator;
use oxc_parser::Parser as OxcParser;
use oxc_span::SourceType;
use tokio::runtime::Builder;
use url::Url;

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
    /// Disable browser web security, including CORS checks.
    #[arg(long)]
    disable_web_security: bool,
    /// Execution timeout in milliseconds (default: 10 minutes).
    #[arg(long)]
    timeout_ms: Option<u64>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .context("failed to initialize tokio runtime")
        .and_then(|runtime| runtime.block_on(run(cli)));

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
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
    config.set_web_security_disabled(cli.disable_web_security);
    let browser = Browser::new(config).context("failed to initialize moli")?;

    let (output_tx, output_rx) = moli_core::renderer_output_transport_channel();
    browser.set_renderer_output_transport_sender(output_tx);

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

    let (app, name) = if let Some(source) = source {
        let name = "__limo_input__.js";
        (source_router(source, name, input_type), name)
    } else {
        let path = cli
            .file
            .as_deref()
            .context("missing JavaScript entry file")?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("JavaScript filename is not valid UTF-8")?;
        (script_router(path, input_type)?, name)
    };

    let server = LocalServer::start(app, cli.port.unwrap_or(0)).await?;
    let mut entry_url = Url::parse(server.base_url())?;
    entry_url.path_segments_mut().unwrap().push(name);

    let mut output_task = tokio::spawn(output::forward(output_rx, entry_url.into()));

    let execution = tokio::select! {
        result = tokio::time::timeout(
            timeout,
            browser.fetch_with_wait_until(server.base_url(), RenderedDomWaitUntil::Idle, timeout),
        ) => {
            result
                .context("JavaScript execution timed out")
                .and_then(|result| result.context("JavaScript execution failed"))
                .map(|_| ())
        }
        result = &mut output_task => return result.context("output forwarding task failed")?,
    };

    // Stop all producers, then drain their queued console output before returning.
    drop(browser);
    output_task
        .await
        .context("output forwarding task failed")??;

    execution
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
