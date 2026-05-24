//! Steering calculations for vehicle physics

use glam::{Vec3, Quat};

use super::components::Vehicle;
use crate::physics::world::PhysicsWorld;
use crate::ecs::entity::Transform;

/// Update steering angles based on input
pub fn update_steering(
    vehicle: &mut Vehicle,
    transform: &Transform,
    dt: f32,
) {
    let max_steering = vehicle.config.max_steering_angle;
    let steering_speed = vehicle.config.steering_speed;
    
    // Calculate target steering angle from input
    let target_steering = vehicle.steering_input * max_steering;
    
    // Smoothly interpolate current steering towards target
    for wheel in vehicle.wheels.iter_mut() {
        if !wheel.is_steering {
            continue;
        }
        
        // Current steering is stored in the lateral slip temporarily
        // In a full implementation, we'd have a dedicated steering_angle field
        let current_steering = wheel.lateral_slip;
        
        // Interpolate towards target at steering speed
        let steering_delta = (target_steering - current_steering).clamp(
            -steering_speed * dt,
            steering_speed * dt,
        );
        
        wheel.lateral_slip = current_steering + steering_delta;
    }
}
