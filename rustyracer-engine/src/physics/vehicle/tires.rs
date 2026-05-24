//! Tire friction model using simplified Pacejka Magic Formula

use glam::Vec3;

use super::components::Vehicle;
use crate::physics::world::PhysicsWorld;
use crate::ecs::entity::Transform;

/// Calculate longitudinal wheel force based on slip behavior
/// Uses simplified Pacejka formula with peak slip threshold, stiffness, and shape parameter
pub fn longitudinal_force(slip: f32, load: f32, friction: f32) -> f32 {
    let peak_slip = 0.15;
    let stiffness = 10.0;
    let shape = 1.8;
    
    // Simplified Pacejka: F = D * sin(C * atan(B * x))
    // Where D = peak force, C = shape factor, B = stiffness factor
    let d = load * friction;
    let c = shape;
    let b = stiffness / (c * d + 0.001); // Avoid division by zero
    
    let force = d * (c * (b * slip).atan()).sin();
    
    // Clamp to friction circle
    force.clamp(-d, d)
}

/// Compute cornering friction forces over slip angles
pub fn lateral_force(slip_angle: f32, load: f32, friction: f32) -> f32 {
    let peak_slip_angle = 0.2; // radians (~11 degrees)
    let cornering_stiffness = 8.0;
    let shape = 1.7;
    
    // Simplified lateral Pacejka
    let d = load * friction;
    let c = shape;
    let b = cornering_stiffness / (c * d + 0.001);
    
    let force = d * (c * (b * slip_angle).atan()).sin();
    
    // Clamp to friction limits
    force.clamp(-d, d)
}

/// Update tire forces for all wheels in contact with the ground
pub fn update_tires(
    vehicle: &mut Vehicle,
    transform: &Transform,
    physics_world: &mut PhysicsWorld,
    dt: f32,
) {
    for (i, wheel) in vehicle.wheels.iter_mut().enumerate() {
        if !wheel.is_contact {
            continue;
        }
        
        // Get wheel world position
        let wheel_world_pos = transform.translation
            + transform.rotation * wheel.local_position;
        
        // Break down rigid body's relative linear velocity into directional vectors
        let forward = transform.rotation * Vec3::Z;
        let right = transform.rotation * Vec3::X;
        
        // Calculate wheel velocity at contact point
        let wheel_velocity = vehicle.velocity 
            + vehicle.angular_velocity.cross(wheel_world_pos - transform.translation);
        
        // Project velocity onto forward and lateral directions
        let forward_velocity = wheel_velocity.dot(forward);
        let lateral_velocity = wheel_velocity.dot(right);
        
        // Compute slip ratios
        let wheel_linear_speed = wheel.angular_velocity * wheel.radius;
        let longitudinal_slip = if forward_velocity.abs() > 0.01 {
            (forward_velocity - wheel_linear_speed) / forward_velocity.abs()
        } else {
            0.0
        };
        
        // Compute slip angle (lateral slip)
        let slip_angle = if forward_velocity.abs() > 0.01 {
            (-lateral_velocity / forward_velocity.abs()).atan()
        } else {
            0.0
        };
        
        wheel.longitudinal_slip = longitudinal_slip;
        wheel.lateral_slip = slip_angle;
        
        // Capture aggregate vertical load from suspension states
        let vertical_load = wheel.spring_force.abs() / 9.81; // Approximate mass from force
        
        // Evaluate combined lateral and longitudinal friction outputs
        let surface_friction = wheel.surface_type.friction_coeff();
        
        let long_force = longitudinal_force(longitudinal_slip, vertical_load, surface_friction);
        let lat_force = lateral_force(slip_angle, vertical_load, surface_friction);
        
        // Apply friction circle limitation
        let max_force = vertical_load * surface_friction;
        let combined_magnitude = (long_force * long_force + lat_force * lat_force).sqrt();
        
        let (scaled_long_force, scaled_lat_force) = if combined_magnitude > max_force && combined_magnitude > 0.001 {
            let scale = max_force / combined_magnitude;
            (long_force * scale, lat_force * scale)
        } else {
            (long_force, lat_force)
        };
        
        // Convert back into unified world vectors
        let longitudinal_vector = forward * scaled_long_force;
        let lateral_vector = right * scaled_lat_force;
        let total_force = longitudinal_vector + lateral_vector;
        
        // Register force back to the Rapier chassis object
        if let Some(contact_point) = wheel.contact_point {
            physics_world.apply_force_at_point(
                vehicle.rigid_body_handle,
                total_force,
                contact_point,
                true,
            );
        }
        
        // Update wheel angular velocity based on longitudinal force
        let wheel_inertia = 0.5 * vehicle.config.wheel_mass * wheel.radius * wheel.radius;
        let torque = scaled_long_force * wheel.radius;
        wheel.angular_velocity += (torque / (wheel_inertia + 0.001)) * dt;
        
        // Apply some damping to wheel rotation
        wheel.angular_velocity *= 0.98;
    }
}
