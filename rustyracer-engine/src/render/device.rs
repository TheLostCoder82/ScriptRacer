//! Render device wrapper for wgpu initialization

use crate::core::{EngineResult, EngineError};
use log::{info, debug};
use wgpu::{Instance, Adapter, Device, Queue, ShaderModule, ShaderSource, Backends};

/// Wrapper around core wgpu objects
pub struct RenderDevice {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl RenderDevice {
    /// Creates a new RenderDevice asynchronously
    pub async fn new() -> EngineResult<Self> {
        info!("Initializing wgpu instance...");
        
        let instance = Instance::new(Backends::VULKAN | Backends::METAL | Backends::DX12);
        
        debug!("Requesting high-performance adapter...");
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| EngineError::InitializationFailed("Failed to request adapter".to_string()))?;
        
        debug!("Requesting device and queue...");
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("RenderDevice"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await?;
        
        info!("RenderDevice initialized successfully");
        
        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
    
    /// Creates a shader module from WGSL source
    pub fn create_shader_module(&self, source: &str, label: &str) -> ShaderModule {
        self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(source.into()),
        })
    }
}
