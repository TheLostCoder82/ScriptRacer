---

# Master Design Document (MDD): "IronMouse Engine"

## 1\. Project Overview

* Objective: Develop a custom 3D game engine in Rust to learn systems programming and engine architecture.  
* Target Platform: Native Linux (Ubuntu 24.04), extendable to Windows/macOS.  
* Hardware Baseline: Intel HD 620 (Integrated Graphics), 7th Gen i7, 16GB RAM.  
* Philosophy: "Safety first, Performance second, Flexibility third." Use Rust's safety guarantees to prevent crashes, optimize only when profiling dictates.

## 2\. High-Level Architecture

The engine follows a Loop-Based Architecture with a clear separation between Simulation and Rendering.

* The Kernel: Manages the main loop, timing, and subsystem lifecycle.  
* The Simulation World: Contains the ECS, Physics, and Game Logic. Runs on a Fixed Timestep (e.g., 60Hz).  
* The Rendering World: Contains GPU resources, Shaders, and Draw Calls. Runs on a Variable Timestep (Monitor Refresh Rate).  
* The Interface Layer: Wraps external libraries (winit, kira, rapier) to decouple them from core logic.

## 3\. Data Flow

1. Input: winit events → Input System → ECS Resources.  
2. Update: ECS Systems read Input → Update Transforms/Velocity → Physics Step.  
3. Render: Render System reads Transforms → Interpolates (if needed) → Encodes wgpu Commands → Submit to GPU.  
4. Audio: Audio System streams sounds based on ECS events (triggers).

## 4\. Tech Stack Summary

| System | Library | Rationale |
| :---- | :---- | :---- |
| Language | Rust (Stable) | Memory safety, modern tooling. |
| Graphics | wgpu | Vulkan/Metal/DX12 abstraction. Lower boilerplate than raw Vulkan. |
| Windowing | winit | Cross-platform window/input handling. |
| Math | glam | SIMD-optimized linear algebra (standard in Rust gamedev). |
| Physics | rapier3d | Pure Rust, high performance, easy ECS integration. |
| Audio | kira | Game-focused, supports spatial audio. |
| Logging | tracing | Structured logging for debugging performance. |
| Serialization | serde \+ ron | Human-readable config files. |

---

# Technical Design Documents (TDD)

## TDD 01: Custom ECS (Entity Component System)

Goal: Learn data-oriented design without excessive macro magic.

* Architecture: Archetypal Storage (Group entities by component composition).  
* Memory Strategy: Struct of Arrays (SoA) within each Archetype.  
  * *Why:* Better cache locality during system iteration than Array of Structures (AoS).  
  * *Rust Implementation:* Use Vec\<T\> for each component type within an Archetype struct. Avoid raw pointers initially; use indices.  
* Entities:  
  * Represented by a u32 ID and a u32 Generation (to prevent ABA problems).  
  * Mapping: EntityID → ArchetypeID \+ RowIndex.  
* Systems:  
  * Functions that borrow specific component storages mutably or immutably.  
  * Borrow Checking: Rust's borrow checker will prevent systems from accessing the same data mutably simultaneously. This is a feature, not a bug.  
* Key Rust Concepts:  
  * struct Archetype { components: HashMap\<ComponentId, Box\<dyn Any\>\> } (Simplified start).  
  * Use std::mem::swap for efficient component removal.

## TDD 02: Rendering System (wgpu)

Goal: Abstract Vulkan complexity while maintaining control.

* Initialization Flow:  
  1. Create wgpu::Instance.  
  2. Request Adapter (matches hardware, e.g., Intel HD 620).  
  3. Create Device and Queue.  
  4. Configure Surface (from winit window).  
* Resource Management:  
  1. Buffers: Uniform Buffers (Per Frame), Vertex Buffers (Per Mesh), Index Buffers (Per Mesh).  
  2. Textures: Use wgpu::TextureView for rendering targets.  
  3. Lifetime: Resources are tied to the Device. Use Rust's Drop trait to automatically clean up GPU resources when Rust structs go out of scope.  
