//! Stateful input tracking system

use std::collections::HashSet;
use winit::event::{VirtualKeyCode, MouseButton, KeyEvent, Key};
use winit::keyboard::NamedKey;

use crate::ecs::system::System;
use crate::ecs::world::World;

/// Input state tracking structure
#[derive(Debug)]
pub struct InputState {
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub just_pressed_keys: HashSet<VirtualKeyCode>,
    pub just_released_keys: HashSet<VirtualKeyCode>,
    pub pressed_mouse: HashSet<MouseButton>,
    pub mouse_position: (f32, f32),
    pub mouse_delta: (f32, f32),
}

impl InputState {
    /// Creates a new empty input state
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            just_released_keys: HashSet::new(),
            pressed_mouse: HashSet::new(),
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
        }
    }

    /// Checks if a key is currently pressed
    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Checks if a key was just pressed this frame
    pub fn is_key_just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    /// Checks if a key was just released this frame
    pub fn is_key_just_released(&self, key: VirtualKeyCode) -> bool {
        self.just_released_keys.contains(&key)
    }

    /// Clears frame-specific state (called at the start of each frame)
    pub fn clear_frame_state(&mut self) {
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
        self.mouse_delta = (0.0, 0.0);
    }

    /// Processes a key event
    pub fn process_key_event(&mut self, event: &KeyEvent) {
        let key_code = match event.logical_key.as_ref() {
            Key::Named(named) => {
                match named {
                    NamedKey::ArrowDown => Some(VirtualKeyCode::Down),
                    NamedKey::ArrowUp => Some(VirtualKeyCode::Up),
                    NamedKey::ArrowLeft => Some(VirtualKeyCode::Left),
                    NamedKey::ArrowRight => Some(VirtualKeyCode::Right),
                    NamedKey::Enter => Some(VirtualKeyCode::Return),
                    NamedKey::Space => Some(VirtualKeyCode::Space),
                    NamedKey::Escape => Some(VirtualKeyCode::Escape),
                    _ => None,
                }
            }
            Key::Character(_) => None,
            _ => None,
        };

        if let Some(key_code) = key_code {
            match event.state {
                winit::event::ElementState::Pressed => {
                    if !self.pressed_keys.contains(&key_code) {
                        self.just_pressed_keys.insert(key_code);
                    }
                    self.pressed_keys.insert(key_code);
                }
                winit::event::ElementState::Released => {
                    self.just_released_keys.insert(key_code);
                    self.pressed_keys.remove(&key_code);
                }
            }
        }
    }

    /// Processes mouse motion
    pub fn process_mouse_motion(&mut self, delta: (f32, f32), position: (f32, f32)) {
        self.mouse_delta = delta;
        self.mouse_position = position;
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// Input system that manages input state in the ECS world
pub struct InputSystem;

impl System for InputSystem {
    fn initialize(&mut self, world: &mut World) {
        world.add_resource(InputState::new());
        log::info!("InputSystem initialized");
    }

    fn run(&mut self, world: &mut World) {
        if let Some(input_state) = world.get_resource_mut::<InputState>() {
            input_state.clear_frame_state();
        }
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self
    }
}
