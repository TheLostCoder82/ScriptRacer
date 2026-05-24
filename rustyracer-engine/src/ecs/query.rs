//! Compile-time query framework for type-safe iteration over components

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::ecs::entity::Entity;
use crate::ecs::world::World;
use crate::ecs::component::{Component, ComponentStorage, SparseStorage};

/// Trait for types that can be queried from the world
pub trait Queryable {
    type Item<'a>: 'a where Self: 'a;
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>>;
}

/// Query for iterating over entities with specific components
pub struct Query<'w, T: Queryable> {
    world: &'w World,
    _marker: PhantomData<T>,
}

impl<'w, T: Queryable> Query<'w, T> {
    /// Creates a new query
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }

    /// Iterates over all matching entities
    pub fn iter(&self) -> impl Iterator<Item = T::Item<'_>> {
        // Get all entities from the world by iterating through component storages
        // This is a simplified implementation - in a real engine, this would be more efficient
        let mut entities = Vec::new();
        
        // Collect entities from the first component type in the query
        // For simplicity, we'll iterate through all entities that have any components
        for (_, storage) in &self.world.components {
            if let Some(sparse) = storage.as_any().downcast_ref::<SparseStorage<T::Item<'static>>>() {
                for (entity, _) in sparse.iter() {
                    if let Some(item) = T::fetch(self.world, entity) {
                        entities.push(item);
                    }
                }
            }
        }
        
        entities.into_iter()
    }
}

// Implementation for single immutable reference
impl<'a, T: Component> Queryable for &'a T {
    type Item<'b> = &'b T where 'a: 'b;

    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_component::<T>(entity)
    }
}

// Implementation for single mutable reference (requires special handling)
impl<'a, T: Component> Queryable for &'a mut T {
    type Item<'b> = &'b mut T where 'a: 'b;

    fn fetch(_world: &World, _entity: Entity) -> Option<Self::Item<'_>> {
        // Mutable queries require special handling in the scheduler
        // This is a placeholder - actual implementation would use unsafe pointers
        None
    }
}

/// Macro to implement Queryable for tuple types up to size 4
macro_rules! impl_queryable_tuple {
    ($($name:ident),*) => {
        impl<'a, $($name: Component),*> Queryable for ($(&'a $name,)*) {
            type Item<'b> = ($(&'b $name,)*) where 'a: 'b;

            fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>> {
                $(
                    let $name = world.get_component::<$name>(entity)?;
                )*
                Some(($($name,)*))
            }
        }
    };
}

impl_queryable_tuple!(A, B);
impl_queryable_tuple!(A, B, C);
impl_queryable_tuple!(A, B, C, D);
