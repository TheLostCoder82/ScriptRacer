//! Window abstraction using winit

use winit::window::{Window as WinitWindow, WindowAttributes};
use winit::event_loop::EventLoop;

use crate::core::{EngineResult, EngineError, WindowConfig};

/// Window wrapper around winit window
pub struct Window {
    inner: WinitWindow,
    config: WindowConfig,
}

impl Window {
    /// Creates a new window from configuration
    pub fn new(config: WindowConfig, event_loop: &EventLoop<()>) -> EngineResult<Self> {
        let window_attrs = WindowAttributes::default()
            .with_title(&config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(config.width as f64, config.height as f64))
            .with_resizable(config.resizable);

        let inner = WinitWindow::new(event_loop, window_attrs)
            .map_err(EngineError::PlatformError)?;

        Ok(Self { inner, config })
    }

    /// Returns the window size
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.inner.inner_size()
    }

    /// Returns the window title
    pub fn title(&self) -> String {
        self.config.title.clone()
    }

    /// Requests to close the window
    pub fn request_close(&self) {
        self.inner.request_redraw();
    }

    /// Returns a reference to the inner winit window
    pub fn inner(&self) -> &WinitWindow {
        &self.inner
    }
}
