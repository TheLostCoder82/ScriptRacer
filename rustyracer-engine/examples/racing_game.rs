//! Racing Game Integration Demo
//! 
//! A standalone executable environment showcasing Phase 3 racing features.

use rustyracer_engine::prelude::*;
use rustyracer_engine::physics::vehicle::{Vehicle, VehicleConfig, VehiclePhysicsSystem};
use rustyracer_engine::input::racing::{RacingInputState, RacingInputSystem};
use rustyracer_engine::racing_lib::{
    Checkpoint, RaceInfo, LapTimer, RacingCamera, CameraMode,
    RaceManagerSystem, CheckpointSystem, LapTimerSystem,
};
use glam::Vec3;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodyType};
use rapier3d::geometry::ColliderBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🏁 RustyRacer — Racing Demo");
    
    // 1. Spin up an engine shell configuring typical display resolution rules
    let mut engine = Engine::new(EngineConfig {
        title: "RustyRacer — Racing Demo".to_string(),
        width: 1920,
        height: 1080,
        fullscreen: false,
        vsync: true,
        ..Default::default()
    })?;
    
    // 2. Programmatically scatter 4 sequential Checkpoint component definitions
    // creating a virtual tracking circuit
    let checkpoint_positions = vec![
        Vec3::new(0.0, 0.0, 0.0),      // Start/Finish line
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(50.0, 0.0, 50.0),
        Vec3::new(0.0, 0.0, 50.0),
    ];
    
    for (i, pos) in checkpoint_positions.iter().enumerate() {
        let is_start_finish = i == 0;
        let checkpoint = Checkpoint::new(i as u32, *pos, 5.0, is_start_finish);
        
        engine.world.spawn((
            checkpoint,
            Transform::from_translation(*pos),
        ));
        
        log::info!("Spawned checkpoint {} at {:?}", i, pos);
    }
    
    // 3. Construct a standard player Vehicle rig with Rapier3D rigid body
    let vehicle_config = VehicleConfig::default();
    
    // Create Rapier rigid body
    let rigid_body_builder = RigidBodyBuilder::dynamic()
        .translation(Vec3::new(0.0, 1.0, 0.0).into())
        .mass(vehicle_config.mass)
        .inertia_tensor(vehicle_config.inertia_tensor.into());
    
    let physics_world = engine.world.get_resource_mut::<PhysicsWorld>()
        .expect("PhysicsWorld not found");
    
    let rigid_body_handle = physics_world.insert_rigidbody(rigid_body_builder.build());
    
    // Add collider
    let collider = ColliderBuilder::cuboid(2.0, 0.5, 4.0)
        .density(1000.0)
        .build();
    physics_world.insert_collider(collider, rigid_body_handle);
    
    drop(physics_world);
    
    // Spawn vehicle entity
    let vehicle_entity = engine.world.spawn((
        Vehicle::new(vehicle_config, rigid_body_handle),
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        RacingInputState::default(),
    ));
    
    log::info!("Spawned player vehicle");
    
    // 4. Spawn an engine camera instance initializing into Chase mode
    let camera_entity = engine.world.spawn((
        RacingCamera::new(CameraMode::Chase, vehicle_entity.id()),
        Transform::from_translation(Vec3::new(0.0, 3.0, -8.0)),
    ));
    
    // Set as active camera
    engine.world.insert_resource(camera_entity);
    
    // 5. Register race info and lap timer
    engine.world.spawn((
        RaceInfo::new("Demo Circuit", 3),
        LapTimer::default(),
    ));
    
    // 6. Manually register our system architecture types into pipeline scheduler queues
    engine.add_system(VehiclePhysicsSystem::new());
    engine.add_system(RacingInputSystem::new());
    engine.add_system(CheckpointSystem::new());
    engine.add_system(LapTimerSystem::new());
    engine.add_system(RaceManagerSystem::new());
    engine.add_system(RacingCameraSystem::new());
    
    println!("✅ All racing systems registered");
    println!("🎮 Controls:");
    println!("   W/Up - Throttle");
    println!("   S/Down - Brake");
    println!("   A/D or Left/Right - Steer");
    println!("   Space - Handbrake");
    println!("   C - Cycle camera view");
    println!("   R - Reset vehicle");
    println!();
    
    // 7. Kick off standard continuous engine loop operations
    println!("🚀 Starting engine...");
    engine.run()?;
    
    Ok(())
}
