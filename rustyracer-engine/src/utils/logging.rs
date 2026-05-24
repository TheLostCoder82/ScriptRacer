//! Logging initialization for the engine

use env_logger::Builder;
use log::LevelFilter;

use crate::core::EngineConfig;

/// Initializes logging based on engine configuration
pub fn init_logging(config: &EngineConfig) {
    let level = match config.log_level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .format_timestamp_millis()
        .format_module_path(true)
        .filter(None, level)
        .init();

    log::info!("Logging initialized with level: {}", config.log_level);
}
