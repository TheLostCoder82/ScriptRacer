//! Entity identification and generation system

use std::fmt;
use std::hash::Hash;

/// Entity identifier wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}

/// Entity ID generator for creating unique entities
#[derive(Debug)]
pub struct EntityGenerator {
    next_id: u64,
}

impl EntityGenerator {
    /// Creates a new entity generator starting at ID 0
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    /// Generates a new unique entity ID
    pub fn generate(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        Entity(id)
    }
}

impl Default for EntityGenerator {
    fn default() -> Self {
        Self::new()
    }
}
