//! Asset manager for tracking and loading assets

use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::core::{EngineResult, EngineError};
use crate::assets::handle::AssetHandle;

/// Placeholder trait for all assets
pub trait Asset: Clone + std::fmt::Debug + Send + Sync + 'static {}

impl<T: Clone + std::fmt::Debug + Send + Sync + 'static> Asset for T {}

/// Manager for loaded assets
pub struct AssetManager {
    assets: HashMap<u64, Box<dyn Any + Send + Sync>>,
    path_map: HashMap<String, u64>,
    next_id: u64,
}

impl AssetManager {
    /// Creates a new empty asset manager
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            path_map: HashMap::new(),
            next_id: 0,
        }
    }

    /// Loads an asset from a path (placeholder implementation for Phase 1)
    pub fn load<T: Asset>(&mut self, path: &str) -> EngineResult<AssetHandle<T>> {
        // Check if already loaded
        if let Some(&id) = self.path_map.get(path) {
            return Ok(AssetHandle::new(id));
        }

        // Allocate a placeholder asset (Phase 1 skeleton)
        let id = self.next_id;
        self.next_id += 1;

        // Create a zeroed placeholder - this will be replaced with actual loading in Phase 2
        let placeholder = unsafe { std::mem::zeroed::<T>() };
        self.assets.insert(id, Box::new(placeholder));
        self.path_map.insert(path.to_string(), id);

        log::info!("Loaded asset placeholder: {} (id: {})", path, id);
        Ok(AssetHandle::new(id))
    }

    /// Gets an immutable reference to an asset
    pub fn get<T: Asset>(&self, handle: AssetHandle<T>) -> Option<&T> {
        self.assets.get(&handle.id).and_then(|b| b.downcast_ref::<T>())
    }

    /// Gets a mutable reference to an asset
    pub fn get_mut<T: Asset>(&mut self, handle: AssetHandle<T>) -> Option<&mut T> {
        self.assets.get_mut(&handle.id).and_then(|b| b.downcast_mut::<T>())
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
