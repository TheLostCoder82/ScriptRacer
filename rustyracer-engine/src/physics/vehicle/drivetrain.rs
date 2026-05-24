//! Drivetrain calculations for vehicle physics

use super::components::Vehicle;
use super::data::{DriveType, TransmissionType};

/// Update engine RPM and torque based on throttle input
pub fn update_engine(vehicle: &mut Vehicle, dt: f32) {
    // Interpolate torque from torque curve based on current RPM
    let rpm_normalized = vehicle.rpm / vehicle.config.max_rpm;
    
    // Simple linear interpolation of torque curve
    let torque_multiplier = interpolate_torque_curve(&vehicle.config.torque_curve, vehicle.rpm);
    
    // Calculate engine torque based on throttle and torque curve
    let base_torque = vehicle.config.max_torque * torque_multiplier;
    vehicle.engine_torque = base_torque * vehicle.throttle_input;
    
    // Update RPM based on torque and inertia (simplified)
    let engine_inertia = 0.5; // kg·m² (simplified)
    let angular_accel = vehicle.engine_torque / engine_inertia;
    
    // Convert to RPM change
    let rpm_change = angular_accel * dt * 60.0 / std::f32::consts::PI * 2.0;
    vehicle.rpm += rpm_change;
    
    // Clamp RPM to valid range
    vehicle.rpm = vehicle.rpm.clamp(vehicle.config.idle_rpm, vehicle.config.max_rpm);
}

/// Update transmission state and gear selection
pub fn update_transmission(vehicle: &mut Vehicle, dt: f32) {
    match vehicle.transmission_type {
        TransmissionType::Automatic => {
            // Automatic shifting based on RPM thresholds
            let upshift_rpm = vehicle.config.max_rpm * 0.85;
            let downshift_rpm = vehicle.config.max_rpm * 0.4;
            
            if vehicle.rpm > upshift_rpm && vehicle.current_gear < vehicle.config.gears.len() as i32 {
                vehicle.current_gear += 1;
            } else if vehicle.rpm < downshift_rpm && vehicle.current_gear > 1 {
                vehicle.current_gear -= 1;
            }
        }
        TransmissionType::Manual | TransmissionType::Sequential => {
            // Manual/sequential - gear changes handled by input
            // Just ensure we don't exceed bounds
            let max_gear = vehicle.config.gears.len() as i32;
            vehicle.current_gear = vehicle.current_gear.clamp(0, max_gear);
        }
    }
}

/// Update drivetrain forces based on engine torque and gear ratios
pub fn update_drivetrain_forces(vehicle: &mut Vehicle, dt: f32) {
    if vehicle.current_gear <= 0 || vehicle.current_gear > vehicle.config.gears.len() as i32 {
        return;
    }
    
    let gear_index = (vehicle.current_gear - 1) as usize;
    let gear_ratio = vehicle.config.gears[gear_index].ratio;
    let final_drive = vehicle.config.final_drive_ratio;
    
    // Calculate total gear reduction
    let total_ratio = gear_ratio * final_drive;
    
    // Calculate wheel torque from engine torque
    let wheel_torque = vehicle.engine_torque * total_ratio;
    
    // Apply torque to driven wheels
    for (i, wheel) in vehicle.wheels.iter_mut().enumerate() {
        if !wheel.is_driven || !wheel.is_contact {
            continue;
        }
        
        // Distribute torque among driven wheels
        let num_driven = match vehicle.drive_type {
            DriveType::FrontWheelDrive => 2,
            DriveType::RearWheelDrive => 2,
            DriveType::AllWheelDrive => 4,
        };
        
        let wheel_drive_torque = wheel_torque / num_driven as f32;
        
        // Apply torque to wheel rotation
        let wheel_inertia = 0.5 * vehicle.config.wheel_mass * wheel.radius * wheel.radius;
        let angular_accel = wheel_drive_torque / (wheel_inertia + 0.001);
        wheel.angular_velocity += angular_accel * dt;
    }
}

/// Interpolate torque value from the torque curve at given RPM
fn interpolate_torque_curve(torque_curve: &[(f32, f32)], rpm: f32) -> f32 {
    if torque_curve.is_empty() {
        return 1.0;
    }
    
    // Find surrounding points
    let mut lower = None;
    let mut upper = None;
    
    for (i, &(curve_rpm, _)) in torque_curve.iter().enumerate() {
        if curve_rpm <= rpm {
            lower = Some(i);
        }
        if curve_rpm >= rpm && upper.is_none() {
            upper = Some(i);
        }
    }
    
    match (lower, upper) {
        (Some(l), Some(u)) if l == u => torque_curve[l].1,
        (Some(l), Some(u)) => {
            let (rpm1, torque1) = torque_curve[l];
            let (rpm2, torque2) = torque_curve[u];
            
            if rpm2 == rpm1 {
                return torque1;
            }
            
            let t = (rpm - rpm1) / (rpm2 - rpm1);
            torque1 + t * (torque2 - torque1)
        }
        (Some(l), None) => torque_curve[l].1,
        (None, Some(u)) => torque_curve[u].1,
        (None, None) => 1.0,
    }
}
