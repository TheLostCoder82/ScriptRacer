//! Physics ECS systems

use crate::prelude::*;
use crate::ecs::{World, System};
use crate::core::time::Time;
use crate::physics::world::PhysicsWorld;
use crate::physics::components::{RigidBody, Collider, RigidBodyType};

/// Physics simulation system
pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn initialize(&mut self, world: &mut World) {
        world.insert_resource(PhysicsWorld::default());
    }
    
    fn run(&mut self, world: &mut World) {
        let time = world.get_resource::<Time>().cloned();
        let dt = time.map(|t| t.fixed_delta_time()).unwrap_or(1.0 / 60.0);
        
        let mut physics_world = world.get_resource_mut::<PhysicsWorld>();
        if let Some(mut pw) = physics_world {
            pw.step(dt);
        }
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self
    }
}

/// Physics synchronization system - syncs physics state with transforms
pub struct PhysicsSyncSystem;

impl System for PhysicsSyncSystem {
    fn run(&mut self, world: &mut World) {
        let physics_world = world.get_resource::<PhysicsWorld>();
        if physics_world.is_none() {
            return;
        }
        let physics_world = physics_world.unwrap();
        
        // Physics to Transform: Update transforms from physics simulation for dynamic bodies
        {
            let mut query = world.query::<(&RigidBody, &mut Transform)>();
            for (entity, (rigid_body, transform)) in query.iter_mut(world) {
                if rigid_body.body_type == RigidBodyType::Dynamic {
                    if let Some(rb) = physics_world.rigidbody_set.get(rigid_body.handle) {
                        let pos = rb.position();
                        let rot = rb.rotation();
                        
                        transform.position = Vec3::new(pos.translation.x, pos.translation.y, pos.translation.z);
                        transform.rotation = Quat::from_xyzw(rot.x, rot.y, rot.z, rot.w);
                    }
                }
            }
        }
        
        // Transform to Physics: Update physics bodies from transforms for kinematic/static bodies
        {
            let mut query = world.query::<(&mut RigidBody, &Transform)>();
            for (entity, (rigid_body, transform)) in query.iter_mut(world) {
                if rigid_body.body_type != RigidBodyType::Dynamic {
                    if let Some(mut rb) = physics_world.rigidbody_set.get_mut(rigid_body.handle) {
                        let pos = na::Vector3::new(transform.position.x, transform.position.y, transform.position.z);
                        let rot = na::Quaternion::new(transform.rotation.w, transform.rotation.x, transform.rotation.y, transform.rotation.z);
                        
                        rb.set_translation(pos, true);
                        rb.set_rotation(na::UnitQuaternion::from_quaternion(rot), true);
                    }
                }
            }
        }
    }
}

impl Default for PhysicsSyncSystem {
    fn default() -> Self {
        Self
    }
}
