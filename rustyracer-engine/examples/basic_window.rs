//! Basic window example demonstrating the engine framework

use rustyracer_engine::prelude::*;
use rustyracer_engine::platform::{Window, InputSystem, InputState};
use rustyracer_engine::ecs::{World, Scheduler};
use rustyracer_engine::utils::logging::init_logging;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

fn main() {
    // Initialize configuration and logging
    let config = EngineConfig::default();
    init_logging(&config);

    log::info!("Starting RustyRacer Engine example");

    // Create world and scheduler
    let mut world = World::new();
    let mut scheduler = Scheduler::new();

    // Add input system and initialize
    scheduler.add_system(InputSystem);
    scheduler.initialize(&mut world);

    // Create event loop and window
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = Window::new(config.window.clone(), &event_loop)
        .expect("Failed to create window");

    log::info!("Window created: {}x{}", config.window.width, config.window.height);

    // Run the event loop
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    log::info!("Close requested, shutting down");
                    elwt.exit();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let Some(input_state) = world.get_resource_mut::<InputState>() {
                        input_state.process_key_event(&event);
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Update time and run systems
                scheduler.run(&mut world);
                
                // Request redraw
                window.inner().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Render frame would go here
            }
            _ => {}
        }
    }).expect("Event loop failed");
}
