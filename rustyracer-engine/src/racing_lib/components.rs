//! Core gameplay component definitions for racing events

use crate::ecs::component::Component;
use glam::Vec3;
use std::time::Duration;

/// Race state machine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaceState {
    Starting,
    Racing,
    Finished,
    Disqualified,
}

/// RaceInfo component tracking race metadata
#[derive(Debug, Clone, Component)]
pub struct RaceInfo {
    pub track_name: String,
    pub total_laps: u32,
    pub current_lap: u32,
    pub state: RaceState,
    pub elapsed_time: Duration,
    pub countdown: f32, // seconds until race starts
}

impl RaceInfo {
    pub fn new(track_name: &str, total_laps: u32) -> Self {
        Self {
            track_name: track_name.to_string(),
            total_laps,
            current_lap: 0,
            state: RaceState::Starting,
            elapsed_time: Duration::ZERO,
            countdown: 3.0,
        }
    }
}

/// LapTimer component tracking granular lap metrics
#[derive(Debug, Clone, Component)]
pub struct LapTimer {
    pub current_lap_time: Duration,
    pub best_lap_time: Option<Duration>,
    pub last_lap_time: Option<Duration>,
    pub lap_history: Vec<Duration>,
    pub sector_times: [Option<Duration>; 3],
    pub best_sector_times: [Option<Duration>; 3],
    pub checkpoints_passed: u32,
}

impl Default for LapTimer {
    fn default() -> Self {
        Self {
            current_lap_time: Duration::ZERO,
            best_lap_time: None,
            last_lap_time: None,
            lap_history: Vec::new(),
            sector_times: [None, None, None],
            best_sector_times: [None, None, None],
            checkpoints_passed: 0,
        }
    }
}

/// Checkpoint component for track markers
#[derive(Debug, Clone, Component)]
pub struct Checkpoint {
    pub index: u32,
    pub position: Vec3,
    pub radius: f32,
    pub is_start_finish: bool,
    pub normal: Vec3, // Direction the checkpoint faces
}

impl Checkpoint {
    pub fn new(index: u32, position: Vec3, radius: f32, is_start_finish: bool) -> Self {
        Self {
            index,
            position,
            radius,
            is_start_finish,
            normal: Vec3::Z,
        }
    }
}

/// AiDriver component for AI-controlled vehicles
#[derive(Debug, Clone, Component)]
pub struct AiDriver {
    pub skill_level: f32,      // 0.0 to 1.0
    pub current_target: Option<Vec3>,
    pub aggressiveness: f32,   // 0.0 to 1.0
    pub vehicle_proxy: Option<u64>, // Entity ID of associated vehicle
}

impl AiDriver {
    pub fn new(skill_level: f32, aggressiveness: f32) -> Self {
        Self {
            skill_level: skill_level.clamp(0.0, 1.0),
            current_target: None,
            aggressiveness: aggressiveness.clamp(0.0, 1.0),
            vehicle_proxy: None,
        }
    }
}

/// Camera mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    Cockpit,
    Chase,
    Orbit,
    RearView,
}

/// RacingCamera component monitoring camera view states
#[derive(Debug, Clone, Component)]
pub struct RacingCamera {
    pub mode: CameraMode,
    pub target_entity: Option<u64>, // Entity ID being tracked
    pub damping: f32,               // Smoothing factor (0.0 to 1.0)
    pub orbit_angle: f32,           // For orbit mode
    pub chase_distance: f32,        // For chase mode
    pub chase_height: f32,          // For chase mode
}

impl RacingCamera {
    pub fn new(mode: CameraMode, target_entity: u64) -> Self {
        Self {
            mode,
            target_entity: Some(target_entity),
            damping: 0.1,
            orbit_angle: 0.0,
            chase_distance: 5.0,
            chase_height: 2.0,
        }
    }
    
    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::Cockpit => CameraMode::Chase,
            CameraMode::Chase => CameraMode::Orbit,
            CameraMode::Orbit => CameraMode::RearView,
            CameraMode::RearView => CameraMode::Cockpit,
        };
    }
}
