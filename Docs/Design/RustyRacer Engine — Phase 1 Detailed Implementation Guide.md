RustyRacer Engine — Phase 1 Detailed Implementation Guide  
   
Phase 1 Goal: Build the core foundation: project setup, custom ECS framework, math integration, asset manager basics, window/input foundation, and core utilities.  
Timeline: Weeks 1–4 | Skill Level: Beginner Rust | Target OS: Ubuntu 24.04 LTS  
   
   
   
🎯 Phase 1 Overview  
   
We will build the absolute base of the engine. By the end of this phase you will have:  
✅ Working project structure & build system  
✅ Custom ECS implementation (entities, components, world, basic scheduler)  
✅ Math subsystem integrated with  glam   
✅ Basic asset manager skeleton  
✅ Window creation with  winit   
✅ Input system base (keyboard/mouse/events)  
✅ Core utilities: logging, error handling, time management  
   
   
   
📋 Prerequisites  
   
System Setup  
   
bash    
\# Update system  
sudo apt update && sudo apt upgrade \-y

\# Install required system libraries  
sudo apt install \-y build-essential pkg-config libssl-dev \\  
  libasound2-dev libudev-dev libxcb-randr0-dev libx11-dev \\  
  libxext-dev libxi-dev libxcursor-dev git curl

\# Install Rust via rustup (if not already installed)  
curl \--proto '=https' \--tlsv1.2 \-sSf https://sh.rustup.rs | sh  
source $HOME/.cargo/env

\# Verify versions  
rustc \--version  \# Should be 1.75+ stable  
cargo \--version  
   
   
\- Use Rust Rover as your IDE: https://www.jetbrains.com/rustrover/  
   
   
   
📁 Step 1 — Project Structure & Cargo.toml  
   
Create the project folder and set up the workspace structure.  
   
1.1 Create Project  
   
bash    
cargo new rustyracer-engine  
cd rustyracer-engine  
   
   
1.2 Final Directory Structure  
   
plaintext    
rustyracer-engine/  
├── Cargo.toml               \# Workspace manifest  
├── Cargo.lock  
├── README.md  
├── LICENSE  
├── src/  
│   ├── lib.rs               \# Public API root  
│   ├── core/                \# Core utilities, math, types  
│   │   ├── mod.rs  
│   │   ├── math.rs  
│   │   ├── time.rs  
│   │   ├── error.rs  
│   │   └── config.rs  
│   ├── ecs/                 \# Custom ECS implementation  
│   │   ├── mod.rs  
│   │   ├── entity.rs  
│   │   ├── component.rs  
│   │   ├── world.rs  
│   │   ├── system.rs  
│   │   ├── scheduler.rs  
│   │   └── query.rs  
│   ├── assets/              \# Asset management  
│   │   ├── mod.rs  
│   │   ├── handle.rs  
│   │   ├── manager.rs  
│   │   └── loader.rs  
│   ├── platform/            \# Window, events, input  
│   │   ├── mod.rs  
│   │   ├── window.rs  
│   │   ├── events.rs  
│   │   └── input.rs  
│   └── utils/               \# Helpers, logging  
│       ├── mod.rs  
│       ├── logging.rs  
│       └── macros.rs  
└── examples/                \# Test projects  
    └── basic\_window.rs  
   
   
1.3 Cargo.toml Configuration  
   
toml    
\[package\]  
name \= "rustyracer-engine"  
version \= "0.1.0"  
edition \= "2021"  
description \= "3D game engine written in Rust — learning project"  
license \= "MIT/Apache-2.0"

\[features\]  
default \= \[\]  
dev \= \["log/std", "env\_logger"\]

\[dependencies\]  
\# Math — SIMD-accelerated linear algebra  
glam \= { version \= "0.28", features \= \["std", "bytemuck", "serde"\] }

\# Window & event handling  
winit \= { version \= "0.30", features \= \["x11", "wayland", "serde"\] }

\# Serialization (for future networking/assets)  
serde \= { version \= "1.0", features \= \["derive"\] }  
serde\_json \= "1.0"

\# Logging & errors  
log \= "0.4"  
env\_logger \= "0.10"  
thiserror \= "1.0"  
anyhow \= "1.0"

\# Utilities  
hashbrown \= { version \= "0.14", features \= \["serde"\] } \# Faster hash maps  
once\_cell \= "1.19" \# Lazy static initialization

\# Dev dependencies for testing  
\[dev-dependencies\]  
criterion \= { version \= "0.5", features \= \["html\_reports"\] }  
   
   
1.4 src/lib.rs — Public API  
   
rust    
//\! RustyRacer Engine — Core Library

\#\!\[warn(missing\_docs)\]  
\#\!\[allow(dead\_code)\]

// Re-export core modules  
pub mod core;  
pub mod ecs;  
pub mod assets;  
pub mod platform;  
pub mod utils;

// Prelude — common imports  
pub mod prelude {  
    pub use crate::core::\*;  
    pub use crate::ecs::\*;  
    pub use crate::assets::\*;  
    pub use crate::platform::\*;  
    pub use glam::\*;  
    pub use winit::event::VirtualKeyCode as KeyCode;  
}  
   
   
   
   
🧮 Step 2 — Core Module Implementation  
   
This contains fundamental types, math wrappers, time, error handling.  
   
2.1 src/core/mod.rs  
   
rust    
//\! Core types, math, utilities and error handling

pub mod math;  
pub mod time;  
pub mod error;  
pub mod config;

