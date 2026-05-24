//! Asset handle for type-safe asset tracking

use std::hash::Hash;
use std::marker::PhantomData;

/// Handle to a loaded asset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle<T: std::fmt::Debug + 'static> {
    id: u64,
    _marker: PhantomData<T>,
}

impl<T: std::fmt::Debug + 'static> AssetHandle<T> {
    /// Creates a new asset handle from an ID
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    /// Returns the internal ID of this handle
    pub fn id(&self) -> u64 {
        self.id
    }
}
