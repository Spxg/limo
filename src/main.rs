mod server;

use std::{
    io::{self, Write},
    path::PathBuf,
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

use crate::server::{LocalServer, script_router};

#[derive(Parser)]
#[command(version, about = "Run JavaScript with moli's browser Web APIs")]
struct Cli {
    /// JavaScript entry file.
    file: PathBuf,
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
    let server = LocalServer::start(script_router(&cli.file)?, cli.port.unwrap_or(0)).await?;

    let mut config = BrowserConfig::default();
    config.set_profile_dir(cli.profile_dir);

    let browser = Browser::new(config).context("failed to initialize moli")?;
    let (output_tx, output_rx) = moli_core::renderer_output_transport_channel();
    browser.set_renderer_output_transport_sender(output_tx);
    let console = tokio::spawn(forward_console(output_rx));

    let execution: Result<()> = async {
        browser
            .fetch_with_wait_until(
                server.base_url(),
                RenderedDomWaitUntil::Idle,
                Duration::from_millis(cli.timeout_ms.unwrap_or(600_000)),
            )
            .await
            .context("JavaScript execution failed")?;
        Ok(())
    }
    .await;

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
