//! World module containing the unified state container for ECS

use std::any::{Any, TypeId};
use std::collections::HashMap;
use log::{info, trace};

use crate::ecs::entity::{Entity, EntityGenerator};
use crate::ecs::component::{Component, ComponentStorage, SparseStorage};

/// Blanket trait for global state resources
pub trait Resource: Send + Sync + 'static + Any + Clone {}

impl<T: Send + Sync + 'static + Any + Clone> Resource for T {}

/// The unified state container for all entities, components, and resources
pub struct World {
    entity_gen: EntityGenerator,
    components: HashMap<TypeId, Box<dyn ComponentStorage>>,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl World {
    /// Creates a new empty world
    pub fn new() -> Self {
        Self {
            entity_gen: EntityGenerator::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    /// Creates a new entity
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.entity_gen.generate();
        trace!("Created entity: {}", entity);
        entity
    }

    /// Destroys an entity and removes all its components
    pub fn destroy_entity(&mut self, entity: Entity) {
        trace!("Destroying entity: {}", entity);
        for storage in self.components.values_mut() {
            storage.remove(entity);
        }
    }

    /// Registers a component type in the world
    pub fn register_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            self.components.insert(type_id, Box::new(SparseStorage::<T>::new()));
            info!("Registered component type: {}", std::any::type_name::<T>());
        }
    }

    /// Adds a component to an entity
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.register_component::<T>();
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get_mut(&type_id) {
            let storage = storage.as_any_mut().downcast_mut::<SparseStorage<T>>().unwrap();
            storage.insert(entity, component);
            trace!("Added component to {}: {}", std::any::type_name::<T>(), entity);
        }
    }

    /// Gets an immutable reference to a component
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id).and_then(|storage| {
            storage.as_any().downcast_ref::<SparseStorage<T>>().and_then(|s| s.get(entity))
        })
    }

    /// Gets a mutable reference to a component
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id).and_then(|storage| {
            storage.as_any_mut().downcast_mut::<SparseStorage<T>>().and_then(|s| s.get_mut(entity))
        })
    }

    /// Checks if an entity has a specific component
    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id).map_or(false, |storage| storage.has(entity))
    }

    /// Adds a resource to the world
    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
        info!("Added resource: {}", std::any::type_name::<R>());
    }

    /// Gets an immutable reference to a resource
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        let type_id = TypeId::of::<R>();
        self.resources.get(&type_id).and_then(|r| r.downcast_ref::<R>())
    }

    /// Gets a mutable reference to a resource
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        let type_id = TypeId::of::<R>();
        self.resources.get_mut(&type_id).and_then(|r| r.downcast_mut::<R>())
    }

    /// Iterates over all entities with a specific component
    pub fn iter_components<T: Component>(&self) -> impl Iterator<Item = (Entity, &T)> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<SparseStorage<T>>())
            .map(|s| s.iter().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
