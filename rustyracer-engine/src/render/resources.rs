//! Core render components and types

use crate::prelude::*;
use crate::assets::handle::AssetHandle;
use crate::ecs::component::Component;
use super::device::RenderDevice;
use wgpu::{Buffer, Texture, TextureView, Sampler, VertexBufferLayout, VertexAttribute, BufferInitDescriptor, BufferUsages};
use bytemuck::{Pod, Zeroable};

/// Vertex structure for mesh data
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}

impl Vertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<Vec3>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: (std::mem::size_of::<Vec3>() * 2) as u64,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

/// Mesh containing vertex and index buffers
pub struct Mesh {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(device: &RenderDevice, name: &str, vertices: &[Vertex], indices: &[u32]) -> Self {
        let vertex_buffer = device.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });
        
        let index_buffer = device.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", name)),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });
        
        Self {
            name: name.to_string(),
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
}

/// Texture resource
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: (u32, u32),
}

/// Material definition
pub struct Material {
    pub name: String,
    pub shader: String,
    pub base_color: Vec4,
    pub base_color_texture: Option<AssetHandle<Texture>>,
    pub pipeline_id: usize,
}

/// ECS component for mesh rendering
#[derive(Clone, Debug)]
pub struct MeshRenderer {
    pub mesh: AssetHandle<Mesh>,
    pub material: AssetHandle<Material>,
    pub visible: bool,
}

impl Component for MeshRenderer {}

/// ECS component for directional light
#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl Component for DirectionalLight {}

/// ECS component for point light
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub radius: f32,
}

impl Component for PointLight {}
