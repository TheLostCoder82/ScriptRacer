//! Time management system for delta time tracking and fixed updates

use std::time::Instant;

/// Time tracking structure for engine timing
#[derive(Debug)]
pub struct Time {
    start_time: Instant,
    last_frame_time: Instant,
    delta_time: f32,
    fixed_delta_time: f32,
    accumulated_time: f32,
    frame_count: u64,
}

impl Time {
    /// Creates a new Time instance with initialized timers
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: 0.0,
            fixed_delta_time: 1.0 / 60.0,
            accumulated_time: 0.0,
            frame_count: 0,
        }
    }

    /// Updates the time tracking, called each frame
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.accumulated_time += self.delta_time;
        self.frame_count += 1;
        self.last_frame_time = now;
    }

    /// Returns the delta time since last frame
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    /// Returns the fixed delta time for physics updates
    pub fn fixed_delta_time(&self) -> f32 {
        self.fixed_delta_time
    }

    /// Returns total elapsed time since engine start
    pub fn elapsed_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    /// Checks if a fixed update should occur
    pub fn should_fixed_update(&mut self) -> bool {
        if self.accumulated_time >= self.fixed_delta_time {
            self.accumulated_time -= self.fixed_delta_time;
            true
        } else {
            false
        }
    }

    /// Returns the interpolation alpha for rendering
    pub fn interpolation_alpha(&self) -> f32 {
        self.accumulated_time / self.fixed_delta_time
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