// Re-export common core items  
pub use math::\*;  
pub use time::\*;  
pub use error::EngineError;  
   
   
2.2 src/core/error.rs  
   
rust    
//\! Custom error type for the engine  
use thiserror::Error;

\#\[derive(Error, Debug)\]  
pub enum EngineError {  
    /// Initialization failed  
    \#\[error("Initialization failed: {0}")\]  
    InitError(String),

    /// Asset loading failed  
    \#\[error("Asset load error: {0}")\]  
    AssetError(String),

    /// Invalid operation  
    \#\[error("Invalid operation: {0}")\]  
    InvalidOperation(String),

    /// Platform/window error  
    \#\[error("Platform error: {0}")\]  
    PlatformError(\#\[from\] winit::error::OsError),

    /// IO error  
    \#\[error("IO error: {0}")\]  
    IoError(\#\[from\] std::io::Error),

    /// Serialization error  
    \#\[error("Serialization error: {0}")\]  
    SerdeError(\#\[from\] serde\_json::Error),  
}

/// Result type alias for engine operations  
pub type EngineResult\<T\> \= Result\<T, EngineError\>;  
   
   
2.3 src/core/time.rs  
   
rust    
//\! Time management utilities — fixed timestep, delta time  
use std::time::{Instant, Duration};

/// Tracks frame timing and controls update rate  
\#\[derive(Debug, Clone)\]  
pub struct Time {  
    start\_time: Instant,  
    last\_frame\_time: Instant,  
    delta\_time: f32,  
    fixed\_delta\_time: f32,  
    accumulated\_time: f32,  
    frame\_count: u64,  
}

impl Time {  
    /// Create new time manager with default 60Hz fixed update  
    pub fn new() \-\> Self {  
        let now \= Instant::now();  
        Self {  
            start\_time: now,  
            last\_frame\_time: now,  
            delta\_time: 0.0,  
            fixed\_delta\_time: 1.0 / 60.0, // 60 updates per second  
            accumulated\_time: 0.0,  
            frame\_count: 0,  
        }  
    }

    /// Call once per frame to update timing values  
    pub fn update(\&mut self) {  
        let now \= Instant::now();  
        let elapsed \= now.duration\_since(self.last\_frame\_time);  
          
        self.delta\_time \= elapsed.as\_secs\_f32();  
        self.accumulated\_time \+= self.delta\_time;  
        self.last\_frame\_time \= now;  
        self.frame\_count \+= 1;  
    }

    /// Get time since last frame in seconds  
    pub fn delta\_time(\&self) \-\> f32 { self.delta\_time }

    /// Get fixed timestep interval (default 1/60s)  
    pub fn fixed\_delta\_time(\&self) \-\> f32 { self.fixed\_delta\_time }

    /// Get total elapsed time since engine start  
    pub fn elapsed\_time(\&self) \-\> f32 { self.start\_time.elapsed().as\_secs\_f32() }

    /// Check if we should run a fixed update step  
    pub fn should\_fixed\_update(\&mut self) \-\> bool {  
        if self.accumulated\_time \>= self.fixed\_delta\_time {  
            self.accumulated\_time \-= self.fixed\_delta\_time;  
            return true;  
        }  
        false  
    }

    /// Get interpolation factor between fixed steps  
    pub fn interpolation\_alpha(\&self) \-\> f32 {  
        self.accumulated\_time / self.fixed\_delta\_time  
    }  
}  
   
   
2.4 src/core/math.rs  
   
rust    
//\! Math subsystem — re-exports glam \+ common helpers  
pub use glam::\*;

/// 3D transform: position, rotation, scale  
\#\[derive(Debug, Clone, Copy, PartialEq, Component)\]  
pub struct Transform {  
    pub position: Vec3,  
    pub rotation: Quat,  
    pub scale: Vec3,  
}

impl Default for Transform {  
    fn default() \-\> Self {  
        Self {  
            position: Vec3::ZERO,  
            rotation: Quat::IDENTITY,  
            scale: Vec3::ONE,  
        }  
    }  
}

impl Transform {  
    /// Create new transform with position  
    pub fn from\_position(position: Vec3) \-\> Self {  
        Self { position, ..Default::default() }  
    }

    /// Convert to 4x4 matrix  
    pub fn to\_matrix(\&self) \-\> Mat4 {  
        Mat4::from\_scale\_rotation\_translation(self.scale, self.rotation, self.position)  
    }

    /// Forward vector (-Z axis, right-handed)  
    pub fn forward(\&self) \-\> Vec3 { self.rotation \* Vec3::NEG\_Z }  
    /// Right vector (+X axis)  
    pub fn right(\&self) \-\> Vec3 { self.rotation \* Vec3::X }  
    /// Up vector (+Y axis)  
    pub fn up(\&self) \-\> Vec3 { self.rotation \* Vec3::Y }

    /// Look at target position  
    pub fn look\_at(\&mut self, target: Vec3, up: Vec3) {  
        self.rotation \= Quat::from\_mat4(\&Mat4::look\_at\_rh(self.position, target, up)).inverse();  
    }  
}

/// Common math utilities  
pub mod math\_utils {  
    use super::\*;

    /// Linear interpolation between two values  
    pub fn lerp(a: f32, b: f32, t: f32) \-\> f32 { a \+ (b \- a) \* t.clamp(0.0, 1.0) }  
      
    /// Clamp a value between min and max  
    pub fn clamp\<T: PartialOrd\>(value: T, min: T, max: T) \-\> T {  
        if value \< min { min } else if value \> max { max } else { value }  
    }

    /// Convert degrees to radians  
    pub fn deg2rad(deg: f32) \-\> f32 { deg \* std::f32::consts::PI / 180.0 }  
    /// Convert radians to degrees  
    pub fn rad2deg(rad: f32) \-\> f32 { rad \* 180.0 / std::f32::consts::PI }  
}  
   
   
2.5 src/core/config.rs  
   
rust    
//\! Engine configuration  
use serde::{Serialize, Deserialize};

\#\[derive(Debug, Clone, Serialize, Deserialize)\]  
pub struct EngineConfig {  
    pub window: WindowConfig,  
    pub log\_level: String,  
    pub fixed\_timestep: f32,  
}

\#\[derive(Debug, Clone, Serialize, Deserialize)\]  
pub struct WindowConfig {  
    pub title: String,  
    pub width: u32,  
    pub height: u32,  
    pub fullscreen: bool,  
    pub resizable: bool,  
}

impl Default for EngineConfig {  
    fn default() \-\> Self {  
        Self {  
            window: WindowConfig {  
                title: "RustyRacer Engine".into(),  
                width: 1280,  
                height: 720,  
                fullscreen: false,  
                resizable: true,  
            },  
            log\_level: "info".into(),  
            fixed\_timestep: 1.0 / 60.0,  
        }  
    }  
}

impl EngineConfig {  
    /// Load config from JSON file  
    pub fn load\_from\_file(path: \&str) \-\> EngineResult\<Self\> {  
        let file \= std::fs::read\_to\_string(path)?;  
        let config \= serde\_json::from\_str(\&file)?;  
        Ok(config)  
    }

    /// Save config to JSON file  
    pub fn save\_to\_file(\&self, path: \&str) \-\> EngineResult\<()\> {  
        let json \= serde\_json::to\_string\_pretty(self)?;  
        std::fs::write(path, json)?;  
        Ok(())  
    }  
}  
   
   
   
   
⚙️ Step 3 — Custom ECS Implementation  
   
This is the most critical part of Phase 1 — we will build our own ECS from scratch, no external crates.  
   
3.1 src/ecs/mod.rs  
   
rust    
//\! Custom lightweight Entity Component System implementation

pub mod entity;  
pub mod component;  
pub mod world;  
pub mod system;  
pub mod scheduler;  
pub mod query;  
pub mod storage;

// Re-export core ECS types  
pub use entity::Entity;  
pub use component::{Component, ComponentStorage};  
pub use world::World;  
pub use system::System;  
pub use scheduler::Scheduler;  
pub use query::Query;  
   
   
3.2 src/ecs/entity.rs  
   
rust    
//\! Entity type — unique identifier for game objects  
use std::fmt;  
use std::hash::Hash;

/// Unique entity ID — lightweight identifier only  
\#\[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)\]  
pub struct Entity(pub u64);

impl Entity {  
    /// Create new entity from raw ID  
    pub fn new(id: u64) \-\> Self { Self(id) }  
      
    /// Get raw numeric ID  
    pub fn id(\&self) \-\> u64 { self.0 }  
}

impl fmt::Display for Entity {  
    fn fmt(\&self, f: \&mut fmt::Formatter\<'\_\>) \-\> fmt::Result {  
        write\!(f, "Entity({})", self.0)  
    }  
}

/// Iterator for generating new entity IDs  
\#\[derive(Debug, Default)\]  
pub struct EntityGenerator {  
    next\_id: u64,  
}

impl EntityGenerator {  
    /// Create new generator starting from 0  
    pub fn new() \-\> Self { Self { next\_id: 0 } }

    /// Generate next unique entity  
    pub fn generate(\&mut self) \-\> Entity {  
        let id \= self.next\_id;  
        self.next\_id \+= 1;  
        Entity(id)  
    }  
}  
   
   
3.3 src/ecs/component.rs  
   
rust    
//\! Component trait and storage definitions  
use super::entity::Entity;  
use std::any::Any;  
use std::fmt::Debug;

/// Base trait for all components — defines requirements  
pub trait Component: Clone \+ Send \+ Sync \+ 'static \+ Debug {}

/// Auto-implement Component for all eligible types  
impl\<T: Clone \+ Send \+ Sync \+ 'static \+ Debug\> Component for T {}

/// Trait for component storage — type-erased interface  
pub trait ComponentStorage: Any \+ Send \+ Sync {  
    /// Remove component for given entity  
    fn remove(\&mut self, entity: Entity);  
    /// Check if entity has this component  
    fn has(\&self, entity: Entity) \-\> bool;  
    /// Get number of stored components  
    fn len(\&self) \-\> usize;  
    /// Whether storage is empty  
    fn is\_empty(\&self) \-\> bool { self.len() \== 0 }  
    /// Convert to any for type casting  
    fn as\_any(\&self) \-\> \&dyn Any;  
    /// Convert to mutable any for type casting  
    fn as\_any\_mut(\&mut self) \-\> \&mut dyn Any;  
}

/// Sparse set storage — efficient, cache-friendly component storage  
\#\[derive(Debug, Clone)\]  
pub struct SparseStorage\<T: Component\> {  
    /// Maps entity ID → index in dense/data  
    sparse: Vec\<Option\<usize\>\>,  
    /// Stores entities in iteration order  
    dense: Vec\<Entity\>,  
    /// Stores component data matching dense array  
    data: Vec\<T\>,  
}

impl\<T: Component\> Default for SparseStorage\<T\> {  
    fn default() \-\> Self {  
        Self {  
            sparse: Vec::new(),  
            dense: Vec::new(),  
            data: Vec::new(),  
        }  
    }  
}

impl\<T: Component\> SparseStorage\<T\> {  
    /// Create empty storage  
    pub fn new() \-\> Self { Self::default() }

    /// Add component for entity  
    pub fn insert(\&mut self, entity: Entity, component: T) {  
        let id \= entity.id() as usize;  
          
        // Resize sparse array if needed  
        if id \>= self.sparse.len() {  
            self.sparse.resize(id \+ 1, None);  
        }

        // Only add if not already present  
        if self.sparse\[id\].is\_none() {  
            let index \= self.dense.len();  
            self.sparse\[id\] \= Some(index);  
            self.dense.push(entity);  
            self.data.push(component);  
        } else {  
            // Update existing component  
            let index \= self.sparse\[id\].unwrap();  
            self.data\[index\] \= component;  
        }  
    }

    /// Get reference to component  
    pub fn get(\&self, entity: Entity) \-\> Option\<\&T\> {  
        let id \= entity.id() as usize;  
        self.sparse.get(id)?.map(|idx| \&self.data\[\*idx\])  
    }

    /// Get mutable reference to component  
    pub fn get\_mut(\&mut self, entity: Entity) \-\> Option\<\&mut T\> {  
        let id \= entity.id() as usize;  
        self.sparse.get(id)?.and\_then(|idx| self.data.get\_mut(\*idx))  
    }

    /// Remove component for entity  
    pub fn remove(\&mut self, entity: Entity) \-\> Option\<T\> {  
        let id \= entity.id() as usize;  
        let index \= self.sparse.get(id)?.copied()?;

        // Swap with last element for fast removal  
        let last\_index \= self.dense.len() \- 1;  
        let removed\_data \= self.data.swap\_remove(index);  
        let swapped\_entity \= self.dense.swap\_remove(index);

        // Update sparse map for swapped entity  
        if index \!= last\_index {  
            let swapped\_id \= swapped\_entity.id() as usize;  
            self.sparse\[swapped\_id\] \= Some(index);  
        }

        // Clear sparse entry for removed entity  
        self.sparse\[id\] \= None;

        Some(removed\_data)  
    }

    /// Check if entity has component  
    pub fn contains(\&self, entity: Entity) \-\> bool {  
        let id \= entity.id() as usize;  
        self.sparse.get(id).copied().flatten().is\_some()  
    }

    /// Iterate over all (entity, component) pairs  
    pub fn iter(\&self) \-\> impl Iterator\<Item \= (Entity, \&T)\> {  
        self.dense.iter().copied().zip(self.data.iter())  
    }

    /// Mutable iterator over all pairs  
    pub fn iter\_mut(\&mut self) \-\> impl Iterator\<Item \= (Entity, \&mut T)\> {  
        self.dense.iter().copied().zip(self.data.iter\_mut())  
    }  
}

// Implement ComponentStorage trait for SparseStorage  
impl\<T: Component\> ComponentStorage for SparseStorage\<T\> {  
    fn remove(\&mut self, entity: Entity) {  
        self.remove(entity);  
    }

    fn has(\&self, entity: Entity) \-\> bool {  
        self.contains(entity)  
    }

    fn len(\&self) \-\> usize {  
        self.data.len()  
    }

    fn as\_any(\&self) \-\> \&dyn Any { self }  
    fn as\_any\_mut(\&mut self) \-\> \&mut dyn Any { self }  
}  
   
   
3.4 src/ecs/world.rs  
   
rust    
//\! World — main container for all entities, components and resources  
use super::entity::{Entity, EntityGenerator};  
use super::component::{Component, ComponentStorage, SparseStorage};  
use std::any::Any;  
use std::collections::HashMap;  
use std::sync::Arc;

/// Global shared data accessible to all systems  
pub trait Resource: Send \+ Sync \+ 'static \+ Any \+ Clone {}  
impl\<T: Send \+ Sync \+ 'static \+ Any \+ Clone\> Resource for T {}

/// World holds the complete game state  
\#\[derive(Debug, Clone)\]  
pub struct World {  
    /// Generates unique entity IDs  
    entity\_gen: EntityGenerator,  
    /// Stores all component storages, mapped by component type ID  
    components: HashMap\<std::any::TypeId, Box\<dyn ComponentStorage\>\>,  
    /// Global shared resources  
    resources: HashMap\<std::any::TypeId, Box\<dyn Any \+ Send \+ Sync\>\>,  
}

impl Default for World {  
    fn default() \-\> Self {  
        Self::new()  
    }  
}

impl World {  
    /// Create empty world  
    pub fn new() \-\> Self {  
        log::debug\!("Creating new World instance");  
        Self {  
            entity\_gen: EntityGenerator::new(),  
            components: HashMap::new(),  
            resources: HashMap::new(),  
        }  
    }

    /// Create new empty entity  
    pub fn create\_entity(\&mut self) \-\> Entity {  
        let entity \= self.entity\_gen.generate();  
        log::trace\!("Created new entity: {}", entity);  
        entity  
    }

    /// Destroy entity and all its components  
    pub fn destroy\_entity(\&mut self, entity: Entity) {  
        log::trace\!("Destroying entity: {}", entity);  
        // Remove from all component storages  
        for storage in self.components.values\_mut() {  
            storage.remove(entity);  
        }  
    }

    /// Register a component type (automatically called when first added)  
    pub fn register\_component\<T: Component\>(\&mut self) {  
        let type\_id \= std::any::TypeId::of::\<T\>();  
        if \!self.components.contains\_key(\&type\_id) {  
            log::debug\!("Registering component type: {}", std::any::type\_name::\<T\>());  
            self.components.insert(type\_id, Box::new(SparseStorage::\<T\>::new()));  
        }  
    }

    /// Add component to entity  
    pub fn add\_component\<T: Component\>(\&mut self, entity: Entity, component: T) {  
        self.register\_component::\<T\>();  
        let storage \= self.components.get\_mut(\&std::any::TypeId::of::\<T\>())  
            .expect("Component should be registered");  
          
        let storage \= storage.as\_any\_mut().downcast\_mut::\<SparseStorage\<T\>\>()  
            .expect("Storage type mismatch");  
          
        storage.insert(entity, component);  
    }

    /// Get reference to component for entity  
    pub fn get\_component\<T: Component\>(\&self, entity: Entity) \-\> Option\<\&T\> {  
        let storage \= self.components.get(\&std::any::TypeId::of::\<T\>())?;  
        let storage \= storage.as\_any().downcast\_ref::\<SparseStorage\<T\>\>()?;  
        storage.get(entity)  
    }

    /// Get mutable reference to component  
    pub fn get\_component\_mut\<T: Component\>(\&mut self, entity: Entity) \-\> Option\<\&mut T\> {  
        let storage \= self.components.get\_mut(\&std::any::TypeId::of::\<T\>())?;  
        let storage \= storage.as\_any\_mut().downcast\_mut::\<SparseStorage\<T\>\>()?;  
        storage.get\_mut(entity)  
    }

    /// Check if entity has component  
    pub fn has\_component\<T: Component\>(\&self, entity: Entity) \-\> bool {  
        self.components.get(\&std::any::TypeId::of::\<T\>())  
            .map(|s| s.has(entity))  
            .unwrap\_or(false)  
    }

    /// Add global resource  
    pub fn add\_resource\<R: Resource\>(\&mut self, resource: R) {  
        log::debug\!("Adding resource: {}", std::any::type\_name::\<R\>());  
        self.resources.insert(std::any::TypeId::of::\<R\>(), Box::new(resource));  
    }

    /// Get reference to resource  
    pub fn get\_resource\<R: Resource\>(\&self) \-\> Option\<\&R\> {  
        self.resources.get(\&std::any::TypeId::of::\<R\>())  
            .and\_then(|r| r.downcast\_ref::\<R\>())  
    }

    /// Get mutable reference to resource  
    pub fn get\_resource\_mut\<R: Resource\>(\&mut self) \-\> Option\<\&mut R\> {  
        self.resources.get\_mut(\&std::any::TypeId::of::\<R\>())  
            .and\_then(|r| r.downcast\_mut::\<R\>())  
    }

    /// Iterate over all entities with a specific component  
    pub fn iter\_components\<T: Component\>(\&self) \-\> impl Iterator\<Item \= (Entity, \&T)\> {  
        self.components.get(\&std::any::TypeId::of::\<T\>())  
            .map(|s| {  
                let storage \= s.as\_any().downcast\_ref::\<SparseStorage\<T\>\>().unwrap();  
                Box::new(storage.iter()) as Box\<dyn Iterator\<Item \= (Entity, \&T)\>\>  
            })  
            .unwrap\_or\_else(|| Box::new(std::iter::empty()))  
    }  
}  
   
   
3.5 src/ecs/query.rs  
   
rust    
//\! Query system — filter and access entities with required components  
use super::entity::Entity;  
use super::component::Component;  
use super::world::World;

/// Query to access entities with specific component combinations  
pub struct Query\<'w, T: Queryable\> {  
    world: &'w World,  
    \_marker: std::marker::PhantomData\<T\>,  
}

/// Trait for types that can be used as query parameters  
pub trait Queryable {  
    type Item\<'a\>;  
    fn fetch(world: \&World, entity: Entity) \-\> Option\<Self::Item\<'\_\>\>;  
}

// Implement Queryable for single component  
impl\<'a, T: Component\> Queryable for &'a T {  
    type Item \= &'a T;  
    fn fetch(world: \&World, entity: Entity) \-\> Option\<Self::Item\<'\_\>\> {  
        world.get\_component(entity)  
    }  
}

impl\<'a, T: Component\> Queryable for &'a mut T {  
    type Item \= &'a mut T;  
    fn fetch(world: \&World, entity: Entity) \-\> Option\<Self::Item\<'\_\>\> {  
        // Safe because we only get one mutable reference at a time  
        unsafe {  
            let world\_ptr \= world as \*const World as \*mut World;  
            (\*world\_ptr).get\_component\_mut(entity)  
        }  
    }  
}

