//! Physics world wrapper for Rapier3D

use crate::prelude::*;
use crate::physics::components::{RigidBody, Collider, RigidBodyType, ColliderShape};
use rapier3d::dynamics::{RigidBodySet, RigidBodyHandle, RigidBodyBuilder, IntegrationParameters, ImpulseJointSet, MultibodyJointSet, IslandManager};
use rapier3d::geometry::{ColliderSet, ColliderHandle, SharedShape, BroadPhase, NarrowPhase};
use rapier3d::pipeline::PhysicsPipeline;
use rapier3d::control::QueryPipeline;

/// Physics world wrapper containing all Rapier3D simulation components
pub struct PhysicsWorld {
    pub gravity: Vec3,
    pub integration_parameters: IntegrationParameters,
    pub rigidbody_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub physics_pipeline: PhysicsPipeline,
    pub query_pipeline: QueryPipeline,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            integration_parameters: IntegrationParameters::default(),
            rigidbody_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            physics_pipeline: PhysicsPipeline::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }
}

impl PhysicsWorld {
    /// Steps the physics simulation
    pub fn step(&mut self, delta_time: f32) {
        self.integration_parameters.dt = delta_time;
        
        let gravity = na::Vector3::new(self.gravity.x, self.gravity.y, self.gravity.z);
        
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigidbody_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            None,
            &mut self.query_pipeline,
        );
    }
    
    /// Creates a new rigid body
    pub fn create_rigidbody(
        &mut self,
        ty: RigidBodyType,
        position: Vec3,
        rotation: Quat,
        mass: f32,
    ) -> RigidBody {
        let rb_type: dynamics::RigidBodyType = ty.into();
        
        let builder = RigidBodyBuilder::new(rb_type)
            .translation(na::Vector3::new(position.x, position.y, position.z))
            .rotation(na::UnitQuaternion::from_quaternion(na::Quaternion::new(
                rotation.w, rotation.x, rotation.y, rotation.z,
            )));
        
        let handle = self.rigidbody_set.insert(builder.build());
        
        RigidBody {
            handle,
            body_type: ty,
            mass,
        }
    }
    
    /// Creates a new collider
    pub fn create_collider(
        &mut self,
        shape: ColliderShape,
        rb: &RigidBody,
        friction: f32,
        restitution: f32,
    ) -> Collider {
        let shared_shape: SharedShape = match &shape {
            ColliderShape::Ball(radius) => SharedShape::ball(*radius),
            ColliderShape::Cuboid(half_extents) => SharedShape::cuboid(
                half_extents.x,
                half_extents.y,
                half_extents.z,
            ),
            ColliderShape::Capsule { radius, height } => SharedShape::capsule_y(*radius, *height),
            ColliderShape::Cylinder { radius, height } => SharedShape::cylinder_y(*radius, *height),
            ColliderShape::TriMesh => SharedShape::trimesh(Vec::new(), Vec::new()),
        };
        
        let collider_builder = rapier3d::geometry::ColliderBuilder::new(shared_shape)
            .friction(friction)
            .restitution(restitution);
        
        let handle = self.collider_set.insert_with_parent(collider_builder.build(), rb.handle, &mut self.rigidbody_set);
        
        Collider { handle, shape }
    }
}
