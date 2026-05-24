//! Race manager system for timing, tracking, and position management

use crate::ecs::system::System;
use crate::ecs::world::World;
use crate::core::time::Time;
use std::time::Duration;

use super::components::{RaceInfo, RaceState, LapTimer, Checkpoint};

/// Position tracking entry
#[derive(Debug, Clone)]
pub struct CompetitorPosition {
    pub entity_id: u64,
    pub laps_completed: u32,
    pub distance_on_current_lap: f32,
    pub total_progress_score: f32,
}

/// Race Manager System implementing the base System trait
pub struct RaceManagerSystem {
    positions: Vec<CompetitorPosition>,
    next_checkpoint_index: u32,
}

impl RaceManagerSystem {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            next_checkpoint_index: 0,
        }
    }
    
    /// Sort competitors by calculated progression score
    fn update_positions(&mut self, world: &World) {
        self.positions.clear();
        
        // Query all competitors with LapTimer and position data
        let entities: Vec<_> = world.query::<(u64, &LapTimer)>().collect();
        
        for (entity_id, lap_timer) in entities {
            let laps_completed = lap_timer.lap_history.len() as u32;
            
            // Calculate distance along current lap segment based on checkpoints passed
            // This is a simplified metric - a full implementation would use actual track distance
            let distance_on_current_lap = lap_timer.checkpoints_passed as f32 * 0.1;
            
            // Sort using calculated progression scoring standard:
            // laps completed + distance along current lap segment
            let total_progress_score = laps_completed as f32 + distance_on_current_lap;
            
            self.positions.push(CompetitorPosition {
                entity_id,
                laps_completed,
                distance_on_current_lap,
                total_progress_score,
            });
        }
        
        // Sort by progress score descending
        self.positions.sort_by(|a, b| {
            b.total_progress_score.partial_cmp(&a.total_progress_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

impl System for RaceManagerSystem {
    fn initialize(&mut self, world: &mut World) {}

    fn run(&mut self, world: &mut World, time: &Time) {
        let dt = time.delta_seconds();
        
        // Process timing sequences across active RaceInfo entities
        let race_infos: Vec<_> = world.query::<&mut RaceInfo>().collect();
        
        for race_info in race_infos {
            match race_info.state {
                RaceState::Starting => {
                    // Manage countdown period
                    race_info.countdown -= dt;
                    
                    if race_info.countdown <= 0.0 {
                        race_info.state = RaceState::Racing;
                        race_info.countdown = 0.0;
                        log::info!("Race started!");
                    }
                }
                RaceState::Racing => {
                    // Update elapsed time
                    race_info.elapsed_time += Duration::from_secs_f32(dt);
                    
                    // Check if any competitor has finished
                    // (In a full implementation, this would query lap completion)
                }
                RaceState::Finished | RaceState::Disqualified => {
                    // Race is over, no updates needed
                }
            }
        }
        
        // Update lap timers
        let mut lap_timers: Vec<_> = world.query::<&mut LapTimer>().collect();
        
        for lap_timer in lap_timers {
            if race_infos.iter().any(|r| r.state == RaceState::Racing) {
                lap_timer.current_lap_time += Duration::from_secs_f32(dt);
            }
        }
        
        // Update positions
        self.update_positions(world);
    }
}

impl Default for RaceManagerSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Checkpoint system for tracking checkpoint passage
pub struct CheckpointSystem;

impl CheckpointSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for CheckpointSystem {
    fn initialize(&mut self, world: &mut World) {}

    fn run(&mut self, world: &mut World, time: &Time) {
        // Query for vehicles and their positions relative to checkpoints
        // This is a placeholder for the full checkpoint detection logic
        
        #[cfg(feature = "dev")]
        log::debug!("Checkpoint system running");
    }
}

impl Default for CheckpointSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Lap timer system
pub struct LapTimerSystem;

impl LapTimerSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for LapTimerSystem {
    fn initialize(&mut self, world: &mut World) {}

    fn run(&mut self, world: &mut World, time: &Time) {
        // Handle lap timing logic including sector times and lap completion
        // This is a placeholder for the full lap timer logic
        
        #[cfg(feature = "dev")]
        log::debug!("Lap timer system running");
    }
}

impl Default for LapTimerSystem {
    fn default() -> Self {
        Self::new()
    }
}
