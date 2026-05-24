//! Render ECS system

use crate::prelude::*;
use crate::ecs::{World, System};
use crate::assets::manager::AssetManager;
use super::resources::{Camera, MeshRenderer, DirectionalLight, PointLight, Mesh, Material};

/// Renderer wrapper (placeholder for full renderer implementation)
pub struct Renderer {
    // Full renderer implementation would go here in later phases
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn render(
        &mut self,
        camera: &Camera,
        camera_transform: &Transform,
        renderables: &[(&Transform, &MeshRenderer)],
        dir_lights: &[DirectionalLight],
        point_lights: &[PointLight],
    ) -> Result<(), crate::core::EngineError> {
        // Placeholder render implementation
        log::debug!("Rendering frame with {} renderables", renderables.len());
        Ok(())
    }
}

/// Render system that executes the rendering pipeline
pub struct RenderSystem {
    renderer: Renderer,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
        }
    }
}

impl System for RenderSystem {
    fn run(&mut self, world: &mut World) {
        // Extract AssetManager resource
        let asset_manager = world.get_resource::<AssetManager>();
        if asset_manager.is_none() {
            log::warn!("AssetManager not found, skipping render");
            return;
        }
        let asset_manager = asset_manager.unwrap();
        
        // Find active camera and its transform
        let camera_query = world.query::<(&Camera, &Transform)>();
        let mut camera_entity = None;
        for (entity, (camera, transform)) in camera_query.iter(world) {
            if camera.active {
                camera_entity = Some((camera, transform));
                break;
            }
        }
        
        let (camera, cam_transform) = match camera_entity {
            Some(c) => c,
            None => {
                log::warn!("No active camera found, skipping render");
                return;
            }
        };
        
        // Collect all visible renderable objects
        let renderable_query = world.query::<(&Transform, &MeshRenderer)>();
        let mut renderables = Vec::new();
        for (entity, (transform, mesh_renderer)) in renderable_query.iter(world) {
            if mesh_renderer.visible {
                renderables.push((transform, mesh_renderer));
            }
        }
        
        // Query all lights
        let dir_light_query = world.query::<&DirectionalLight>();
        let dir_lights: Vec<DirectionalLight> = dir_light_query
            .iter(world)
            .map(|(_, light)| *light)
            .collect();
        
        let point_light_query = world.query::<&PointLight>();
        let point_lights: Vec<PointLight> = point_light_query
            .iter(world)
            .map(|(_, light)| *light)
            .collect();
        
        // Execute render call
        if let Err(e) = self.renderer.render(camera, cam_transform, &renderables, &dir_lights, &point_lights) {
            log::error!("Render error: {}", e);
        }
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self::new()
    }
}
