//! Cache-friendly component storage system using sparse sets

use std::any::{Any, TypeId};
use crate::ecs::entity::Entity;

/// Blanket marker trait for all components
pub trait Component: Clone + Send + Sync + 'static + std::fmt::Debug {}

impl<T: Clone + Send + Sync + 'static + std::fmt::Debug> Component for T {}

/// Type-erased object trait for component storage
pub trait ComponentStorage: Any + Send + Sync {
    fn remove(&mut self, entity: Entity);
    fn has(&self, entity: Entity) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Sparse set storage for a specific component type
pub struct SparseStorage<T: Component> {
    sparse: Vec<Option<usize>>,
    dense: Vec<Entity>,
    data: Vec<T>,
}

impl<T: Component> SparseStorage<T> {
    /// Creates a new empty sparse storage
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Inserts or updates a component for an entity
    pub fn insert(&mut self, entity: Entity, component: T) {
        let entity_id = entity.0 as usize;
        
        // Ensure sparse array is large enough
        while self.sparse.len() <= entity_id {
            self.sparse.push(None);
        }

        // Check if entity already exists
        if let Some(index) = self.sparse[entity_id] {
            // Update existing component
            self.data[index] = component;
        } else {
            // Add new component
            let index = self.data.len();
            self.sparse[entity_id] = Some(index);
            self.dense.push(entity);
            self.data.push(component);
        }
    }

    /// Gets an immutable reference to a component
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let entity_id = entity.0 as usize;
        self.sparse.get(entity_id).and_then(|&opt_index| {
            opt_index.map(|index| &self.data[index])
        })
    }

    /// Gets a mutable reference to a component
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let entity_id = entity.0 as usize;
        self.sparse.get_mut(entity_id).and_then(|opt_index| {
            opt_index.map(|index| &mut self.data[index])
        })
    }

    /// Removes a component from an entity
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let entity_id = entity.0 as usize;
        
        if let Some(Some(index)) = self.sparse.get(entity_id).copied().flatten() {
            let component = self.data.remove(index);
            let last_entity = self.dense.pop();
            
            // If we removed the last element, just clear the sparse entry
            if index < self.data.len() {
                if let Some(last_entity) = last_entity {
                    let last_entity_id = last_entity.0 as usize;
                    self.sparse[last_entity_id] = Some(index);
                    self.dense[index] = last_entity;
                }
            }
            
            self.sparse[entity_id] = None;
            Some(component)
        } else {
            None
        }
    }

    /// Returns an iterator over all entities and their components
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.dense.iter().zip(self.data.iter()).map(|(&e, c)| (e, c))
    }

    /// Returns a mutable iterator over all entities and their components
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.dense.iter().copied().zip(self.data.iter_mut()).map(|(e, c)| (e, c))
    }
}

impl<T: Component> Default for SparseStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component> ComponentStorage for SparseStorage<T> {
    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    }

    fn has(&self, entity: Entity) -> bool {
        self.get(entity).is_some()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
