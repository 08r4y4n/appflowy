use log::SetLoggerError;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn init_log(name: &str, env_filter: &str) -> std::Result<(), SetLoggerError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter.to_owned()));
    let formatting_layer = BunyanFormattingLayer::new(name.to_owned(), std::io::stdout);
    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_target(false)
        .compact()
        .finish()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    let _ = LogTracer::init()?;
    let _ = set_global_default(subscriber)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        init_log("flowy-log", "info");
        tracing::info!("😁 Tracing info log");
        log::info!("😁 bridge 'log' to 'tracing'");
    }
}
