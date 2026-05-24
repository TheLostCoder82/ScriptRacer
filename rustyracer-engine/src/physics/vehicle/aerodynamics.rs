//! Aerodynamics calculations for vehicle physics

use glam::Vec3;

use super::components::Vehicle;
use crate::physics::world::PhysicsWorld;
use crate::ecs::entity::Transform;
use super::data::PhysicsConstants;

/// Update aerodynamic forces on the vehicle
pub fn update_aerodynamics(
    vehicle: &mut Vehicle,
    transform: &Transform,
    physics_world: &mut PhysicsWorld,
    dt: f32,
    air_density: f32,
) {
    let speed = vehicle.speed;
    let speed_squared = speed * speed;
    
    // Get forward direction in world space
    let forward = transform.rotation * Vec3::Z;
    let up = transform.rotation * Vec3::Y;
    
    // Calculate drag force: F_drag = 0.5 * rho * v² * Cd * A
    let drag_coefficient = vehicle.config.drag_coefficient;
    let frontal_area = vehicle.config.frontal_area;
    let drag_magnitude = 0.5 * air_density * speed_squared * drag_coefficient * frontal_area;
    
    // Drag opposes motion
    let drag_direction = if speed > 0.01 {
        -vehicle.velocity.normalize_or_zero()
    } else {
        Vec3::ZERO
    };
    
    let drag_force = drag_direction * drag_magnitude;
    
    // Calculate downforce: F_down = 0.5 * rho * v² * Cl * A
    // Using a simplified lift coefficient (negative for downforce)
    let lift_coefficient = -0.8; // Negative = downforce
    let downforce_magnitude = 0.5 * air_density * speed_squared * lift_coefficient.abs() * frontal_area;
    
    // Downforce acts downward in vehicle local space
    let downforce = -up * downforce_magnitude;
    
    // Apply forces at center of pressure (slightly forward of COM for stability)
    let center_of_pressure = transform.translation + forward * 0.3;
    
    let total_aero_force = drag_force + downforce;
    
    physics_world.apply_force_at_point(
        vehicle.rigid_body_handle,
        total_aero_force,
        center_of_pressure,
        true,
    );
}
