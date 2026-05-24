Here is the complete Phase 1 Implementation Guide broken down into a series of **highly specific, iterative prompts for GitHub Copilot**.  
Each prompt is designed to contextually guide the AI to generate modern, clean, production-ready Rust code that explicitly aligns with the architecture detailed in your specification document.

## **🛠️ Module 1: Project Architecture & Manifest**

### **Prompt 1.1: Workspace Cargo.toml Setup**

Plaintext  
Create a \`Cargo.toml\` manifest for a 3D game engine learning project named "rustyracer-engine".   
Use the 2021 edition. Configure a development feature called "dev" that enables "log/std" and "env\_logger". 

Include the following specific dependencies and features:  
\- glam (v0.28) with features: "std", "bytemuck", "serde"  
\- winit (v0.30) with features: "x11", "wayland", "serde"  
\- serde (v1.0) with "derive" feature enabled, and serde\_json (v1.0)  
\- log (v0.4) and env\_logger (v0.10)  
\- thiserror (v1.0) and anyhow (v1.0)  
\- hashbrown (v0.14) with "serde" feature  
\- once\_cell (v1.19)  
\- Add criterion (v0.5) with "html\_reports" as a development dependency.

### **Prompt 1.2: Root Library API (src/lib.rs)**

Plaintext  
Write the root \`src/lib.rs\` file for "rustyracer-engine".   
\- Enable \`\#\!\[warn(missing\_docs)\]\` and \`\#\!\[allow(dead\_code)\]\`.  
\- Declare and re-export the following public submodules: \`core\`, \`ecs\`, \`assets\`, \`platform\`, and \`utils\`.  
\- Implement a public \`prelude\` module that re-exports all items from:  
  \- \`crate::core::\*\`  
  \- \`crate::ecs::\*\`  
  \- \`crate::assets::\*\`  
  \- \`crate::platform::\*\`  
  \- \`glam::\*\`  
\- Explicitly alias \`winit::event::VirtualKeyCode\` to \`KeyCode\` within the prelude.

## **🧮 Module 2: Engine Core & Infrastructure**

### **Prompt 2.1: Central Error Handling Subsystem (src/core/error.rs)**

Plaintext  
Create the \`src/core/error.rs\` module utilizing the \`thiserror\` crate. Define a public enum named \`EngineError\` with the following variants and custom error string formatting:  
1\. \`InitError(String)\` \-\> "Initialization failed: {0}"  
2\. \`AssetError(String)\` \-\> "Asset load error: {0}"  
3\. \`InvalidOperation(String)\` \-\> "Invalid operation: {0}"  
4\. \`PlatformError(winit::error::OsError)\` via \`\#\[from\]\` auto-conversion \-\> "Platform error: {0}"  
5\. \`IoError(std::io::Error)\` via \`\#\[from\]\` \-\> "IO error: {0}"  
6\. \`SerdeError(serde\_json::Error)\` via \`\#\[from\]\` \-\> "Serialization error: {0}"

Provide a type alias \`pub type EngineResult\<T\> \= Result\<T, EngineError\>;\`. Ensure all types have standard documentation comments.

### **Prompt 2.2: Time Management System (src/core/time.rs)**

Plaintext  
Implement \`src/core/time.rs\` to handle engine delta time tracking and fixed updates.   
Create a struct \`Time\` containing: \`start\_time\` (Instant), \`last\_frame\_time\` (Instant), \`delta\_time\` (f32), \`fixed\_delta\_time\` (f32), \`accumulated\_time\` (f32), and \`frame\_count\` (u64).

Implement these public methods:  
\- \`new()\`: Initializes times with \`Instant::now()\`, sets \`fixed\_delta\_time\` to 1.0 / 60.0, and sets all other numeric fields to 0\.  
\- \`update(\&mut self)\`: Updates \`delta\_time\`, increments \`accumulated\_time\` and \`frame\_count\`, and resets \`last\_frame\_time\` to now.  
\- \`delta\_time(\&self) \-\> f32\`  
\- \`fixed\_delta\_time(\&self) \-\> f32\`  
\- \`elapsed\_time(\&self) \-\> f32\` (returns total seconds since engine start)  
\- \`should\_fixed\_update(\&mut self) \-\> bool\`: Checks if accumulated time exceeds the fixed step interval; if true, decrements accumulated time by the step interval and returns true.  
\- \`interpolation\_alpha(\&self) \-\> f32\`: Returns the remaining ratio (\`accumulated\_time / fixed\_delta\_time\`).

### **Prompt 2.3: Transforms & Mathematics (src/core/math.rs)**

Plaintext  
Create \`src/core/math.rs\` which re-exports all items from \`glam::\*\`.   
Define a public struct \`Transform\` that derives \`Debug\`, \`Clone\`, \`Copy\`, \`PartialEq\`, and a custom \`Component\` trait. It must contain:  
\- \`position: Vec3\`  
\- \`rotation: Quat\`  
\- \`scale: Vec3\`

Implement \`Default\` (Position=ZERO, Rotation=IDENTITY, Scale=ONE).  
Implement these methods on \`Transform\`:  
\- \`from\_position(position: Vec3) \-\> Self\`  
\- \`to\_matrix(\&self) \-\> Mat4\` using \`Mat4::from\_scale\_rotation\_translation\`  
\- \`forward(\&self) \-\> Vec3\` (returns the transformed direction of \`Vec3::NEG\_Z\`)  
\- \`right(\&self) \-\> Vec3\` (returns the transformed direction of \`Vec3::X\`)  
\- \`up(\&self) \-\> Vec3\` (returns the transformed direction of \`Vec3::Y\`)  
\- \`look\_at(\&mut self, target: Vec3, up: Vec3)\`: Modifies the rotation quaternion to look at a target position using right-handed logic (\`Mat4::look\_at\_rh\`).

Add a submodule \`math\_utils\` containing standalone functions for:  
\- \`lerp(a: f32, b: f32, t: f32) \-\> f32\`  
\- \`clamp\<T: PartialOrd\>(value: T, min: T, max: T) \-\> T\`  
\- \`deg2rad(deg: f32) \-\> f32\`  
\- \`rad2deg(rad: f32) \-\> f32\`

### **Prompt 2.4: Engine Configuration Management (src/core/config.rs)**

Plaintext  
Implement \`src/core/config.rs\` for file-based configuration. Use \`serde\` to make the following structures serializable and deserializable:  
\- \`WindowConfig\`: Contains fields \`title: String\`, \`width: u32\`, \`height: u32\`, \`fullscreen: bool\`, and \`resizable: bool\`.  
\- \`EngineConfig\`: Contains fields \`window: WindowConfig\`, \`log\_level: String\`, and \`fixed\_timestep: f32\`.

Implement \`Default\` for \`EngineConfig\` with a 1280x720 resizable window, title "RustyRacer Engine", log level "info", and timestep 1/60.  
Provide two public methods on \`EngineConfig\`:  
\- \`load\_from\_file(path: \&str) \-\> EngineResult\<Self\>\` (Reads from path and parses JSON)  
\- \`save\_to\_file(\&self, path: \&str) \-\> EngineResult\<()\>\` (Serializes to pretty JSON and saves to path)

## **⚙️ Module 3: Custom Entity Component System (ECS)**

### **Prompt 3.1: Entity & Generator Design (src/ecs/entity.rs)**

Plaintext  
Write a pure-Rust lightweight entity identification system in \`src/ecs/entity.rs\`.  
\- Create a tuple-struct \`Entity(pub u64)\` that derives \`Debug\`, \`Clone\`, \`Copy\`, \`PartialEq\`, \`Eq\`, and \`Hash\`.   
\- Implement \`fmt::Display\` for \`Entity\` formatting it as \`"Entity({})"\`.  
\- Create an \`EntityGenerator\` struct with a \`next\_id: u64\` field. Provide a \`new()\` constructor and a \`generate(\&mut self) \-\> Entity\` method that safely increments and yields new sequential Entity IDs.

### **Prompt 3.2: Cache-Friendly Component Storage System (src/ecs/component.rs)**

Plaintext  
Implement a Sparse Set Component Storage system in \`src/ecs/component.rs\`.  
\- Define a blanket marker trait: \`pub trait Component: Clone \+ Send \+ Sync \+ 'static \+ std::fmt::Debug {}\` and auto-implement it for all compliant types.  
\- Define a type-erased object trait \`ComponentStorage: std::any::Any \+ Send \+ Sync\` featuring methods: \`remove(\&mut self, entity: Entity)\`, \`has(\&self, entity: Entity) \-\> bool\`, \`len(\&self) \-\> usize\`, \`is\_empty(\&self) \-\> bool\`, \`as\_any(\&self) \-\> \&dyn Any\`, and \`as\_any\_mut(\&mut self) \-\> \&mut dyn Any\`.  
\- Implement a type-safe \`SparseStorage\<T: Component\>\` struct using three vectors: \`sparse: Vec\<Option\<usize\>\>\`, \`dense: Vec\<Entity\>\`, and \`data: Vec\<T\>\`.   
\- Implement safe insertions, lookups (\`get\`, \`get\_mut\`), and an efficient \`swap\_remove\` style deletion algorithm that keeps components closely packed in the \`dense\` array.  
\- Implement the \`ComponentStorage\` trait for \`SparseStorage\<T\>\`. Include standard immutable and mutable reference iterators over \`(Entity, \&T)\` and \`(Entity, \&mut T)\`.

### **Prompt 3.3: Global State & Component Container (src/ecs/world.rs)**

Plaintext  
Write the unified state container \`src/ecs/world.rs\`.  
\- Define a blanket \`Resource\` trait for global state: \`pub trait Resource: Send \+ Sync \+ 'static \+ std::any::Any \+ Clone {}\`.  
\- Build the \`World\` struct containing: \`entity\_gen: EntityGenerator\`, \`components: HashMap\<std::any::TypeId, Box\<dyn ComponentStorage\>\>\`, and \`resources: HashMap\<std::any::TypeId, Box\<dyn Any \+ Send \+ Sync\>\>\`.  
\- Implement standard operations on \`World\`: \`create\_entity\`, \`destroy\_entity\` (which must clean up deleted entities from all existing component storages via log tracing calls), \`register\_component\<T\>\`, \`add\_component\<T\>\`, \`get\_component\<T\>\`, \`get\_component\_mut\<T\>\`, \`has\_component\<T\>\`, \`add\_resource\<R\>\`, \`get\_resource\<R\>\`, \`get\_resource\_mut\<R\>\`, and an iterator method \`iter\_components\<T\>()\` downcasting storage systems seamlessly. Ensure log entry points are recorded using the \`log\` crate.

### **Prompt 3.4: Compile-Time Query Framework (src/ecs/query.rs)**

Plaintext  
Implement \`src/ecs/query.rs\` to allow type-safe iteration over specific component configurations in the \`World\`.  
\- Define a structure \`Query\<'w, T: Queryable\>\` containing fields \`world: &'w World\` and a \`PhantomData\` marker.  
\- Define a trait \`Queryable\` with an associated lifetime type \`Item\<'a\>\` and a lookup extraction function \`fetch(world: \&World, entity: Entity) \-\> Option\<Self::Item\<'\_\>\>\`.  
\- Implement \`Queryable\` for single component immutable references (\`\&T\`) and mutable references (\`\&mut T\`). Hint: Use unsafe code block pointers to gracefully bypass alias check constraints within the localized loop.  
\- Use a declarative macro \`impl\_queryable\_tuple\!\` to auto-generate \`Queryable\` implementations for tuple groups up to size 4: \`(A, B)\`, \`(A, B, C)\`, and \`(A, B, C, D)\`.  
\- Provide an \`iter()\` function on \`Query\` that resolves structural array matches against internal component sparse layouts.

### **Prompt 3.5: Systems Architecture & Execution Scheduling (src/ecs/system.rs & scheduler.rs)**

Plaintext  
Build out the logic execution engine for the ECS framework.  
In \`src/ecs/system.rs\`:  
\- Create a \`System\` trait with a \`run(\&mut self, world: \&mut World)\` entry point alongside optional hook methods \`initialize\` and \`cleanup\`.  
\- Build a generic helper function \`system\<F\>(func: F)\` converting plain closure arguments into a \`FunctionSystem\` wrapper.

In \`src/ecs/scheduler.rs\`:  
\- Define an \`ExecutionStage\` enum outlining explicit frame steps: \`PreUpdate\`, \`Input\`, \`Update\`, \`Physics\`, \`PostUpdate\`, \`Render\`.  
\- Create a \`Scheduler\` struct containing an execution order list: \`systems: Vec\<Box\<dyn System\>\>\`.  
\- Provide implementation methods allowing developers to add new logic structures via \`add\_system\<S\>\`, call global bootstrap initialization steps via \`initialize(\&mut self, world: \&mut World)\`, run processing sweeps using \`run\`, and drop components gracefully through \`cleanup\`.

## **📦 Module 4: Asset Architecture Core**

### **Prompt 4.1: Handle-Based Asset Tracking Framework (src/assets/)**

Plaintext  
Write a robust, type-safe asset tracking structure inside \`src/assets/\`.  
In \`src/assets/handle.rs\`:  
\- Create a structure \`AssetHandle\<T: std::fmt::Debug \+ 'static\>\` that encapsulates a raw numeric \`id: u64\` and a \`PhantomData\` marker. Make sure it derives \`Debug\`, \`Clone\`, \`Copy\`, \`PartialEq\`, \`Eq\`, and \`Hash\`.

In \`src/assets/manager.rs\`:  
\- Define a placeholder asset trait: \`pub trait Asset: Clone \+ std::fmt::Debug \+ Send \+ Sync \+ 'static {}\`.  
\- Implement an \`AssetManager\` tracking loaded files: \`assets: HashMap\<u64, Box\<dyn Any \+ Send \+ Sync\>\>\`, \`path\_map: HashMap\<String, u64\>\`, and \`next\_id: u64\`.  
\- Provide a \`load\<T: Asset\>(\&mut self, path: \&str) \-\> EngineResult\<AssetHandle\<T\>\>\` method. For Phase 1, make it check if the path already exists in \`path\_map\`. If not, allocate a placeholder object using \`std::mem::zeroed\` (wrapped in an unsafe block) to simulate an asset container skeleton until Phase 2 file loading logic is ready.  
\- Add safe query access points: \`get\<T\>\` and \`get\_mut\<T\>\`.

## **🪟 Module 5: Platform, OS Events & Input Mapping**

### **Prompt 5.1: Window Initialization with Winit (src/platform/window.rs)**

Plaintext  
Write a clean platform abstraction layer inside \`src/platform/window.rs\`.  
\- Create a structural configuration wrapper called \`Window\` containing an inner \`winit::window::Window\` variable instance and an associated \`WindowConfig\` instance.  
\- Provide a public instantiation method \`new(config: WindowConfig, event\_loop: \&winit::event\_loop::EventLoop\<()\>) \-\> EngineResult\<Self\>\`.  
\- Convert configuration property values to corresponding winit properties using \`WindowAttributes::default()\`, matching structural settings for dimensions, title, and resize capability. Convert initialization failures into custom engine error variations using \`map\_err\`.  
\- Include clean access mapping functions: \`size()\`, \`title()\`, \`request\_close()\`, and \`inner()\`.

### **Prompt 5.2: State-Tracking Input Subsystem (src/platform/input.rs)**

Plaintext  
Implement an stateful tracking system inside \`src/platform/input.rs\`.  
\- Define an \`InputState\` struct containing standard operational fields:  
  \- \`pressed\_keys: HashSet\<VirtualKeyCode\>\`  
  \- \`just\_pressed\_keys: HashSet\<VirtualKeyCode\>\`  
  \- \`just\_released\_keys: HashSet\<VirtualKeyCode\>\`  
  \- \`pressed\_mouse: HashSet\<MouseButton\>\`  
  \- \`mouse\_position: (f32, f32)\`  
  \- \`mouse\_delta: (f32, f32)\`  
\- Write tracking access checks: \`is\_key\_pressed\`, \`is\_key\_just\_pressed\`, and \`is\_key\_just\_released\`.  
\- Add frame boundaries resetting method: \`clear\_frame\_state\` that empties delta arrays at frame boundaries.  
\- Add window event handlers: \`process\_key\_event\` and \`process\_mouse\_motion\`.  
\- Create an empty \`InputSystem\` struct implementing the \`System\` trait. During \`initialize\`, have it append an empty instance of \`InputState\` directly into global resources. On \`run\`, have it invoke \`clear\_frame\_state\` on that resource.

## **🛠️ Module 6: Module Assembly & Integration Validation**

### **Prompt 6.1: Utility Module Assembly (src/utils/)**

Plaintext  
Construct the engine auxiliary systems layout inside \`src/utils/\`.  
In \`src/utils/logging.rs\`:  
\- Build an execution bridge setup method: \`init\_logging(config: \&EngineConfig)\`.   
\- Configure the initialization pipeline using \`env\_logger::Builder\`. Set up processing properties to include full millisecond accuracy timestamps (\`format\_timestamp\_millis\`) and explicit code line track printing tags (\`format\_module\_path(true)\`). Read default visibility thresholds from config fields.

In \`src/utils/macros.rs\`:  
\- Keep this file ready as an empty structural module script wrapper for phase extensions.

Ensure submodules are properly declared and re-exported in \`src/utils/mod.rs\`.

### **Prompt 6.2: Complete Framework Verification Example (examples/basic\_window.rs)**

Plaintext  
Write a complete application integration verification routine inside \`examples/basic\_window.rs\` to validate the core framework systems.  
\- Initialize configuration data objects using \`EngineConfig::default()\` and configure logging dependencies by invoking \`utils::logging::init\_logging\`.  
\- Instantiate an operational state \`World\` and execution \`Scheduler\`. Add the default \`InputSystem\` to the scheduler, and initialize it.  
\- Create an OS window handle using \`winit::event\_loop::EventLoop::new()\` and pass it to your custom \`Window::new\`.  
\- Start the game loop block by processing \`event\_loop.run\` callbacks. Match and resolve standard window management states, closing the window when \`WindowEvent::CloseRequested\` triggers. On \`MainEventsCleared\`, invoke the scheduler execution cycle (\`scheduler.run\`), update state structures, and flag redrawing routines via \`request\_redraw()\`.  
