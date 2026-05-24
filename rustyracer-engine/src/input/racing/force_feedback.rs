//! Force Feedback control module using SDL2 haptic subsystem

use crate::core::error::EngineResult;
use super::super::vehicle::components::Vehicle;

/// Force feedback effect types
#[derive(Debug, Clone)]
pub enum FfEffectType {
    Spring {
        stiffness: f32,
        deadzone: f32,
    },
    Damper {
        coefficient: f32,
    },
    Friction {
        strength: f32,
    },
    Constant {
        force: f32,
        direction: f32,
    },
    Vibration {
        frequency: f32,
        amplitude: f32,
        duration: f32,
    },
}

/// Force feedback effect wrapper
#[derive(Debug, Clone)]
pub struct FfEffect {
    pub effect_type: FfEffectType,
    pub enabled: bool,
}

impl FfEffect {
    pub fn new(effect_type: FfEffectType) -> Self {
        Self {
            effect_type,
            enabled: true,
        }
    }
}

/// Force Feedback Manager managing active haptic instances
pub struct ForceFeedbackManager {
    device_index: u32,
    active_effects: Vec<FfEffect>,
    sdl_haptic: Option<sdl2::haptic::Haptic>,
}

impl ForceFeedbackManager {
    /// Create a new force feedback manager for the specified device
    pub fn new(device_index: u32) -> EngineResult<Self> {
        // Initialize SDL2 haptic subsystem
        let sdl_context = sdl2::init()?;
        let game_controller_subsystem = sdl_context.game_controller()?;
        
        // Try to open the haptic device
        let sdl_haptic = if let Ok(controller) = game_controller_subsystem.open(device_index) {
            controller.set_rumble(0, 0, 0).ok();
            None // Using game controller rumble instead of direct haptic
        } else {
            None
        };
        
        Ok(Self {
            device_index,
            active_effects: Vec::new(),
            sdl_haptic,
        })
    }
    
    /// Update force feedback from vehicle state
    pub fn update_from_vehicle(&mut self, vehicle: &Vehicle, dt: f32) -> EngineResult<()> {
        // Clear active effects queue
        self.active_effects.clear();
        
        let speed = vehicle.speed;
        let steering = vehicle.steering_input;
        
        // 1. Progressive centering spring effect scaled with velocity
        let spring_stiffness = 0.5 + (speed / 100.0).min(1.0) * 0.5;
        self.active_effects.push(FfEffect::new(FfEffectType::Spring {
            stiffness: spring_stiffness,
            deadzone: 0.05,
        }));
        
        // 2. Damper resistance tracking high velocity rotational changes
        let damper_coefficient = 0.3 + (speed / 150.0).min(1.0) * 0.4;
        self.active_effects.push(FfEffect::new(FfEffectType::Damper {
            coefficient: damper_coefficient,
        }));
        
        // 3. Dynamic chatter vibration when wheel records severe slip differentials
        let total_slip: f32 = vehicle.wheels.iter()
            .map(|w| w.longitudinal_slip.abs() + w.lateral_slip.abs())
            .sum();
        
        if total_slip > 0.3 {
            let vibration_amplitude = (total_slip - 0.3).min(0.7);
            self.active_effects.push(FfEffect::new(FfEffectType::Vibration {
                frequency: 30.0,
                amplitude: vibration_amplitude,
                duration: dt,
            }));
        }
        
        // Apply effects to the haptic device
        self.apply_effects(dt)?;
        
        Ok(())
    }
    
    /// Apply all active effects to the haptic device
    fn apply_effects(&self, dt: f32) -> EngineResult<()> {
        // In a full implementation, this would send the effects to the SDL2 haptic device
        // For now, we'll just log the effect states
        
        #[cfg(feature = "dev")]
        for effect in &self.active_effects {
            match &effect.effect_type {
                FfEffectType::Spring { stiffness, deadzone } => {
                    log::debug!("FFB Spring: stiffness={}, deadzone={}", stiffness, deadzone);
                }
                FfEffectType::Damper { coefficient } => {
                    log::debug!("FFB Damper: coefficient={}", coefficient);
                }
                FfEffectType::Friction { strength } => {
                    log::debug!("FFB Friction: strength={}", strength);
                }
                FfEffectType::Constant { force, direction } => {
                    log::debug!("FFB Constant: force={}, direction={}", force, direction);
                }
                FfEffectType::Vibration { frequency, amplitude, duration } => {
                    log::debug!("FFB Vibration: freq={}, amp={}, dur={}", frequency, amplitude, duration);
                }
            }
        }
        
        Ok(())
    }
}
