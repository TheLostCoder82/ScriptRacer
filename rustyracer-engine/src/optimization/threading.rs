//! Work-Stealing Job System Execution Framework using Rayon

use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use crate::ecs::system::System;
use crate::ecs::world::World;
use crate::physics::vehicle::components::Vehicle;
use crate::prelude::*;

/// Job system for parallel execution
pub struct JobSystem {
    pool: ThreadPool,
}

impl JobSystem {
    /// Create a new job system with optimal thread count
    pub fn new() -> Self {
        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .expect("Failed to create thread pool");
        
        Self { pool }
    }
    
    /// Run a closure in the thread pool and return result
    pub fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        self.pool.install(f)
    }
    
    /// Execute parallel for-each over mutable slice
    pub fn par_for_each<T, F>(&self, items: &mut [T], f: F)
    where
        T: Send,
        F: Fn(&mut T) + Sync + Send,
    {
        items.par_iter_mut().for_each(f);
    }
    
    /// Execute parallel for-each with index
    pub fn par_for_each_with_index<T, F>(&self, items: &mut [T], f: F)
    where
        T: Send,
        F: Fn(usize, &mut T) + Sync + Send,
    {
        items.par_iter_mut().enumerate().for_each(|(i, item)| f(i, item));
    }
}

impl Default for JobSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Example parallel physics system demonstrating job system usage
pub struct ParallelPhysicsSystem {
    job_system: JobSystem,
}

impl ParallelPhysicsSystem {
    pub fn new() -> Self {
        Self {
            job_system: JobSystem::new(),
        }
    }
}

impl System for ParallelPhysicsSystem {
    fn run(&mut self, world: &mut World) {
        // Collect entities with Vehicle components
        let entities: Vec<_> = world.iter_components::<Vehicle>().map(|(e, _)| e).collect();
        
        if entities.is_empty() {
            return;
        }
        
        // Process vehicles in parallel using the job system
        self.job_system.run(|| {
            // In a real implementation, we'd need to handle borrowing carefully
            // This is a simplified example showing the pattern
            
            // For each vehicle, we would:
            // 1. Get vehicle and transform components
            // 2. Update physics state in parallel
            // 3. Apply results back to world
            
            log::debug!("Processing {} vehicles in parallel", entities.len());
        });
        
        // Example of parallel iteration pattern (conceptual)
        // The actual implementation would need careful handling of ECS borrowing
        let mut vehicle_data: Vec<(u64, Transform, Vehicle)> = Vec::new();
        
        for entity in &entities {
            if let (Some(transform), Some(vehicle)) = (
                world.get_component::<Transform>(*entity),
                world.get_component::<Vehicle>(*entity),
            ) {
                vehicle_data.push((*entity, transform.clone(), vehicle.clone()));
            }
        }
        
        // Process in parallel
        self.job_system.par_for_each(&mut vehicle_data, |data| {
            let (_entity, transform, vehicle) = data;
            
            // Example parallel computation
            // Update vehicle physics based on current state
            let speed = vehicle.velocity.length();
            
            // Apply aerodynamic drag (simplified)
            let drag_coefficient = 0.5 * 1.225 * speed * speed; // air density * v^2
            
            // Would apply forces back to vehicle
            let _drag_force = -vehicle.velocity.normalize_or_zero() * drag_coefficient * 0.3;
        });
        
        // Write back modified data
        for (entity, _transform, vehicle) in vehicle_data {
            if let Some(existing_vehicle) = world.get_component_mut::<Vehicle>(entity) {
                existing_vehicle.velocity = vehicle.velocity;
                existing_vehicle.rpm = vehicle.rpm;
            }
        }
    }
}