// Implement Queryable for tuples of components (up to 4\)  
macro\_rules\! impl\_queryable\_tuple {  
    ($($T:ident),+) \=\> {  
        impl\<$($T: Component),+\> Queryable for ($(&$T),+) {  
            type Item\<'a\> \= ($(&'a $T),+);  
            fn fetch(world: \&World, entity: Entity) \-\> Option\<Self::Item\<'\_\>\> {  
                Some(($(world.get\_component::\<$T\>(entity)?),+))  
            }  
        }

        impl\<$($T: Component),+\> Queryable for ($(\&mut $T),+) {  
            type Item\<'a\> \= ($(&'a mut $T),+);  
            fn fetch(world: \&World, entity: Entity) \-\> Option\<Self::Item\<'\_\>\> {  
                unsafe {  
                    let world\_ptr \= world as \*const World as \*mut World;  
                    Some(($((\*world\_ptr).get\_component\_mut::\<$T\>(entity)?),+))  
                }  
            }  
        }  
    };  
}

impl\_queryable\_tuple\!(A, B);  
impl\_queryable\_tuple\!(A, B, C);  
impl\_queryable\_tuple\!(A, B, C, D);

impl\<'w, T: Queryable\> Query\<'w, T\> {  
    /// Create new query from world  
    pub fn new(world: &'w World) \-\> Self {  
        Self { world, \_marker: std::marker::PhantomData }  
    }