* Pipeline:  
  1. Use wgpu::RenderPipeline for shader states.  
  2. Shaders: Write in WGSL (WebGPU Shading Language). It is similar to GLSL/HLSL but stricter.  
* Frame Loop:  
  1. surface.get\_current\_texture().  
  2. Encode commands into CommandEncoder.  
  3. queue.submit().  
  4. texture.present().

## TDD 03: Input System

Goal: Decouple hardware events from game logic.

* Integration: winit event loop callbacks.  
* State Management:  
  * Do not process logic inside the event callback.  
  * Maintain a InputState resource (e.g., struct Input { keys: HashMap\<Key, bool\>, mouse: Vec2 }).  
  * Events update InputState; ECS Systems read InputState.  
* Frame Coordination:  
  * Clear InputState at the start of each frame.  
  * Accumulate events during the frame.  
  * Lock state before Simulation Step.

## TDD 04: Physics Integration (Rapier)

Goal: Deterministic simulation without rewriting physics code.

* Strategy: Rapier runs as a specific System within the ECS.  
* Data Sync:  
  * Pre-Step: ECS writes Transform → Rapier RigidBodyHandle.  
  * Post-Step: Rapier RigidBodyHandle → ECS Transform.  
* Ownership:  
  * The ECS owns the Transform.  
  * Rapier owns the Velocity and Collision data.  
  * *Note:* To avoid duplication, consider storing RigidBodyHandle in an ECS component, and only store Transform in ECS for rendering. Let Rapier be the source of truth for physics bodies.  
* Timestep: Must match the ECS Fixed Timestep (e.g., 1/60s).

## TDD 05: Asset Management

Goal: Prevent frame stutter during loading.

* Handle System:  
  * Assets are not stored directly in Components.  
  * Components store Handle\<Mesh\>, Handle\<Texture\>.  
  * Handle is a lightweight ID (usize \+ Generation).  
* Loading:  
  * Background thread loads file from disk.  
  * Parses data (e.g., GLTF).  
  * Uploads to GPU (requires wgpu Device access).  
  * Swaps into the Asset Store.  
* Rust Concurrency:  
  * Use std::sync::Arc for shared asset ownership.  
  * Use crossbeam\_channel or std::mpsc for communication between loading thread and main thread.

## TDD 06: Audio System (Kira)

Goal: Spatial audio without blocking the main thread.

* Architecture:  
  * Kira runs its own internal audio thread.  
  * Engine sends "Play Sound" commands via a channel.  
* Spatialization:  
  * Update Kira's Emitter positions every frame based on ECS Transform data.  
* Resources:  
  * Load sounds asynchronously (similar to Asset Management).

---

# Rust-Specific Implementation Notes (For C++ Developers)

Since you are coming from C++/Vulkan, these are the critical mental shifts for this engine:

1. Ownership vs. Raw Pointers:  
   * *C++:* You manually new and delete. You pass void\* or raw pointers around.  
   * *Rust:* You pass ownership. If a function takes Vec\<Mesh\>, it *owns* that vector now. If you want to lend it, you pass \&mut Vec\<Mesh\>.  
   * *Engine Impact:* You rarely need to worry about memory leaks. The Drop trait acts like a deterministic destructor.  
2. The Borrow Checker:  
   * *C++:* You can accidentally modify data while iterating it.  
   * *Rust:* The compiler will refuse to compile if two systems try to mutate the same ECS component simultaneously. This prevents data races at compile time.  
   * *Engine Impact:* You will fight the compiler initially. When you do, it means you have a potential bug. Use split\_at\_mut or architectural changes to resolve.  
3. Unsafe Blocks:  
   * *C++:* Everything is implicitly unsafe.  
   * *Rust:* You can use unsafe for raw pointer manipulation (like in your custom ECS), but it must be wrapped in a safe interface.  
   * *Engine Impact:* Keep unsafe blocks tiny. Isolate them in a memory.rs module.  
4. Enums and Pattern Matching:  
   * *C++:* You use enum class and switch.  
   * *Rust:* You use enum with data (e.g., enum Event { Move(Vec3), Jump }) and match.  
   * *Engine Impact:* Great for Input systems and State Machines.

