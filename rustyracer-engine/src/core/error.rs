//! Central error handling subsystem for the engine

use thiserror::Error;

/// Engine error types with custom formatting
#[derive(Error, Debug)]
pub enum EngineError {
    /// Initialization failed
    #[error("Initialization failed: {0}")]
    InitError(String),

    /// Asset loading error
    #[error("Asset load error: {0}")]
    AssetError(String),

    /// Invalid operation error
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Platform error from winit
    #[error("Platform error: {0}")]
    PlatformError(#[from] winit::error::OsError),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Result type alias for engine operations
pub type EngineResult<T> = Result<T, EngineError>;