    /// Iterate over all matching entities and components  
    pub fn iter(\&self) \-\> impl Iterator\<Item \= T::Item\<'\_\>\> \+ 'w {  
        // Use first component in query to iterate  
        let type\_id \= std::any::TypeId::of::\<\<\<T as Queryable\>::Item\<'static\> as std::ops::Deref\>::Target\>();  
          
        self.world.components.get(\&type\_id)  
            .map(|storage| {  
                let entities \= storage.as\_any().downcast\_ref::\<super::component::SparseStorage\<()\>\>()  
                    .unwrap()  
                    .dense  
                    .iter()  
                    .copied();  
                  
                Box::new(entities.filter\_map(|e| T::fetch(self.world, e))) as Box\<dyn Iterator\<Item \= T::Item\<'\_\>\>\>  
            })  
            .unwrap\_or\_else(|| Box::new(std::iter::empty()))  
    }  
}  
   
   
3.6 src/ecs/system.rs  
   
rust    
//\! System trait — logic that operates on world state  
use super::world::World;

/// Base trait for all game logic systems  
pub trait System: Send \+ Sync \+ 'static {  
    /// Run system logic — called every frame/update  
    fn run(\&mut self, world: \&mut World);

    /// Optional: initialize system when added to scheduler  
    fn initialize(\&mut self, \_world: \&mut World) {}

    /// Optional: cleanup when system is removed  
    fn cleanup(\&mut self, \_world: \&mut World) {}  
}

