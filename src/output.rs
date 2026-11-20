use std::{
    fmt::Write as _,
    io::{self, Write},
};

use anyhow::{Context, Result, bail};
use moli_core::{
    DiagnosticEvent, DiagnosticSource, DiagnosticsSubscription, ErrorReportOutcome,
    ExceptionOrigin, RendererOutputItem, RendererOutputTransportMessage,
    RendererOutputTransportReceiver, RendererOwnerAction, RendererProtocolObservation,
    page::{
        RendererDedicatedWorkerTargetEvent, RendererServiceWorkerTargetEvent,
        RendererSharedWorkerTargetEvent,
    },
};

pub async fn forward(mut output: RendererOutputTransportReceiver) -> Result<()> {
    while let Some(message) = output.recv().await {
        let RendererOutputTransportMessage::Publication(publication) = message else {
            continue;
        };

        for record in publication.into_records() {
            let item = record.into_parts().1;
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

pub async fn forward_diagnostics(
    diagnostics: &mut DiagnosticsSubscription,
    entry_url: &str,
) -> Result<()> {
    loop {
        let event = diagnostics
            .recv()
            .await
            .context("moli diagnostics subscription closed")?;
        report_diagnostic(&event, entry_url)?;
    }
}

pub fn drain_diagnostics(diagnostics: &mut DiagnosticsSubscription, entry_url: &str) -> Result<()> {
    while let Ok(event) = diagnostics.try_recv() {
        report_diagnostic(&event, entry_url)?;
    }
    Ok(())
}

fn report_diagnostic(event: &DiagnosticEvent, entry_url: &str) -> Result<()> {
    match event {
        DiagnosticEvent::RuntimeException {
            origin,
            message,
            filename,
            lineno,
            colno,
            stack,
            outcome: ErrorReportOutcome::Uncanceled,
            ..
        } => {
            let mut error = message.clone();
            if !filename.is_empty() && *lineno > 0 {
                let location = if matches!(origin, ExceptionOrigin::UncaughtException) {
                    "thrown at"
                } else {
                    "at"
                };
                write!(&mut error, "\n    {location} {filename}:{lineno}")?;
                if *colno > 0 {
                    write!(&mut error, ":{colno}")?;
                }
            }
            if let Some(stack) = stack {
                for frame in stack.lines() {
                    write!(&mut error, "\n    {frame}")?;
                }
            }
            bail!("{error}");
        }
        DiagnosticEvent::ScriptLoadFailure {
            source,
            message,
            entry_url: failed_entry_url,
            ..
        } => {
            if matches!(source, DiagnosticSource::Page { .. }) && failed_entry_url == entry_url {
                bail!("{message}");
            }
            let mut stderr = io::stderr().lock();
            writeln!(stderr, "{message}")?;
            stderr.flush()?;
        }
        _ => {}
    }
    Ok(())
}
