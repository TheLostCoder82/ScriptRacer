//! Systems architecture for ECS execution

use crate::ecs::world::World;

/// Trait for systems that can be executed by the scheduler
pub trait System: Send + Sync {
    /// Runs the system logic
    fn run(&mut self, world: &mut World);

    /// Optional initialization hook
    fn initialize(&mut self, _world: &mut World) {}

    /// Optional cleanup hook
    fn cleanup(&mut self, _world: &mut World) {}
}

/// A system wrapper for function closures
pub struct FunctionSystem<F> {
    func: F,
}

impl<F> FunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    /// Creates a new function system
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> System for FunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
}

/// Helper function to convert a closure into a system
pub fn system<F>(func: F) -> FunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    FunctionSystem::new(func)
}
