//! Deterministic Rollback Orchestrator

use std::collections::VecDeque;
use crate::ecs::{system::System, world::World};
use crate::physics::world::PhysicsWorld;
use crate::physics::vehicle::components::Vehicle;
use crate::core::error::EngineResult;
use glam::{Vec3, Quat};

/// Maximum rollback history frames
const MAX_HISTORY: usize = 8;

/// Saved state for rollback
#[derive(Clone)]
pub struct SavedState {
    pub frame: u32,
    pub vehicles: Vec<VehicleSnapshot>,
    pub physics_gravity: Vec3,
    pub input_frame: u32,
}

/// Snapshot of vehicle state
#[derive(Clone)]
pub struct VehicleSnapshot {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub rpm: f32,
    pub gear: i32,
    pub wheel_states: Vec<WheelSnapshot>,
}

/// Snapshot of wheel state
#[derive(Clone)]
pub struct WheelSnapshot {
    pub angular_velocity: f32,
    pub suspension_travel: f32,
    pub is_contact: bool,
}

impl VehicleSnapshot {
    fn from_vehicle(vehicle: &Vehicle, position: Vec3, rotation: Quat, velocity: Vec3, angular_velocity: Vec3) -> Self {
        let wheel_states = vehicle.wheels.iter().map(|w| WheelSnapshot {
            angular_velocity: w.angular_velocity,
            suspension_travel: w.suspension_travel,
            is_contact: w.is_contact,
        }).collect();
        
        Self {
            position,
            rotation,
            velocity,
            angular_velocity,
            rpm: vehicle.rpm,
            gear: vehicle.current_gear,
            wheel_states,
        }
    }
    
    fn apply_to_vehicle(&self, vehicle: &mut Vehicle) {
        vehicle.rpm = self.rpm;
        vehicle.current_gear = self.gear;
        
        for (i, wheel_snap) in self.wheel_states.iter().enumerate() {
            if let Some(wheel) = vehicle.wheels.get_mut(i) {
                wheel.angular_velocity = wheel_snap.angular_velocity;
                wheel.suspension_travel = wheel_snap.suspension_travel;
                wheel.is_contact = wheel_snap.is_contact;
            }
        }
    }
}

/// Rollback system for deterministic netcode
pub struct RollbackSystem {
    history: VecDeque<SavedState>,
    current_frame: u32,
    max_history: usize,
}

impl Default for RollbackSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RollbackSystem {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(MAX_HISTORY),
            current_frame: 0,
            max_history: MAX_HISTORY,
        }
    }
    
    /// Save current state to history
    pub fn save_state(&mut self, world: &World) {
        // Get physics gravity
        let physics_gravity = world.get_resource::<PhysicsWorld>()
            .map(|pw| pw.gravity)
            .unwrap_or(Vec3::ZERO);
        
        // Snapshot all vehicles
        let mut vehicles = Vec::new();
        for (entity, vehicle) in world.iter_components::<Vehicle>() {
            if let Some(transform) = world.get_component::<crate::prelude::Transform>(entity) {
                let snapshot = VehicleSnapshot::from_vehicle(
                    vehicle,
                    transform.position,
                    transform.rotation,
                    vehicle.velocity,
                    vehicle.angular_velocity,
                );
                vehicles.push(snapshot);
            }
        }
        
        let state = SavedState {
            frame: self.current_frame,
            vehicles,
            physics_gravity,
            input_frame: self.current_frame,
        };
        
        self.history.push_back(state);
        
        // Trim history if over limit
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
        
        self.current_frame += 1;
    }
    
    /// Rollback to a target frame
    pub fn rollback(&mut self, target_frame: u32, world: &mut World) -> EngineResult<()> {
        // Find the saved state for target frame
        let target_state = self.history.iter()
            .find(|s| s.frame == target_frame)
            .ok_or_else(|| crate::core::error::EngineError::Network(
                format!("Target frame {} not found in history", target_frame)
            ))?;
        
        // Restore physics gravity
        if let Some(mut physics_world) = world.get_resource_mut::<PhysicsWorld>() {
            physics_world.gravity = target_state.physics_gravity;
        }
        
        // Restore vehicle states
        let entities: Vec<_> = world.iter_components::<Vehicle>().map(|(e, _)| e).collect();
        for (i, entity) in entities.iter().enumerate() {
            if let (Some(mut vehicle), Some(mut transform)) = (
                world.get_component_mut::<Vehicle>(*entity),
                world.get_component_mut::<crate::prelude::Transform>(*entity),
            ) {
                if let Some(snapshot) = target_state.vehicles.get(i) {
                    transform.position = snapshot.position;
                    transform.rotation = snapshot.rotation;
                    vehicle.velocity = snapshot.velocity;
                    vehicle.angular_velocity = snapshot.angular_velocity;
                    snapshot.apply_to_vehicle(&mut vehicle);
                }
            }
        }
        
        Ok(())
    }
    
    /// Advance frame counter
    pub fn advance_frame(&mut self, world: &mut World) {
        self.current_frame += 1;
    }
}

impl System for RollbackSystem {
    fn run(&mut self, world: &mut World) {
        // Check if rollback is needed based on network state
        // This would integrate with NetworkState resource
        // For now, just save state each frame
        self.save_state(world);
    }
}
