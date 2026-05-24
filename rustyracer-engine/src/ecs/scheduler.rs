//! Scheduler for system execution and stage management

use crate::ecs::world::World;
use crate::ecs::system::System;

/// Execution stages for the game loop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStage {
    PreUpdate,
    Input,
    Update,
    Physics,
    PostUpdate,
    Render,
}

/// Scheduler for managing and executing systems
pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    /// Creates a new empty scheduler
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the scheduler
    pub fn add_system<S: System + 'static>(&mut self, mut system: S) {
        self.systems.push(Box::new(system));
    }

    /// Initializes all systems
    pub fn initialize(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.initialize(world);
        }
    }

    /// Runs all systems in order
    pub fn run(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }

    /// Cleans up all systems
    pub fn cleanup(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.cleanup(world);
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
