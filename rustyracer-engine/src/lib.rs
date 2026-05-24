#![warn(missing_docs)]
#![allow(dead_code)]

pub mod core;
pub mod ecs;
pub mod assets;
pub mod platform;
pub mod utils;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::*;
    pub use crate::ecs::*;
    pub use crate::assets::*;
    pub use crate::platform::*;
    pub use glam::*;
    pub use winit::event::VirtualKeyCode as KeyCode;
}
