//! Debug Renderer & 3D Visualization System

use crate::prelude::*;
use crate::ecs::{component::Component, system::System, world::World};
use crate::render::{Renderer, Camera};
use crate::physics::{world::PhysicsWorld, components::{Collider, ColliderShape}};
use crate::physics::vehicle::components::{Vehicle, Wheel};
use crate::racing_lib::components::Checkpoint;

/// Debug renderer for visualizing physics and game objects
pub struct DebugRenderer {
    pub enabled: bool,
    pub draw_physics: bool,
    pub draw_vehicles: bool,
    pub draw_navmesh: bool,
}

impl Default for DebugRenderer {
    fn default() -> Self {
        Self {
            enabled: true,
            draw_physics: true,
            draw_vehicles: true,
            draw_navmesh: false,
        }
    }
}

impl DebugRenderer {
    /// Draw coordinate axes at a transform position
    pub fn draw_axes(&mut self, transform: &Transform, size: f32) {
        let pos = transform.position;
        
        // X axis - Red
        let x_end = pos + Vec3::X * size;
        self.draw_line(pos, x_end, Color::RED);
        
        // Y axis - Green
        let y_end = pos + Vec3::Y * size;
        self.draw_line(pos, y_end, Color::GREEN);
        
        // Z axis - Blue
        let z_end = pos + Vec3::Z * size;
        self.draw_line(pos, z_end, Color::BLUE);
    }
    
    /// Draw colliders from the physics world
    pub fn draw_colliders(&mut self, physics_world: &PhysicsWorld) {
        for (handle, collider) in physics_world.collider_set.iter() {
            let shape = collider.shape();
            let position = collider.position();
            
            match shape.as_shape_type() {
                rapier3d::geometry::ShapeType::Ball => {
                    let ball = shape.as_ball().unwrap();
                    let center = Vec3::new(
                        position.translation.x,
                        position.translation.y,
                        position.translation.z,
                    );
                    self.draw_sphere(center, ball.radius, Color::CYAN);
                }
                rapier3d::geometry::ShapeType::Cuboid => {
                    let cuboid = shape.as_cuboid().unwrap();
                    let center = Vec3::new(
                        position.translation.x,
                        position.translation.y,
                        position.translation.z,
                    );
                    let half_extents = Vec3::new(
                        cuboid.half_extents.x,
                        cuboid.half_extents.y,
                        cuboid.half_extents.z,
                    );
                    self.draw_cuboid(center, half_extents, Color::YELLOW);
                }
                _ => {}
            }
        }
    }
    
    /// Draw vehicle debug visualization
    pub fn draw_vehicle_debug(&mut self, vehicle: &Vehicle, transform: &Transform) {
        for wheel in &vehicle.wheels {
            // Calculate wheel world position
            let wheel_pos = transform.position + transform.rotation * wheel.local_position;
            
            // Draw suspension travel line
            if wheel.is_contact {
                let contact_point = wheel.contact_point.unwrap_or(wheel_pos);
                self.draw_line(wheel_pos, contact_point, Color::ORANGE);
            }
            
            // Draw wheel circle
            self.draw_circle(
                wheel_pos,
                Vec3::Y,
                wheel.radius,
                Color::WHITE,
            );
            
            // Draw wheel contact normal if in contact
            if wheel.is_contact {
                if let Some(normal) = wheel.contact_normal {
                    let normal_end = contact_point + normal * 0.5;
                    self.draw_line(contact_point, normal_end, Color::MAGENTA);
                }
            }
        }
    }
    
    /// Render debug visuals
    pub fn render(&mut self, renderer: &mut Renderer, camera: &Camera, transform: &Transform) {
        // Implementation would pass command encoder to debug draw
        // This is a placeholder for the actual wgpu debug rendering
    }
    
    /// Internal helper to draw a line
    fn draw_line(&mut self, start: Vec3, end: Vec3, color: Color) {
        // Placeholder - would integrate with wgpu debug drawing
    }
    
    /// Internal helper to draw a sphere
    fn draw_sphere(&mut self, center: Vec3, radius: f32, color: Color) {
        // Placeholder - would draw wireframe sphere
    }
    
    /// Internal helper to draw a cuboid
    fn draw_cuboid(&mut self, center: Vec3, half_extents: Vec3, color: Color) {
        // Placeholder - would draw wireframe box
    }
    
    /// Internal helper to draw a circle
    fn draw_circle(&mut self, center: Vec3, normal: Vec3, radius: f32, color: Color) {
        // Placeholder - would draw circle outline
    }
}

impl System for DebugRenderer {
    fn run(&mut self, world: &mut World) {
        if !self.enabled {
            return;
        }
        
        // Get camera transform for view reference
        let camera_transform = world.get_resource::<Transform>().cloned().unwrap_or_default();
        
        // Draw physics colliders
        if self.draw_physics {
            if let Some(physics_world) = world.get_resource::<PhysicsWorld>() {
                self.draw_colliders(physics_world);
            }
        }
        
        // Draw vehicles
        if self.draw_vehicles {
            // Iterate through entities with Vehicle component
            let entities: Vec<_> = world.iter_components::<Vehicle>().map(|(e, _)| e).collect();
            
            for entity in entities {
                if let (Some(vehicle), Some(transform)) = (
                    world.get_component::<Vehicle>(entity),
                    world.get_component::<Transform>(entity),
                ) {
                    self.draw_vehicle_debug(vehicle, transform);
                    self.draw_axes(transform, 1.0);
                }
            }
        }
        
        // Draw checkpoints
        let checkpoint_entities: Vec<_> = world.iter_components::<Checkpoint>().map(|(e, _)| e).collect();
        for entity in checkpoint_entities {
            if let Some(checkpoint) = world.get_component::<Checkpoint>(entity) {
                let checkpoint_transform = Transform {
                    position: checkpoint.position,
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                };
                self.draw_axes(&checkpoint_transform, 0.5);
            }
        }
    }
}

/// Color utility for debug drawing
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const ORANGE: Color = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };
}
