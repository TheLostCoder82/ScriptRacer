//! In-Game Developer Console using egui

use std::collections::VecDeque;
use egui::Context;
use crate::ecs::world::World;
use crate::physics::world::PhysicsWorld;

/// Log level for console messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// A single console message
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub text: String,
    pub level: LogLevel,
    pub timestamp: f32, // Time since engine start in seconds
}

/// In-game developer console
pub struct Console {
    pub messages: VecDeque<ConsoleMessage>,
    pub input_buffer: String,
    pub history: VecDeque<String>,
    pub visible: bool,
    pub max_messages: usize,
    pub max_history: usize,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            messages: VecDeque::with_capacity(100),
            input_buffer: String::new(),
            history: VecDeque::with_capacity(50),
            visible: false,
            max_messages: 100,
            max_history: 50,
        }
    }
}

impl Console {
    /// Log a message to the console
    pub fn log(&mut self, text: impl Into<String>, level: LogLevel) {
        let text = text.into();
        let timestamp = 0.0; // Would get from global clock resource
        
        self.messages.push_back(ConsoleMessage {
            text,
            level,
            timestamp,
        });
        
        // Trim old messages if over limit
        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }
    
    /// Process a command from the input buffer
    pub fn process_command(&mut self, world: &mut World) {
        let command = self.input_buffer.trim().to_lowercase();
        let tokens: Vec<&str> = command.split_whitespace().collect();
        
        if tokens.is_empty() {
            return;
        }
        
        match tokens[0] {
            "help" => {
                self.log("Available commands:", LogLevel::Info);
                self.log("  help - Show this help message", LogLevel::Info);
                self.log("  list_entities - Count total entities", LogLevel::Info);
                self.log("  toggle_gravity - Toggle physics gravity", LogLevel::Info);
                self.log("  clear - Clear console messages", LogLevel::Info);
            }
            "list_entities" => {
                // Count entities by iterating components
                let mut entity_count = 0;
                // This is a simplified count - would need proper entity iteration
                self.log(format!("Total entities: {}", entity_count), LogLevel::Info);
            }
            "toggle_gravity" => {
                if let Some(physics_world) = world.get_resource_mut::<PhysicsWorld>() {
                    if physics_world.gravity.y < -0.1 {
                        physics_world.gravity.y = 0.0;
                        self.log("Gravity disabled", LogLevel::Success);
                    } else {
                        physics_world.gravity.y = -9.81;
                        self.log("Gravity enabled", LogLevel::Success);
                    }
                } else {
                    self.log("Physics world not found", LogLevel::Error);
                }
            }
            "clear" => {
                self.messages.clear();
                self.log("Console cleared", LogLevel::Success);
            }
            _ => {
                self.log(format!("Unknown command: {}", tokens[0]), LogLevel::Warning);
            }
        }
        
        // Add to history if not empty
        if !self.input_buffer.trim().is_empty() {
            self.history.push_front(self.input_buffer.clone());
            if self.history.len() > self.max_history {
                self.history.pop_back();
            }
        }
        
        self.input_buffer.clear();
    }
    
    /// Draw the console UI
    pub fn draw(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }
        
        egui::Window::new("Developer Console")
            .anchor(egui::Align2::LEFT_BOTTOM, (10.0, -10.0))
            .resizable(true)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                // Scrollable message history
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for msg in &self.messages {
                            let color = match msg.level {
                                LogLevel::Info => egui::Color32::WHITE,
                                LogLevel::Warning => egui::Color32::YELLOW,
                                LogLevel::Error => egui::Color32::RED,
                                LogLevel::Success => egui::Color32::GREEN,
                            };
                            ui.label(egui::RichText::new(&msg.text).color(color));
                        }
                    });
                
                ui.separator();
                
                // Input field
                ui.horizontal(|ui| {
                    ui.label(">");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.input_buffer)
                            .hint_text("Type command...")
                    );
                    
                    // Execute on Enter
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.process_command(world);
                    }
                });
            });
    }
    
    /// Toggle console visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

// Note: The draw method needs world access for command processing
// This would typically be handled in the system run method
