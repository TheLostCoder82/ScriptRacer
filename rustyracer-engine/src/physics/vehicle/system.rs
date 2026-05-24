//! Vehicle Physics System - Main orchestration system

use crate::ecs::system::System;
use crate::ecs::world::World;
use crate::ecs::entity::Transform;
use crate::core::time::Time;

use super::components::Vehicle;
use super::suspension::update_suspension;
use super::tires::update_tires;
use super::drivetrain::{update_engine, update_transmission, update_drivetrain_forces};
use super::aerodynamics::update_aerodynamics;
use super::utils::update_steering;
use super::data::PhysicsConstants;
use crate::physics::world::PhysicsWorld;

/// Main vehicle physics system implementing the engine's System trait
pub struct VehiclePhysicsSystem {
    physics_constants: PhysicsConstants,
}

impl VehiclePhysicsSystem {
    pub fn new() -> Self {
        Self {
            physics_constants: PhysicsConstants::default(),
        }
    }
}

impl System for VehiclePhysicsSystem {
    fn initialize(&mut self, world: &mut World) {}

    fn run(&mut self, world: &mut World, time: &Time) {
        let dt = self.physics_constants.timestep;
        
        // Get reference to physics world
        let mut physics_world = world.get_resource_mut::<PhysicsWorld>()
            .expect("PhysicsWorld resource not found");
        
        // Query all vehicles with their transforms
        let entities: Vec<_> = world.query::<(&mut Vehicle, &Transform)>().collect();
        
        for (vehicle, transform) in entities {
            // Execute physics calculator steps in sequence
            
            // 1. Update suspension forces
            update_suspension(vehicle, transform, &mut physics_world, dt);
            
            // 2. Update engine state
            update_engine(vehicle, dt);
            
            // 3. Update transmission
            update_transmission(vehicle, dt);
            
            // 4. Update drivetrain forces
            update_drivetrain_forces(vehicle, dt);
            
            // 5. Update tire forces
            update_tires(vehicle, transform, &mut physics_world, dt);
            
            // 6. Update aerodynamics
            update_aerodynamics(
                vehicle,
                transform,
                &mut physics_world,
                dt,
                self.physics_constants.air_density,
            );
            
            // 7. Update steering
            update_steering(vehicle, transform, dt);
        }
        
        // Post-execution: Update internal states from Rapier
        // This would sync velocity and angular velocity from the rigid body
        // back to the Vehicle component for the next frame
    }
}

impl Default for VehiclePhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}
