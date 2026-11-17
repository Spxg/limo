use std::io::{self, Write};

use anyhow::{Result, bail};
use moli_core::{
    RendererOutputItem, RendererOutputTransportMessage, RendererOutputTransportReceiver,
    RendererOwnerAction, RendererProtocolObservation,
    page::{
        RendererDedicatedWorkerTargetEvent, RendererServiceWorkerTargetEvent,
        RendererSharedWorkerTargetEvent,
    },
};

pub async fn forward(mut output: RendererOutputTransportReceiver, entry_url: String) -> Result<()> {
    while let Some(message) = output.recv().await {
        let RendererOutputTransportMessage::Publication(publication) = message else {
            continue;
        };

        for record in publication.into_records() {
            let item = record.into_parts().1;
            match &item {
                RendererOutputItem::Observation(RendererProtocolObservation::RuntimeException(
                    exception,
                )) => {
                    if exception.handled {
                        continue;
                    }
                    bail!(
                        "{}",
                        exception.stack.as_deref().unwrap_or(&exception.message)
                    );
                }
                RendererOutputItem::Observation(
                    RendererProtocolObservation::ScriptLoadFailure { message, url },
                ) => {
                    if url == &entry_url {
                        bail!("{message}");
                    }
                    let mut stderr = io::stderr().lock();
                    writeln!(stderr, "{message}")?;
                    stderr.flush()?;
                    continue;
                }
                _ => {}
            }

            let (message, stack) = match item {
                RendererOutputItem::Observation(RendererProtocolObservation::RuntimeConsole(
                    message,
                )) => (message.message, message.stack),
                RendererOutputItem::OwnerAction(
                    RendererOwnerAction::DedicatedWorkerTargetLifecycle(
                        RendererDedicatedWorkerTargetEvent::Console { message, .. },
                    ),
                ) => (message.message, message.stack),
                RendererOutputItem::OwnerAction(
                    RendererOwnerAction::SharedWorkerTargetLifecycle(
                        RendererSharedWorkerTargetEvent::Console { message, .. },
                    ),
                ) => (message.message, message.stack),
                RendererOutputItem::OwnerAction(
                    RendererOwnerAction::ServiceWorkerTargetLifecycle(
                        RendererServiceWorkerTargetEvent::Console { message, .. },
                    ),
                ) => (message.message, message.stack),
                _ => continue,
            };

            let (level, text) = message.split_once(": ").unwrap_or(("", &message));
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "{text}")?;

            // moli captures stacks for all console messages; only console.trace() requests printing one.
            if level == "trace"
                && let Some(stack) = stack
            {
                writeln!(stdout, "{stack}")?;
            }

            stdout.flush()?;
        }
    }

    if output.is_terminal() {
        bail!("renderer output transport terminated unexpectedly");
    }

    Ok(())
}
