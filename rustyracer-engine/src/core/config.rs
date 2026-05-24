//! Engine configuration management

use serde::{Deserialize, Serialize};

use crate::core::{EngineResult, EngineError};

/// Window configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "RustyRacer Engine".to_string(),
            width: 1280,
            height: 720,
            fullscreen: false,
            resizable: true,
        }
    }
}

/// Engine configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub window: WindowConfig,
    pub log_level: String,
    pub fixed_timestep: f32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            log_level: "info".to_string(),
            fixed_timestep: 1.0 / 60.0,
        }
    }
}

impl EngineConfig {
    /// Loads configuration from a JSON file
    pub fn load_from_file(path: &str) -> EngineResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: EngineConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Saves configuration to a JSON file
    pub fn save_to_file(&self, path: &str) -> EngineResult<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