/// Convenience struct to create systems from closures  
pub struct FunctionSystem\<F: FnMut(\&mut World) \+ Send \+ Sync \+ 'static\> {  
    func: F,  
}

impl\<F: FnMut(\&mut World) \+ Send \+ Sync \+ 'static\> System for FunctionSystem\<F\> {  
    fn run(\&mut self, world: \&mut World) {  
        (self.func)(world);  
    }  
}

/// Helper to create system from function/closure  
pub fn system\<F: FnMut(\&mut World) \+ Send \+ Sync \+ 'static\>(func: F) \-\> impl System {  
    FunctionSystem { func }  
}  
   
   
3.7 src/ecs/scheduler.rs  
   
rust    
//\! Scheduler — manages execution order of systems  
use super::world::World;  
use super::system::System;

/// Controls when systems run  
\#\[derive(Debug, Clone, Copy, PartialEq, Eq)\]  
pub enum ExecutionStage {  
    /// Run before input processing  
    PreUpdate,  
    /// Run input processing  
    Input,  
    /// Run game logic updates  
    Update,  
    /// Run physics simulation  
    Physics,  
    /// Run after logic/physics, before rendering  
    PostUpdate,  
    /// Run rendering  
    Render,  
}

/// Scheduler holds and executes systems in order  
\#\[derive(Debug, Default)\]  
pub struct Scheduler {  
    systems: Vec\<Box\<dyn System\>\>,  
}

