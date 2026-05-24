//! Raycast suspension simulation for vehicle physics

use glam::Vec3;
use rapier3d::geometry::{ColliderHandle, QueryFilter};

use super::components::Vehicle;
use crate::physics::world::PhysicsWorld;
use crate::ecs::entity::Transform;

/// Update suspension forces using raycast queries
pub fn update_suspension(
    vehicle: &mut Vehicle,
    world_transform: &Transform,
    physics_world: &mut PhysicsWorld,
    dt: f32,
) {
    let gravity = 9.81;
    
    for (i, wheel) in vehicle.wheels.iter_mut().enumerate() {
        // Calculate world space starting coordinate for the wheel
        let wheel_world_pos = world_transform.translation
            + world_transform.rotation * wheel.local_position;
        
        // Formulate downward raycast spanning wheel radius + max suspension travel
        let ray_origin = wheel_world_pos;
        let ray_direction = Vec3::new(0.0, -1.0, 0.0);
        let ray_length = wheel.radius + vehicle.config.suspension_travel;
        
        // Perform raycast through Rapier3D query pipeline
        let filter = QueryFilter::new();
        let hit = physics_world.cast_ray(
            ray_origin,
            ray_direction,
            ray_length,
            true,
            filter,
        );
        
        if hit.is_none() {
            // No intersection - reset suspension metrics
            wheel.suspension_travel = 0.0;
            wheel.spring_force = 0.0;
            wheel.damper_force = 0.0;
            wheel.is_contact = false;
            wheel.contact_point = None;
            wheel.contact_normal = None;
            continue;
        }
        
        let hit_data = hit.unwrap();
        
        // Map contact depths and derive compression travel values
        let penetration = hit_data.toi;
        let compression = (ray_length - penetration).max(0.0);
        wheel.suspension_travel = compression;
        wheel.is_contact = true;
        wheel.contact_point = Some(ray_origin + ray_direction * penetration);
        wheel.contact_normal = Some(Vec3::new(0.0, 1.0, 0.0));
        wheel.penetration_depth = compression;
        
        // Calculate Spring Force using Hooke's Law (F = -k * x)
        let spring_force = -vehicle.config.spring_rate * compression;
        wheel.spring_force = spring_force;
        
        // Determine localized dampening velocity along vertical axis
        // Implement independent profiles for compression vs rebound dynamics (F = -c * v)
        let current_velocity = vehicle.velocity.y;
        let damping_coefficient = if current_velocity < 0.0 {
            // Compression
            vehicle.config.damping_compression
        } else {
            // Rebound
            vehicle.config.damping_rebound
        };
        
        let damper_force = -damping_coefficient * current_velocity;
        wheel.damper_force = damper_force;
        
        // Calculate total combined vertical force
        let total_force = spring_force + damper_force;
        
        // Apply force at wheel position to the rigid body
        let force_vector = Vec3::new(0.0, total_force, 0.0);
        if let Some(contact_point) = wheel.contact_point {
            physics_world.apply_force_at_point(
                vehicle.rigid_body_handle,
                force_vector,
                contact_point,
                true,
            );
        }
    }
}
