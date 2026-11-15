mod server;

use std::{
    io::{self, IsTerminal, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use moli_core::{
    RendererOutputItem, RendererOutputTransportMessage, RendererOutputTransportReceiver,
    RendererProtocolObservation,
    runtime::{Browser, BrowserConfig, RenderedDomWaitUntil},
};
use tokio::runtime::Builder;

use crate::server::{LocalServer, blank_router, script_router};

#[derive(Parser)]
#[command(version, about = "Run JavaScript with moli's browser Web APIs")]
struct Cli {
    /// JavaScript entry file, or - to read standard input.
    file: Option<PathBuf>,
    /// Evaluate JavaScript code.
    #[arg(
        short = 'e',
        long = "eval",
        value_name = "CODE",
        conflicts_with = "file"
    )]
    eval: Option<String>,
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
            Some(path) if path != Path::new("-") => None,
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
    let app = if source.is_some() {
        blank_router()
    } else {
        script_router(
            cli.file
                .as_deref()
                .context("missing JavaScript entry file")?,
        )?
    };
    let server = LocalServer::start(app, cli.port.unwrap_or(0)).await?;

    let timeout = Duration::from_millis(cli.timeout_ms.unwrap_or(600_000));
    let mut config = BrowserConfig::default();
    config.set_profile_dir(cli.profile_dir);
    let browser = Browser::new(config).context("failed to initialize moli")?;

    let (output_tx, output_rx) = moli_core::renderer_output_transport_channel();
    browser.set_renderer_output_transport_sender(output_tx);
    let console = tokio::spawn(forward_console(output_rx));

    let execution: Result<()> = tokio::time::timeout(timeout, async {
        if let Some(source) = source {
            let mut page = browser.fetch(server.base_url()).await?;
            let result = page.evaluate_runtime_expression_async(&source).await?;
            if let Some(exception) = result.get("exception").and_then(|value| value.as_str()) {
                bail!("{exception}");
            }
            browser.wait_for_page_idle(&mut page, timeout).await?;
        } else {
            browser
                .fetch_with_wait_until(server.base_url(), RenderedDomWaitUntil::Idle, timeout)
                .await
                .context("JavaScript execution failed")?;
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

async fn forward_console(mut output: RendererOutputTransportReceiver) -> Result<()> {
    while let Some(message) = output.recv().await {
        let RendererOutputTransportMessage::Publication(publication) = message else {
            continue;
        };
        for record in publication.into_records() {
            if let RendererOutputItem::Observation(RendererProtocolObservation::RuntimeConsole(
                message,
            )) = record.into_parts().1
            {
                let text = message
                    .message
                    .split_once(": ")
                    .map_or(message.message.as_str(), |(_, text)| text);
                let mut stdout = io::stdout().lock();
                writeln!(stdout, "{text}")?;
                stdout.flush()?;
            }
        }
    }
    if output.is_terminal() {
        bail!("renderer output transport terminated unexpectedly");
    }
    Ok(())
}
