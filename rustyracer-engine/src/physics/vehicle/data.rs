//! Vehicle physics data models and environment structures

use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Physics constants for the simulation
#[derive(Debug, Clone, Copy)]
pub struct PhysicsConstants {
    pub gravity: f32,
    pub timestep: f32,
    pub air_density: f32,
}

impl Default for PhysicsConstants {
    fn default() -> Self {
        Self {
            gravity: 9.81,
            timestep: 1.0 / 60.0,
            air_density: 1.225, // Sea level air density in kg/m³
        }
    }
}

/// Surface types for friction calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceType {
    Tarmac,
    WetTarmac,
    Gravel,
    Grass,
    Ice,
    Sand,
    Kerb,
}

impl SurfaceType {
    pub fn friction_coeff(&self) -> f32 {
        match self {
            SurfaceType::Tarmac => 1.0,
            SurfaceType::WetTarmac => 0.7,
            SurfaceType::Gravel => 0.6,
            SurfaceType::Grass => 0.5,
            SurfaceType::Ice => 0.15,
            SurfaceType::Sand => 0.4,
            SurfaceType::Kerb => 0.9,
        }
    }

    pub fn rolling_resistance(&self) -> f32 {
        match self {
            SurfaceType::Tarmac => 0.015,
            SurfaceType::WetTarmac => 0.02,
            SurfaceType::Gravel => 0.05,
            SurfaceType::Grass => 0.08,
            SurfaceType::Ice => 0.01,
            SurfaceType::Sand => 0.1,
            SurfaceType::Kerb => 0.02,
        }
    }
}

/// Drive type configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriveType {
    FrontWheelDrive,
    RearWheelDrive,
    AllWheelDrive,
}

/// Transmission type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransmissionType {
    Manual,
    Automatic,
    Sequential,
}

/// Gear definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gear {
    pub ratio: f32,
    pub name: &'static str,
}

impl Gear {
    pub fn new(ratio: f32, name: &'static str) -> Self {
        Self { ratio, name }
    }
}

/// Vehicle configuration for chassis, engine, transmission, wheels, tires, suspension, and steering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleConfig {
    // Chassis metrics
    pub mass: f32,                    // kg
    pub inertia_tensor: glam::Vec3,   // kg·m²
    pub center_of_mass: glam::Vec3,   // m (relative to vehicle origin)
    pub track_width_front: f32,       // m
    pub track_width_rear: f32,        // m
    pub wheelbase: f32,               // m
    pub drag_coefficient: f32,        // Cd
    pub frontal_area: f32,            // m²
    
    // Engine performance
    pub max_torque: f32,              // N·m
    pub max_power: f32,               // W
    pub idle_rpm: f32,                // RPM
    pub max_rpm: f32,                 // RPM
    pub torque_curve: Vec<(f32, f32)>, // (RPM, torque multiplier)
    
    // Transmission
    pub transmission_type: TransmissionType,
    pub drive_type: DriveType,
    pub gears: Vec<Gear>,
    pub final_drive_ratio: f32,
    pub shift_time: f32,              // seconds
    
    // Wheel dimensions
    pub wheel_radius: f32,            // m
    pub wheel_width: f32,             // m
    pub wheel_mass: f32,              // kg per wheel
    
    // Tire grip data
    pub tire_stiffness: f32,          // N/rad
    pub tire_damping: f32,            // N·s/rad
    pub peak_slip_ratio: f32,         // optimal slip for max grip
    
    // Suspension mechanics
    pub spring_rate: f32,             // N/m
    pub damping_compression: f32,     // N·s/m
    pub damping_rebound: f32,         // N·s/m
    pub suspension_travel: f32,       // m
    pub anti_roll_bar_stiffness: f32, // N·m/rad
    
    // Steering constraints
    pub max_steering_angle: f32,      // radians
    pub steering_ratio: f32,          // steering wheel angle / road wheel angle
    pub steering_speed: f32,          // rad/s
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            // Chassis - sports car configuration
            mass: 1400.0,
            inertia_tensor: glam::Vec3::new(2500.0, 4000.0, 1500.0),
            center_of_mass: glam::Vec3::new(0.0, 0.3, 0.5),
            track_width_front: 1.6,
            track_width_rear: 1.65,
            wheelbase: 2.7,
            drag_coefficient: 0.32,
            frontal_area: 2.1,
            
            // Engine
            max_torque: 400.0,
            max_power: 350000.0, // ~470 hp
            idle_rpm: 800.0,
            max_rpm: 7500.0,
            torque_curve: vec![
                (0.0, 0.5),
                (2000.0, 0.8),
                (4000.0, 1.0),
                (6000.0, 0.95),
                (7500.0, 0.7),
            ],
            
            // Transmission - 6-speed sequential
            transmission_type: TransmissionType::Sequential,
            drive_type: DriveType::RearWheelDrive,
            gears: vec![
                Gear::new(3.8, "1st"),
                Gear::new(2.5, "2nd"),
                Gear::new(1.9, "3rd"),
                Gear::new(1.5, "4th"),
                Gear::new(1.2, "5th"),
                Gear::new(1.0, "6th"),
            ],
            final_drive_ratio: 3.5,
            shift_time: 0.15,
            
            // Wheels
            wheel_radius: 0.33, // ~26 inch diameter
            wheel_width: 0.245, // 245mm width
            wheel_mass: 15.0,
            
            // Tires
            tire_stiffness: 150000.0,
            tire_damping: 500.0,
            peak_slip_ratio: 0.15,
            
            // Suspension
            spring_rate: 180000.0,
            damping_compression: 15000.0,
            damping_rebound: 20000.0,
            suspension_travel: 0.15,
            anti_roll_bar_stiffness: 25000.0,
            
            // Steering
            max_steering_angle: 0.6, // ~35 degrees
            steering_ratio: 15.0,
            steering_speed: 3.0,
        }
    }
}
