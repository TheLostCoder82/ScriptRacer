//! ECS Components for Vehicles and Wheels

use crate::ecs::component::Component;
use rapier3d::dynamics::RigidBodyHandle;

use super::data::{DriveType, SurfaceType, TransmissionType, VehicleConfig};

/// Wheel component representing individual wheel state
#[derive(Debug, Clone, Component)]
pub struct Wheel {
    // Configuration
    pub local_position: glam::Vec3,
    pub radius: f32,
    pub width: f32,
    pub is_steering: bool,
    pub is_driven: bool,

    // Suspension states
    pub suspension_travel: f32,      // Current compression (m)
    pub spring_force: f32,           // Current spring force (N)
    pub damper_force: f32,           // Current damper force (N)
    pub last_position: glam::Vec3,   // Last recorded world position

    // Tire state dynamics
    pub angular_velocity: f32,       // rad/s
    pub longitudinal_slip: f32,      // Slip ratio
    pub lateral_slip: f32,           // Slip angle (rad)
    pub current_friction: f32,       // Current friction coefficient
    pub surface_type: SurfaceType,   // Current surface

    // Contact manifold data
    pub is_contact: bool,
    pub contact_point: Option<glam::Vec3>,
    pub contact_normal: Option<glam::Vec3>,
    pub penetration_depth: f32,
}

impl Wheel {
    pub fn new(local_pos: glam::Vec3, steering: bool, driven: bool) -> Self {
        Self {
            local_position: local_pos,
            radius: 0.33,
            width: 0.245,
            is_steering: steering,
            is_driven: driven,
            suspension_travel: 0.0,
            spring_force: 0.0,
            damper_force: 0.0,
            last_position: glam::Vec3::ZERO,
            angular_velocity: 0.0,
            longitudinal_slip: 0.0,
            lateral_slip: 0.0,
            current_friction: 1.0,
            surface_type: SurfaceType::Tarmac,
            is_contact: false,
            contact_point: None,
            contact_normal: None,
            penetration_depth: 0.0,
        }
    }
}

/// Vehicle component acting as the runtime hub for vehicle physics
#[derive(Debug, Clone, Component)]
pub struct Vehicle {
    // Configuration
    pub config: VehicleConfig,

    // Wheels
    pub wheels: Vec<Wheel>,

    // Engine states
    pub rpm: f32,
    pub engine_torque: f32,
    pub throttle_input: f32,

    // User controller inputs
    pub brake_input: f32,
    pub handbrake: bool,
    pub steering_input: f32,      // -1.0 to 1.0
    pub clutch_input: f32,

    // Transmission state
    pub current_gear: i32,        // 0 = neutral, -1 = reverse
    pub transmission_type: TransmissionType,
    pub drive_type: DriveType,

    // Motion states
    pub velocity: glam::Vec3,
    pub acceleration: glam::Vec3,
    pub speed: f32,
    pub angular_velocity: glam::Vec3,

    // Rapier3D linkage
    pub rigid_body_handle: RigidBodyHandle,
}

impl Vehicle {
    pub fn new(config: VehicleConfig, rigid_body: RigidBodyHandle) -> Self {
        // Initialize 4 wheels at default positions relative to chassis
        let front_track = config.track_width_front / 2.0;
        let rear_track = config.track_width_rear / 2.0;
        let wheelbase_front = config.wheelbase * 0.5;
        let wheelbase_rear = -config.wheelbase * 0.5;

        let mut wheels = Vec::with_capacity(4);

        // Front left wheel (steering, driven based on drive type)
        wheels.push(Wheel::new(
            glam::Vec3::new(-front_track, -config.wheel_radius, wheelbase_front),
            true,
            config.drive_type == DriveType::FrontWheelDrive || config.drive_type == DriveType::AllWheelDrive,
        ));

        // Front right wheel
        wheels.push(Wheel::new(
            glam::Vec3::new(front_track, -config.wheel_radius, wheelbase_front),
            true,
            config.drive_type == DriveType::FrontWheelDrive || config.drive_type == DriveType::AllWheelDrive,
        ));

        // Rear left wheel
        wheels.push(Wheel::new(
            glam::Vec3::new(-rear_track, -config.wheel_radius, wheelbase_rear),
            false,
            config.drive_type == DriveType::RearWheelDrive || config.drive_type == DriveType::AllWheelDrive,
        ));

        // Rear right wheel
        wheels.push(Wheel::new(
            glam::Vec3::new(rear_track, -config.wheel_radius, wheelbase_rear),
            false,
            config.drive_type == DriveType::RearWheelDrive || config.drive_type == DriveType::AllWheelDrive,
        ));

        Self {
            config,
            wheels,
            rpm: config.idle_rpm,
            engine_torque: 0.0,
            throttle_input: 0.0,
            brake_input: 0.0,
            handbrake: false,
            steering_input: 0.0,
            clutch_input: 0.0,
            current_gear: 1,
            transmission_type: config.transmission_type,
            drive_type: config.drive_type,
            velocity: glam::Vec3::ZERO,
            acceleration: glam::Vec3::ZERO,
            speed: 0.0,
            angular_velocity: glam::Vec3::ZERO,
            rigid_body_handle: rigid_body,
        }
    }
}