impl Scheduler {  
    /// Create empty scheduler  
    pub fn new() \-\> Self { Self::default() }

    /// Add system to execution queue  
    pub fn add\_system\<S: System\>(\&mut self, system: S) {  
        log::debug\!("Adding system: {}", std::any::type\_name::\<S\>());  
        self.systems.push(Box::new(system));  
    }

    /// Initialize all systems  
    pub fn initialize(\&mut self, world: \&mut World) {  
        log::info\!("Initializing scheduler systems");  
        for sys in \&mut self.systems {  
            sys.initialize(world);  
        }  
    }

    /// Run all systems in order  
    pub fn run(\&mut self, world: \&mut World) {  
        for sys in \&mut self.systems {  
            sys.run(world);  
        }  
    }

    /// Cleanup all systems  
    pub fn cleanup(\&mut self, world: \&mut World) {  
        log::info\!("Cleaning up scheduler systems");  
        for sys in \&mut self.systems {  
            sys.cleanup(world);  
        }  
    }  
}  
   
   
   
   
📦 Step 4 — Asset Manager Base  
   
Simple skeleton for loading and referencing assets.  
   
4.1 src/assets/mod.rs  
   
rust    
//\! Asset management system — loading, caching and referencing game assets

pub mod handle;  
pub mod manager;  
pub mod loader;

pub use handle::AssetHandle;  
pub use manager::AssetManager;  
   
   
4.2 src/assets/handle.rs  
   
rust    
//\! Typed asset handles — safe references to loaded assets  
use std::marker::PhantomData;  
use std::fmt::Debug;

