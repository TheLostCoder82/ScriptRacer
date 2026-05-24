#![warn(missing_docs)]
#![allow(dead_code)]

pub mod core;
pub mod ecs;
pub mod assets;
pub mod platform;
pub mod utils;
pub mod render;
pub mod physics;
pub mod audio;
pub mod ui;
pub mod base_lib;
pub mod input;
pub mod racing_lib;
pub mod editor;
pub mod networking;
pub mod optimization;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::*;
    pub use crate::ecs::*;
    pub use crate::assets::*;
    pub use crate::platform::*;
    pub use glam::*;
    pub use winit::event::VirtualKeyCode as KeyCode;
}
