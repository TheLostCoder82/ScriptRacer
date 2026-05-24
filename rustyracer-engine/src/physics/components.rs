//! Physics components for Rapier3D integration

use crate::prelude::*;
use crate::ecs::component::Component;
use rapier3d::dynamics;
use rapier3d::geometry;

/// Rigid body type enum
#[derive(Clone, Copy, Debug)]
pub enum RigidBodyType {
    Static,
    Dynamic,
    KinematicPositionBased,
    KinematicVelocityBased,
}

impl From<RigidBodyType> for dynamics::RigidBodyType {
    fn from(ty: RigidBodyType) -> Self {
        match ty {
            RigidBodyType::Static => dynamics::RigidBodyType::Fixed,
            RigidBodyType::Dynamic => dynamics::RigidBodyType::Dynamic,
            RigidBodyType::KinematicPositionBased => dynamics::RigidBodyType::KinematicPositionBased,
            RigidBodyType::KinematicVelocityBased => dynamics::RigidBodyType::KinematicVelocityBased,
        }
    }
}

/// ECS component for rigid body
#[derive(Clone, Debug)]
pub struct RigidBody {
    pub handle: dynamics::RigidBodyHandle,
    pub body_type: RigidBodyType,
    pub mass: f32,
}

impl Component for RigidBody {}

/// Collider shape enum
#[derive(Clone, Debug)]
pub enum ColliderShape {
    Ball(f32),
    Cuboid(Vec3),
    Capsule { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
    TriMesh,
}

/// ECS component for collider
#[derive(Clone, Debug)]
pub struct Collider {
    pub handle: geometry::ColliderHandle,
    pub shape: ColliderShape,
}

impl Component for Collider {}
