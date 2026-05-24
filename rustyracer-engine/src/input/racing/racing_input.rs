//! Racing input system with unified control multiplexing

use crate::ecs::system::System;
use crate::ecs::world::World;
use crate::ecs::component::Component;
use crate::core::error::EngineResult;
use crate::core::time::Time;

use super::vehicle::components::Vehicle;
use super::force_feedback::ForceFeedbackManager;

/// Racing input state component maintaining numeric input fields
#[derive(Debug, Clone, Component)]
pub struct RacingInputState {
    // Numeric floating fields (0.0 to 1.0 limits)
    pub throttle: f32,
    pub brake: f32,
    pub clutch: f32,
    
    // Directional steering states (-1.0 to 1.0 ranges)
    pub steering: f32,
    
    // Boolean flags
    pub shift_up: bool,
    pub shift_down: bool,
    pub handbrake: bool,
    pub reset: bool,
}

impl Default for RacingInputState {
    fn default() -> Self {
        Self {
            throttle: 0.0,
            brake: 0.0,
            clutch: 0.0,
            steering: 0.0,
            shift_up: false,
            shift_down: false,
            handbrake: false,
            reset: false,
        }
    }
}

/// Racing input system managing wheel controllers and keyboard fallback
pub struct RacingInputSystem {
    racing_wheel_index: Option<u32>,
    force_feedback_manager: Option<ForceFeedbackManager>,
}

impl RacingInputSystem {
    pub fn new() -> Self {
        Self {
            racing_wheel_index: None,
            force_feedback_manager: None,
        }
    }
    
    /// Try to detect and initialize a racing wheel controller
    fn detect_racing_wheel(&mut self) -> EngineResult<()> {
        // Initialize SDL2 game controller subsystem
        let sdl_context = sdl2::init()?;
        let game_controller_subsystem = sdl_context.game_controller()?;
        
        // Scan for available controllers
        for i in 0..game_controller_subsystem.num_connected_controllers() {
            if game_controller_subsystem.is_game_controller(i) {
                if let Ok(controller) = game_controller_subsystem.open(i) {
                    let name = controller.name();
                    log::info!("Detected controller: {}", name);
                    
                    // Check if this looks like a racing wheel
                    // (In a full implementation, we'd check specific capabilities)
                    self.racing_wheel_index = Some(i);
                    
                    // Initialize force feedback manager
                    match ForceFeedbackManager::new(i) {
                        Ok(ffb) => {
                            self.force_feedback_manager = Some(ffb);
                            log::info!("Force feedback initialized for {}", name);
                        }
                        Err(e) => {
                            log::warn!("Force feedback not available: {}", e);
                        }
                    }
                    
                    return Ok(());
                }
            }
        }
        
        log::info!("No racing wheel detected, will use keyboard input");
        Ok(())
    }
    
    /// Read input from racing wheel
    fn read_wheel_input(&self, input_state: &mut RacingInputState) {
        // In a full implementation, this would poll the SDL2 controller state
        // For now, we'll leave placeholder logic
        
        #[cfg(feature = "dev")]
        log::debug!("Reading wheel input");
    }
    
    /// Read input from keyboard
    fn read_keyboard_input(&self, input_state: &mut RacingInputState) {
        // Keyboard mapping assumptions:
        // - W/Up: Throttle
        // - S/Down: Brake
        // - A/Left: Steer left
        // - D/Right: Steer right
        // - Space: Handbrake
        // - R: Reset
        // - Shift: Shift up
        // - Ctrl: Shift down
        
        // This is a placeholder - actual implementation would query winit keyboard state
        input_state.throttle = 0.0;
        input_state.brake = 0.0;
        input_state.steering = 0.0;
        input_state.handbrake = false;
        input_state.reset = false;
        input_state.shift_up = false;
        input_state.shift_down = false;
    }
}

impl System for RacingInputSystem {
    fn initialize(&mut self, world: &mut World) {
        // Try to detect an active racing wheel controller
        if let Err(e) = self.detect_racing_wheel() {
            log::warn!("Failed to detect racing wheel: {}", e);
        }
    }

    fn run(&mut self, world: &mut World, time: &Time) {
        let dt = time.delta_seconds();
        
        // Query for racing input state and vehicle components
        let mut entities: Vec<_> = world.query::<(&mut RacingInputState, &mut Vehicle)>().collect();
        
        for (input_state, vehicle) in entities {
            // Route input based on whether wheel controller is registered
            if self.racing_wheel_index.is_some() {
                self.read_wheel_input(input_state);
            } else {
                self.read_keyboard_input(input_state);
            }
            
            // Map input states back to the current active Vehicle components
            vehicle.throttle_input = input_state.throttle;
            vehicle.brake_input = input_state.brake;
            vehicle.clutch_input = input_state.clutch;
            vehicle.steering_input = input_state.steering;
            vehicle.handbrake = input_state.handbrake;
            
            // Handle gear shifts
            if input_state.shift_up && vehicle.current_gear < vehicle.config.gears.len() as i32 {
                vehicle.current_gear += 1;
            }
            if input_state.shift_down && vehicle.current_gear > 0 {
                vehicle.current_gear -= 1;
            }
            
            // Drive updated delta parameters into the FFB framework instance
            if let Some(ref mut ffb) = self.force_feedback_manager {
                if let Err(e) = ffb.update_from_vehicle(vehicle, dt) {
                    log::warn!("FFB update error: {}", e);
                }
            }
        }
    }
}

impl Default for RacingInputSystem {
    fn default() -> Self {
        Self::new()
    }
}
