//! Multi-mode racing camera controller system

use crate::ecs::system::System;
use crate::ecs::world::World;
use crate::ecs::entity::Transform;
use crate::core::time::Time;
use glam::{Vec3, Quat, Mat4};

use super::components::{RacingCamera, CameraMode};

/// Racing camera system implementing multi-view perspectives
pub struct RacingCameraSystem {
    orbit_speed: f32,
}

impl RacingCameraSystem {
    pub fn new() -> Self {
        Self {
            orbit_speed: 0.5, // radians per second
        }
    }
    
    /// Update camera based on Cockpit mode
    fn update_cockpit(&self, camera: &mut RacingCamera, vehicle_transform: &Transform, camera_transform: &mut Transform) {
        // Snap camera inline directly inside cabin bounding zones
        // Fix rotations squarely along the car frame forward orientations
        
        // Position camera at driver's eye position (slightly above and forward of vehicle origin)
        let eye_offset = Vec3::new(0.0, 0.8, 0.5);
        camera_transform.translation = vehicle_transform.translation 
            + vehicle_transform.rotation * eye_offset;
        
        // Match vehicle rotation for the view direction
        camera_transform.rotation = vehicle_transform.rotation;
    }
    
    /// Update camera based on Chase mode
    fn update_chase(&self, camera: &mut RacingCamera, vehicle_transform: &Transform, camera_transform: &mut Transform, speed: f32) {
        // Implement smooth linear target vector point lerping tracking behind vehicle
        
        // Calculate chase offset based on configuration
        let chase_offset = Vec3::new(0.0, camera.chase_height, -camera.chase_distance);
        let target_position = vehicle_transform.translation 
            + vehicle_transform.rotation * chase_offset;
        
        // Smoothly interpolate camera position towards target
        camera_transform.translation = camera_transform.translation.lerp(
            target_position,
            camera.damping,
        );
        
        // Look at the vehicle from behind
        let look_at_target = vehicle_transform.translation + Vec3::new(0.0, 0.5, 0.0);
        let direction = (look_at_target - camera_transform.translation).normalize();
        
        // Create rotation to look at target
        if direction.length() > 0.001 {
            let forward = direction;
            let right = Vec3::Y.cross(forward).normalize();
            let up = forward.cross(right).normalize();
            camera_transform.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
        }
        
        // Implement dynamic FOV modification that widens as velocity increases
        // (This would be handled by the render system reading a FOV value from the camera)
    }
    
    /// Update camera based on Orbit mode
    fn update_orbit(&self, camera: &mut RacingCamera, vehicle_transform: &Transform, camera_transform: &mut Transform, dt: f32, time_elapsed: f32) {
        // Apply smooth angular geometric rotational loops wrapping around parent vectors
        
        // Update orbit angle based on elapsed time
        camera.orbit_angle += self.orbit_speed * dt;
        
        // Calculate orbit radius
        let orbit_radius = 8.0;
        let orbit_height = 3.0;
        
        // Calculate camera position in orbit
        let x = camera.orbit_angle.cos() * orbit_radius;
        let z = camera.orbit_angle.sin() * orbit_radius;
        
        let orbit_position = Vec3::new(x, orbit_height, z);
        camera_transform.translation = vehicle_transform.translation + orbit_position;
        
        // Always look at the vehicle
        let look_at_target = vehicle_transform.translation + Vec3::new(0.0, 0.5, 0.0);
        let direction = (look_at_target - camera_transform.translation).normalize();
        
        if direction.length() > 0.001 {
            let forward = direction;
            let right = Vec3::Y.cross(forward).normalize();
            let up = forward.cross(right).normalize();
            camera_transform.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
        }
    }
    
    /// Update camera based on RearView mode
    fn update_rearview(&self, camera: &mut RacingCamera, vehicle_transform: &Transform, camera_transform: &mut Transform) {
        // Snap camera view plane pointing backwards out from the rear of the vehicle
        
        // Position camera looking out the back window
        let rear_offset = Vec3::new(0.0, 0.7, -0.3);
        camera_transform.translation = vehicle_transform.translation 
            + vehicle_transform.rotation * rear_offset;
        
        // Rotate to face backwards (180 degrees from forward)
        let rear_rotation = vehicle_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);
        camera_transform.rotation = rear_rotation;
    }
}

use glam::Mat3;

impl System for RacingCameraSystem {
    fn initialize(&mut self, world: &mut World) {}

    fn run(&mut self, world: &mut World, time: &Time) {
        let dt = time.delta_seconds();
        let time_elapsed = time.elapsed_seconds();
        
        // Query for racing cameras and their associated transforms
        // In a full implementation, we'd also need to find the vehicle transform being tracked
        
        let cameras: Vec<_> = world.query::<(&mut RacingCamera, &mut Transform)>().collect();
        
        for (camera, camera_transform) in cameras {
            // Find the vehicle entity being tracked
            if let Some(target_entity) = camera.target_entity {
                // Try to get the vehicle's transform
                if let Some(vehicle_transform) = world.get_component::<Transform>(target_entity) {
                    // Get vehicle speed for chase mode FOV effects
                    let speed = 0.0; // Would get from Vehicle component
                    
                    match camera.mode {
                        CameraMode::Cockpit => {
                            self.update_cockpit(camera, &vehicle_transform, camera_transform);
                        }
                        CameraMode::Chase => {
                            self.update_chase(camera, &vehicle_transform, camera_transform, speed);
                        }
                        CameraMode::Orbit => {
                            self.update_orbit(camera, &vehicle_transform, camera_transform, dt, time_elapsed);
                        }
                        CameraMode::RearView => {
                            self.update_rearview(camera, &vehicle_transform, camera_transform);
                        }
                    }
                }
            }
        }
    }
}

impl Default for RacingCameraSystem {
    fn default() -> Self {
        Self::new()
    }
}
