//! Camera component and math

use crate::prelude::*;
use crate::ecs::component::Component;
use bytemuck::{Pod, Zeroable};

/// Projection type
#[derive(Clone, Copy, Debug)]
pub enum Projection {
    Perspective,
    Orthographic,
}

/// Camera component
#[derive(Clone, Debug)]
pub struct Camera {
    pub projection: Projection,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub viewport: (u32, u32),
    pub active: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            projection: Projection::Perspective,
            fov_y: 60.0_f32.to_radians(),
            near: 0.1,
            far: 1000.0,
            viewport: (1280, 720),
            active: true,
        }
    }
}

impl Component for Camera {}

impl Camera {
    /// Builds the projection matrix
    pub fn build_projection_matrix(&self) -> Mat4 {
        match self.projection {
            Projection::Perspective => {
                Mat4::perspective_rh(self.fov_y, self.viewport.0 as f32 / self.viewport.1 as f32, self.near, self.far)
            }
            Projection::Orthographic => {
                let aspect = self.viewport.0 as f32 / self.viewport.1 as f32;
                Mat4::orthographic_rh(-aspect * 5.0, aspect * 5.0, -5.0, 5.0, self.near, self.far)
            }
        }
    }
    
    /// Builds the view matrix from a transform
    pub fn build_view_matrix(&self, transform: &Transform) -> Mat4 {
        let position = transform.position;
        let forward = transform.forward();
        let target = position + forward;
        Mat4::look_at_rh(position, target, Vec3::Y)
    }
    
    /// Builds the view-projection matrix
    pub fn build_view_projection(&self, transform: &Transform) -> Mat4 {
        let proj = self.build_projection_matrix();
        let view = self.build_view_matrix(transform);
        proj * view
    }
}

/// Uniform buffer data for camera
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: Mat4,
    pub position: Vec3,
    pub _padding: f32,
}

impl CameraUniform {
    pub fn new(camera: &Camera, transform: &Transform) -> Self {
        Self {
            view_proj: camera.build_view_projection(transform),
            position: transform.position,
            _padding: 0.0,
        }
    }
}
