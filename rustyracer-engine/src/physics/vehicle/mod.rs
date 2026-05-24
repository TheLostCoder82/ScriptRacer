//! Vehicle Physics Module
//! 
//! Custom vehicle physics simulation for racing games.

pub mod components;
pub mod data;
pub mod suspension;
pub mod tires;
pub mod drivetrain;
pub mod aerodynamics;
pub mod system;
pub mod utils;

// Re-export types for the engine API surface
pub use components::{Vehicle, Wheel, DriveType, Gear, TransmissionType};
pub use data::{VehicleConfig, PhysicsConstants, SurfaceType};
pub use system::VehiclePhysicsSystem;
