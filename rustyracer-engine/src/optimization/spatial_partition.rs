//! Octree Spatial Partitioning System

use std::collections::VecDeque;
use crate::prelude::*;
use crate::ecs::{component::Component, system::System, world::World};

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
    
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
    
    /// Check if this AABB intersects with another
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
    
    /// Check if this AABB contains a point
    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}

/// Octree node
struct OctreeNode {
    bounds: Aabb,
    children: Option<Box<[OctreeNode; 8]>>,
    entities: Vec<u64>,
    capacity: usize,
}

impl OctreeNode {
    fn new(bounds: Aabb, capacity: usize) -> Self {
        Self {
            bounds,
            children: None,
            entities: Vec::new(),
            capacity,
        }
    }
    
    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
    
    fn subdivide(&mut self) {
        let center = self.bounds.center();
        let size = self.bounds.size() * 0.5;
        
        let mut children = Vec::with_capacity(8);
        
        for i in 0..8 {
            let offset = Vec3::new(
                if i & 1 != 0 { size.x } else { -size.x },
                if i & 2 != 0 { size.y } else { -size.y },
                if i & 4 != 0 { size.z } else { -size.z },
            );
            
            let child_bounds = Aabb::new(
                center + offset - size * 0.5,
                center + offset + size * 0.5,
            );
            
            children.push(OctreeNode::new(child_bounds, self.capacity));
        }
        
        self.children = Some(children.into_boxed_slice().try_into().unwrap());
        
        // Redistribute entities to children
        let entities = std::mem::take(&mut self.entities);
        for entity in entities {
            self.insert(entity);
        }
    }
    
    fn insert(&mut self, entity: u64) {
        // If leaf and not full, add entity here
        if self.is_leaf() {
            if self.entities.len() < self.capacity {
                self.entities.push(entity);
                return;
            }
            // Subdivide if over capacity
            self.subdivide();
        }
        
        // Try to insert into children
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                // Would need entity bounds to determine which child
                // For now, just add to first child that can accept
                if child.entities.len() < child.capacity {
                    child.insert(entity);
                    return;
                }
            }
        }
        
        // Fallback: keep at this level
        self.entities.push(entity);
    }
    
    fn query(&self, bounds: &Aabb, result: &mut Vec<u64>) {
        if !self.bounds.intersects(bounds) {
            return;
        }
        
        // Add entities in this node
        for &entity in &self.entities {
            result.push(entity);
        }
        
        // Query children
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(bounds, result);
            }
        }
    }
}

/// Octree spatial partitioning structure
pub struct Octree {
    root: OctreeNode,
    entity_bounds: hashbrown::HashMap<u64, Aabb>,
}

impl Octree {
    pub fn new(bounds: Aabb, capacity: usize) -> Self {
        Self {
            root: OctreeNode::new(bounds, capacity),
            entity_bounds: hashbrown::HashMap::new(),
        }
    }
    
    /// Insert an entity with its bounds
    pub fn insert(&mut self, entity: u64, bounds: Aabb) {
        self.entity_bounds.insert(entity, bounds);
        self.root.insert(entity);
    }
    
    /// Remove an entity
    pub fn remove(&mut self, entity: u64) {
        self.entity_bounds.remove(&entity);
        // Note: Full removal from octree would require rebuild or lazy removal
    }
    
    /// Query entities within bounds
    pub fn query(&self, bounds: &Aabb) -> Vec<u64> {
        let mut result = Vec::new();
        self.root.query(bounds, &mut result);
        result
    }
    
    /// Update entity bounds
    pub fn update(&mut self, entity: u64, new_bounds: Aabb) {
        self.entity_bounds.insert(entity, new_bounds);
        self.root.insert(entity);
    }
}

/// Visible entities resource
#[derive(Default)]
pub struct VisibleEntities {
    pub entities: Vec<u64>,
}

/// Spatial partitioning system
pub struct SpatialPartitionSystem {
    octree: Octree,
    world_bounds: Aabb,
}

impl SpatialPartitionSystem {
    pub fn new(world_bounds: Aabb) -> Self {
        Self {
            octree: Octree::new(world_bounds, 4),
            world_bounds,
        }
    }
    
    /// Build AABB from transform
    fn build_aabb(transform: &Transform, size: Vec3) -> Aabb {
        let half_size = size * 0.5;
        Aabb::new(
            transform.position - half_size,
            transform.position + half_size,
        )
    }
}

impl System for SpatialPartitionSystem {
    fn run(&mut self, world: &mut World) {
        // Clear and rebuild octree each frame
        // In production, would do incremental updates
        
        // Get camera for view frustum culling
        let camera_transform = world.get_resource::<Transform>().cloned();
        
        // Process all entities with Transform and MeshRenderer
        // Note: MeshRenderer not defined yet, using Transform only for demo
        let mut visible = Vec::new();
        
        for (entity, transform) in world.iter_components::<Transform>() {
            // Create approximate bounds
            let bounds = Self::build_aabb(transform, Vec3::new(1.0, 1.0, 1.0));
            self.octree.update(entity, bounds);
            
            // Simple distance-based visibility check
            if let Some(cam) = &camera_transform {
                let distance = (transform.position - cam.position).length();
                if distance < 100.0 {
                    visible.push(entity);
                }
            } else {
                visible.push(entity);
            }
        }
        
        // Store visible entities as resource
        world.add_resource(VisibleEntities { entities: visible });
    }
}