/// Unique typed handle to an asset  
\#\[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)\]  
pub struct AssetHandle\<T: Debug \+ 'static\> {  
    id: u64,  
    \_marker: PhantomData\<T\>,  
}

impl\<T: Debug \+ 'static\> AssetHandle\<T\> {  
    /// Create new handle from raw ID  
    pub fn new(id: u64) \-\> Self {  
        Self { id, \_marker: PhantomData }  
    }

    /// Get raw numeric ID  
    pub fn id(\&self) \-\> u64 { self.id }  
}  
   
   
4.3 src/assets/manager.rs  
   
rust    
//\! Central asset storage and management  
use super::handle::AssetHandle;  
use crate::core::EngineResult;  
use std::any::Any;  
use std::collections::HashMap;  
use std::fmt::Debug;

/// Trait for all asset types  
pub trait Asset: Clone \+ Debug \+ Send \+ Sync \+ 'static {}  
impl\<T: Clone \+ Debug \+ Send \+ Sync \+ 'static\> Asset for T {}

/// Central manager for all loaded assets  
\#\[derive(Debug, Default)\]  
pub struct AssetManager {  
    assets: HashMap\<u64, Box\<dyn Any \+ Send \+ Sync\>\>,  
    path\_map: HashMap\<String, u64\>,  
    next\_id: u64,  
}

impl AssetManager {  
    /// Create empty asset manager  
    pub fn new() \-\> Self { Self::default() }

    /// Load asset from file path — implementation deferred to Phase 2  
    pub fn load\<T: Asset\>(\&mut self, path: \&str) \-\> EngineResult\<AssetHandle\<T\>\> {  
        log::debug\!("Loading asset: {}", path);  
          
        // Check if already loaded  
        if let Some(\&id) \= self.path\_map.get(path) {  
            return Ok(AssetHandle::new(id));  
        }

        // Placeholder — actual loading logic comes later  
        let id \= self.next\_id;  
        self.next\_id \+= 1;

        // Create empty placeholder asset  
        let asset: T \= unsafe { std::mem::zeroed() }; // Replace with real loading later  
          
        self.assets.insert(id, Box::new(asset));  
        self.path\_map.insert(path.to\_string(), id);

        Ok(AssetHandle::new(id))  
    }

    /// Get reference to loaded asset  
    pub fn get\<T: Asset\>(\&self, handle: AssetHandle\<T\>) \-\> Option\<\&T\> {  
        self.assets.get(\&handle.id()).and\_then(|a| a.downcast\_ref::\<T\>())  
    }

    /// Get mutable reference to loaded asset  
    pub fn get\_mut\<T: Asset\>(\&mut self, handle: AssetHandle\<T\>) \-\> Option\<\&mut T\> {  
        self.assets.get\_mut(\&handle.id()).and\_then(|a| a.downcast\_mut::\<T\>())  
    }  
}  
   
   
   
   
🪟 Step 5 — Platform Layer: Window & Input  
   
Basic window creation and event handling using  winit .  
   
5.1 src/platform/mod.rs  
   
rust    
//\! Platform abstraction layer — window creation, events and input

pub mod window;  
pub mod events;  
pub mod input;

pub use window::Window;  
pub use events::EventLoop;  
pub use input::{InputState, InputSystem};  
   
   
5.2 src/platform/window.rs  
   
rust    
//\! Window creation and management  
use crate::core::{EngineResult, EngineError, WindowConfig};  
use winit::event\_loop::EventLoop;  
use winit::window::{WindowAttributes, WindowBuilder};

/// Winit window wrapper  
\#\[derive(Debug)\]  
pub struct Window {  
    inner: winit::window::Window,  
    config: WindowConfig,  
}

impl Window {  
    /// Create new window from configuration  
    pub fn new(config: WindowConfig, event\_loop: \&EventLoop\<()\>) \-\> EngineResult\<Self\> {  
        log::info\!("Creating window: {} ({}x{})", config.title, config.width, config.height);

        let attributes \= WindowAttributes::default()  
            .with\_title(config.title.clone())  
            .with\_inner\_size(winit::dpi::LogicalSize::new(config.width, config.height))  
            .with\_resizable(config.resizable);

        let inner \= WindowBuilder::new()  
            .build(event\_loop)  
            .map\_err(EngineError::PlatformError)?;

        Ok(Self { inner, config })  
    }

    /// Get window size in pixels  
    pub fn size(\&self) \-\> (u32, u32) {  
        let size \= self.inner.inner\_size();  
        (size.width, size.height)  
    }

    /// Get window title  
    pub fn title(\&self) \-\> \&str { \&self.config.title }

    /// Request window close  
    pub fn request\_close(\&self) { self.inner.request\_close() }

    /// Get underlying winit window  
    pub fn inner(\&self) \-\> \&winit::window::Window { \&self.inner }  
}  
   
   
5.3 src/platform/input.rs  
   
rust    
//\! Input system — track keyboard, mouse and game state  
use crate::ecs::{System, World};  
use winit::event::{ElementState, VirtualKeyCode, MouseButton};  
use std::collections::HashSet;

/// Current input state  
\#\[derive(Debug, Clone, Default)\]  
pub struct InputState {  
    /// Keys currently held down  
    pressed\_keys: HashSet\<VirtualKeyCode\>,  
    /// Keys pressed this frame  
    just\_pressed\_keys: HashSet\<VirtualKeyCode\>,  
    /// Keys released this frame  
    just\_released\_keys: HashSet\<VirtualKeyCode\>,  
    /// Mouse buttons currently held  
    pressed\_mouse: HashSet\<MouseButton\>,  
    /// Mouse position in pixels  
    mouse\_position: (f32, f32),  
    /// Mouse movement delta this frame  
    mouse\_delta: (f32, f32),  
}

