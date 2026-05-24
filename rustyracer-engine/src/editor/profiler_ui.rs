//! Performance Profiler Overlay using puffin and puffin_egui

use egui::Context;

/// Performance profiler UI overlay
pub struct ProfilerUi {
    pub enabled: bool,
    pub open_windows: bool,
}

impl Default for ProfilerUi {
    fn default() -> Self {
        Self {
            enabled: false,
            open_windows: false,
        }
    }
}

impl ProfilerUi {
    /// Initialize the profiler, enabling puffin scopes
    pub fn init() {
        puffin::set_scopes_on(true);
    }
    
    /// Draw the profiler UI window
    pub fn draw(&mut self, ctx: &Context) {
        if !self.enabled {
            return;
        }
        
        egui::Window::new("Performance Profiler")
            .open(&mut self.open_windows)
            .show(ctx, |ui| {
                // Frame time display
                let frame_time = ctx.input(|i| i.predicted_dt * 1000.0);
                ui.label(format!("Frame time: {:.2} ms", frame_time));
                
                // FPS display
                let fps = ctx.input(|i| 1.0 / i.predicted_dt);
                ui.label(format!("FPS: {:.1}", fps));
                
                ui.separator();
                
                // Puffin profiler UI
                puffin_egui::profiler_ui(ui);
                
                ui.separator();
                
                // Stream graph in collapsing header
                egui::CollapsingHeader::new("Frame Timeline")
                    .default_open(true)
                    .show(ui, |ui| {
                        puffin_egui::stream_graph_ui(ui);
                    });
            });
    }
    
    /// Toggle the profiler visibility
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Example usage with puffin profiling scopes
/// 
/// ```ignore
/// use rustyracer_engine::editor::profiler_ui::ProfilerUi;
/// 
/// fn some_function() {
///     puffin::profile_scope!("some_function");
///     // ... function logic
/// }
/// ```