impl InputState {  
    /// Check if key is currently pressed  
    pub fn is\_key\_pressed(\&self, key: VirtualKeyCode) \-\> bool {  
        self.pressed\_keys.contains(\&key)  
    }

    /// Check if key was pressed this exact frame  
    pub fn is\_key\_just\_pressed(\&self, key: VirtualKeyCode) \-\> bool {  
        self.just\_pressed\_keys.contains(\&key)  
    }

    /// Check if key was released this exact frame  
    pub fn is\_key\_just\_released(\&self, key: VirtualKeyCode) \-\> bool {  
        self.just\_released\_keys.contains(\&key)  
    }

    /// Get current mouse position  
    pub fn mouse\_position(\&self) \-\> (f32, f32) { self.mouse\_position }

    /// Get mouse movement since last frame  
    pub fn mouse\_delta(\&self) \-\> (f32, f32) { self.mouse\_delta }

    /// Clear one-frame state — call at end of each frame  
    pub fn clear\_frame\_state(\&mut self) {  
        self.just\_pressed\_keys.clear();  
        self.just\_released\_keys.clear();  
        self.mouse\_delta \= (0.0, 0.0);  
    }

    /// Process keyboard event  
    pub fn process\_key\_event(\&mut self, key: VirtualKeyCode, state: ElementState) {  
        match state {  
            ElementState::Pressed \=\> {  
                if \!self.pressed\_keys.contains(\&key) {  
                    self.just\_pressed\_keys.insert(key);  
                }  
                self.pressed\_keys.insert(key);  
            }  
            ElementState::Released \=\> {  
                self.pressed\_keys.remove(\&key);  
                self.just\_released\_keys.insert(key);  
            }  
        }  
    }

    /// Process mouse movement  
    pub fn process\_mouse\_motion(\&mut self, x: f32, y: f32, delta\_x: f32, delta\_y: f32) {  
        self.mouse\_position \= (x, y);  
        self.mouse\_delta \= (self.mouse\_delta.0 \+ delta\_x, self.mouse\_delta.1 \+ delta\_y);  
    }  
}

/// System that manages input state  
\#\[derive(Debug, Default)\]  
pub struct InputSystem;

impl System for InputSystem {  
    fn run(\&mut self, world: \&mut World) {  
        // Clear previous frame's transient state  
        if let Some(input) \= world.get\_resource\_mut::\<InputState\>() {  
            input.clear\_frame\_state();  
        }

        // Input events will be populated from main event loop in actual app  
    }

    fn initialize(\&mut self, world: \&mut World) {  
        // Add input state as global resource  
        world.add\_resource(InputState::default());  
    }  
}  
   
   
   
   
🛠️ Step 6 — Core Utilities  
   
6.1 src/utils/mod.rs  
   
rust    
//\! General utility functions and helpers

pub mod logging;  
pub mod macros;  
   
   
6.2 src/utils/logging.rs  
   
rust    
//\! Logging initialization  
use crate::core::EngineConfig;  
use env\_logger::Env;

/// Initialize logging system based on config  
pub fn init\_logging(config: \&EngineConfig) {  
    let env \= Env::default().default\_filter\_or(\&config.log\_level);  
    env\_logger::Builder::from\_env(env)  
        .format\_timestamp\_millis()  
        .format\_module\_path(true)  
        .init();  
      
    log::info\!("Logging initialized — level: {}", config.log\_level);  
}  
   
   
   
   
🧪 Step 7 — Test Your Implementation  
   
Create  examples/basic\_window.rs  to verify everything works:  
   
rust    
use rustyracer\_engine::prelude::\*;  
use winit::event::{Event, WindowEvent};

fn main() \-\> EngineResult\<()\> {  
    // Initialize  
    let config \= EngineConfig::default();  
    utils::logging::init\_logging(\&config);

    // Create core systems  
    let mut world \= World::new();  
    let mut scheduler \= Scheduler::new();

    // Create window and event loop  
    let event\_loop \= EventLoop::new();  
    let mut window \= Window::new(config.window, \&event\_loop)?;

    // Add systems  
    scheduler.add\_system(InputSystem::default());

    // Initialize  
    scheduler.initialize(\&mut world);

    // Main loop  
    log::info\!("Starting main loop");  
    event\_loop.run(move |event, \_, control\_flow| {  
        control\_flow.set\_poll();

        match event {  
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } \=\> {  
                log::info\!("Close requested");  
                control\_flow.set\_exit();  
            }

            Event::MainEventsCleared \=\> {  
                // Run game systems  
                scheduler.run(\&mut world);  
                window.inner().request\_redraw();  
            }

            \_ \=\> {}  
        }  
    })  
}  
   
   
Run with:  
   
bash    
cargo run \--example basic\_window  
   
   
   
   
✅ Phase 1 Completion Checklist  
   
Project structure matches the defined layout  
Cargo.toml configured with all required dependencies  
Custom ECS working: create entities, add/get/remove components  
World and Scheduler functional  
 glam  math types integrated and working  
Transform struct implemented  
Asset manager skeleton compiles  
Window creates successfully using  winit   
Input system tracks key presses and mouse movement  
Logging and error handling working  
   
   
   
