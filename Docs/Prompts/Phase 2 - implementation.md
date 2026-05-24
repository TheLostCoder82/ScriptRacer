Here is an optimized, modular sequence of prompts designed for GitHub Copilot.  
Since LLMs and Copilot perform best when code generation is isolated to a single responsibility at a time, this guide breaks down the Phase 2 implementation into small, highly contextual steps. It provides the necessary boilerplate, dependencies, and type references for each prompt so Copilot doesn't hallucinate missing types.

## **🛠️ Module 1: Dependencies & Base Layout**

### **Prompt 1.1: Update Cargo.toml Dependencies**

Plaintext  
Update the project's Cargo.toml file to add the Phase 2 dependencies while preserving existing Phase 1 dependencies (glam, winit, serde, serde\_json, log, env\_logger, thiserror, anyhow, hashbrown, once\_cell). 

Add the following crates with their specified features:  
\- wgpu (v0.20) with features: \["vulkan", "metal", "dx12", "webgpu", "spirv-shader-passthrough"\]  
\- wgpu-profiler (v0.13)  
\- bytemuck (v1.14) with feature: \["derive"\]  
\- image (v0.24) with features: \["png", "jpeg", "tga", "bmp"\]  
\- gltf (v1.1) with feature: \["serde"\]  
\- rapier3d (v0.21) with features: \["f32", "serde"\]  
\- kira (v0.15) with features: \["symphonia", "serde", "std"\]  
\- symphonia (v0.5) with features: \["mp3", "ogg", "wav", "flac"\]  
\- egui (v0.26)  
\- egui-wgpu (v0.26) with feature: \["wgpu-0-20"\]  
\- egui-winit (v0.26) with feature: \["winit-0-30"\]  
\- tokio (v1.35) with feature: \["full"\]  
\- walkdir (v2.4)  
\- hotlib (v0.1)

## **🎨 Module 2: Rendering Subsystem**

### **Prompt 2.1: Implement Render Device Wrapper (src/render/device.rs)**

Plaintext  
Create \`src/render/device.rs\`. Implement a struct \`RenderDevice\` that acts as a wrapper around core wgpu objects. 

Requirements:  
1\. It must contain fields for \`instance: wgpu::Instance\`, \`adapter: wgpu::Adapter\`, \`device: wgpu::Device\`, and \`queue: wgpu::Queue\`.  
2\. Implement an asynchronous \`pub async fn new() \-\> EngineResult\<Self\>\` method. It should initialize the wgpu instance tracking Backends::VULKAN | Backends::METAL | Backends::DX12, request a high-performance adapter, and request a device and queue with default limits and features. Use logging to trace initialization.  
3\. Implement \`pub fn create\_shader\_module(\&self, source: \&str, label: \&str) \-\> wgpu::ShaderModule\` using \`ShaderSource::Wgsl\`.

### **Prompt 2.2: Build Core Render Components & Types (src/render/resources.rs)**

Plaintext  
Create \`src/render/resources.rs\`. Import types from your prelude, \`super::device::RenderDevice\`, and \`wgpu\`.

Implement the following structures:  
1\. \`Vertex\`: A C-compatible struct (\`\#\[repr(C)\]\`) deriving Pod and Zeroable, containing \`position: Vec3\`, \`normal: Vec3\`, and \`tex\_coords: Vec2\`. Implement \`pub fn desc\<'a\>() \-\> wgpu::VertexBufferLayout\<'a\>\` matching these attributes at locations 0, 1, and 2\.  
2\. \`Mesh\`: A struct containing \`name: String\`, \`vertex\_buffer: wgpu::Buffer\`, \`index\_buffer: wgpu::Buffer\`, and \`index\_count: u32\`. Implement a constructor \`pub fn new(device: \&RenderDevice, name: \&str, vertices: &\[Vertex\], indices: &\[u32\]) \-\> Self\` that initializes both buffers using buffer initialization descriptors with \`BufferUsages::VERTEX\` and \`BufferUsages::INDEX\`.  
3\. \`Texture\`: A struct containing \`texture: wgpu::Texture\`, \`view: wgpu::TextureView\`, \`sampler: wgpu::Sampler\`, and \`size: (u32, u32)\`.  
4\. \`Material\`: A struct containing \`name: String\`, \`shader: String\`, \`base\_color: Vec4\`, \`base\_color\_texture: Option\<AssetHandle\<Texture\>\>\`, and \`pipeline\_id: usize\`.  
5\. ECS Components: Create three separate \`\#\[derive(Component)\]\` structs:  
   \- \`MeshRenderer\`: holds \`mesh: AssetHandle\<Mesh\>\`, \`material: AssetHandle\<Material\>\`, and \`visible: bool\`.  
   \- \`DirectionalLight\`: holds \`direction: Vec3\`, \`color: Vec3\`, and \`intensity: f32\`.  
   \- \`PointLight\`: holds \`position: Vec3\`, \`color: Vec3\`, \`intensity: f32\`, and \`radius: f32\`.

### **Prompt 2.3: Add Camera Math and Uniform Buffers (src/render/camera.rs)**

Plaintext  
Create \`src/render/camera.rs\`.   
1\. Define an enum \`Projection\` with variants \`Perspective\` and \`Orthographic\`.  
2\. Create a component struct \`Camera\` containing \`projection: Projection\`, \`fov\_y: f32\`, \`near: f32\`, \`far: f32\`, \`viewport: (u32, u32)\`, and \`active: bool\`. Implement \`Default\` with perspective projection, 60-degree FOV in radians, near \= 0.1, far \= 1000.0, and viewport \= (1280, 720).  
3\. Implement methods on \`Camera\`:  
   \- \`build\_projection\_matrix(\&self) \-\> Mat4\` using \`Mat4::perspective\_rh\` or \`Mat4::orthographic\_rh\`.  
   \- \`build\_view\_matrix(\&self, transform: \&Transform) \-\> Mat4\` using \`Mat4::look\_at\_rh\` based on the transform's position, forward vector, and up vector.  
   \- \`build\_view\_projection(\&self, transform: \&Transform) \-\> Mat4\` to multiply the projection matrix by the view matrix.  
4\. Create a C-compatible uniform struct \`CameraUniform\` containing \`view\_proj: Mat4\`, \`position: Vec3\`, and \`\_padding: f32\`. Derive Pod and Zeroable. Add a constructor \`pub fn new(camera: \&Camera, transform: \&Transform) \-\> Self\`.

### **Prompt 2.4: Write the WGSL Shaders (src/render/shaders/)**

Plaintext  
Create two vertex and fragment WGSL shader files:

1\. \`src/render/shaders/basic.vert.wgsl\`: Write a 3D vertex shader that accepts a \`CameraUniform\` struct at group 0 binding 0, a \`ModelUniform\` struct at group 1 binding 1, and standard \`VertexInput\` locations (position, normal, tex\_coords). Compute and return a \`VertexOutput\` with calculated \`world\_pos\`, normalized world space \`normal\`, \`tex\_coords\`, and clip space position via the camera's \`view\_proj\`.

2\. \`src/render/shaders/basic.frag.wgsl\`: Write a fragment shader that accepts group 0 binding 2 (\`Light\` uniform buffer), group 2 binding 0 (\`base\_color: vec4\<f32\>\`), group 2 binding 1 (\`base\_texture: texture\_2d\<f32\>\`), and group 2 binding 2 (\`base\_sampler: sampler\`). Compute standard diffuse lighting via \`dot(norm, \-light.direction)\`, sample the diffuse color map texture, blend it with \`base\_color\`, and return the illuminated fragment color.

### **Prompt 2.5: Build the Render ECS System (src/render/systems.rs)**

Plaintext  
Create \`src/render/systems.rs\`. Implement \`RenderSystem\`, which implements your engine's \`System\` trait.

Requirements:  
1\. \`RenderSystem\` should wrap a \`Renderer\` struct instance.  
2\. In the \`run(\&mut self, world: \&mut World)\` method:  
   \- Extract the \`AssetManager\` resource.  
   \- Run a Query to find the active \`Camera\` component and its matching \`Transform\`. If not found, return early.  
   \- Collect all visible renderable objects by querying \`(\&Transform, \&MeshRenderer)\`. Look up their meshes and materials via the \`AssetManager\` and push them into an array of references.  
   \- Query all \`DirectionalLight\` and \`PointLight\` components to collect lighting attributes.  
   \- Call \`self.renderer.render(camera, cam\_transform, \&renderables, \&dir\_lights, \&point\_lights)\` and log any errors returned.

## **🧱 Module 3: Physics Subsystem**

### **Prompt 3.1: Define Physics Component Set (src/physics/components.rs)**

Plaintext  
Create \`src/physics/components.rs\`.   
1\. Define a custom enum \`RigidBodyType\` containing variants: \`Static\`, \`Dynamic\`, \`KinematicPositionBased\`, and \`KinematicVelocityBased\`. Implement \`From\<RigidBodyType\> for rapier3d::dynamics::RigidBodyType\`.  
2\. Implement an ECS component \`RigidBody\` containing fields: \`handle: rapier3d::dynamics::RigidBodyHandle\`, \`body\_type: RigidBodyType\`, and \`mass: f32\`.  
3\. Define a custom enum \`ColliderShape\` with variants: \`Ball(f32)\`, \`Cuboid(Vec3)\`, \`Capsule { radius: f32, height: f32 }\`, \`Cylinder { radius: f32, height: f32 }\`, and \`TriMesh\`.  
4\. Implement an ECS component \`Collider\` containing fields: \`handle: rapier3d::geometry::ColliderHandle\` and \`shape: ColliderShape\`.

### **Prompt 3.2: Implement Rapier3D Physics World (src/physics/world.rs)**

Plaintext  
Create \`src/physics/world.rs\`. Implement a wrapper struct called \`PhysicsWorld\` that owns all mandatory Rapier3D simulation components:  
\- gravity: Vec3  
\- integration\_parameters: IntegrationParameters  
\- rigidbody\_set: RigidBodySet  
\- collider\_set: ColliderSet  
\- impulse\_joint\_set: ImpulseJointSet  
\- multibody\_joint\_set: MultibodyJointSet  
\- island\_manager: IslandManager  
\- broad\_phase: BroadPhase  
\- narrow\_phase: NarrowPhase  
\- physics\_pipeline: PhysicsPipeline  
\- query\_pipeline: QueryPipeline

Requirements:  
1\. Implement \`Default\` to initialize all fields to their standard initial states with a default gravity of (0.0, \-9.81, 0.0).  
2\. Implement \`pub fn step(\&mut self, delta\_time: f32)\` which sets \`integration\_parameters.dt\`, triggers \`physics\_pipeline.step()\`, and updates the \`query\_pipeline\`.  
3\. Implement \`pub fn create\_rigidbody(\&mut self, ty: RigidBodyType, position: Vec3, rotation: Quat, mass: f32) \-\> RigidBody\` using Rapier's \`RigidBodyBuilder\`.  
4\. Implement \`pub fn create\_collider(\&mut self, shape: ColliderShape, rb: \&RigidBody, friction: f32, restitution: f32) \-\> Collider\` that converts the custom \`ColliderShape\` into a Rapier \`SharedShape\` and links it to the parent rigid body handle.

### **Prompt 3.3: Write Physics Sim & Sync Systems (src/physics/systems.rs)**

Plaintext  
Create \`src/physics/systems.rs\`. Implement two distinct systems implementing your \`System\` trait:

1\. \`PhysicsSystem\`:  
   \- \`initialize\`: Insert a default \`PhysicsWorld\` instance as an ECS world resource.  
   \- \`run\`: Retrieve the \`Time\` and \`PhysicsWorld\` resources, extract \`fixed\_delta\_time()\`, and call \`physics.step(dt)\`.

2\. \`PhysicsSyncSystem\`:  
   \- \`run\`: Two-way synchronization between physics simulation states and your engine's internal components:  
     a) Physics to Transform: Query all \`(\&RigidBody, \&mut Transform)\` targets. For \`Dynamic\` body types, fetch the rigid body position and orientation rotation data from \`PhysicsWorld\` and apply them to the entity's \`Transform\`.  
     b) Transform to Physics: Query all \`(\&mut RigidBody, \&Transform)\` targets. For non-Dynamic body types (kinematic/static), set their positions and rotations directly on the underlying simulation body objects inside the \`PhysicsWorld\`.

## **🔊 Module 4: Spatial Audio Subsystem**

### **Prompt 4.1: Audio Components (src/audio/components.rs)**

Plaintext  
Create \`src/audio/components.rs\`. Write the foundational structures for the Kira audio integration:  
1\. \`AudioClip\`: A custom type holding asset-side details: \`duration: f32\` and \`frame\_count: usize\`.  
2\. \`AudioSource\`: An ECS component representing a sound emitter with fields: \`clip: AssetHandle\<AudioClip\>\`, \`volume: f32\`, \`pitch: f32\`, \`looping: bool\`, \`play\_on\_spawn: bool\`, \`range: f32\`, \`is\_playing: bool\`, and \`track\_id: Option\<kira::track::TrackId\>\`. Implement \`Default\`.  
3\. \`AudioListener\`: An ECS component representing the audio receiver, holding \`active: bool\`. Implement \`Default\` where \`active \= true\`.

### **Prompt 4.2: Audio System Lifecycle (src/audio/system.rs)**

Plaintext  
Create \`src/audio/system.rs\`. Implement \`AudioSystem\` conforming to your engine's \`System\` trait.  
1\. \`initialize\`: Initialize a default \`AudioManager\` resource wrapper instance inside the world.  
2\. \`run\`:  
   \- Retrieve \`AudioManager\` mutably.  
   \- Query the active listener via \`(\&AudioListener, \&Transform)\`. Take the first match and pass its position, forward direction vector, and up vector into the sound backend listener tracker via \`audio.set\_listener(...)\`.  
   \- Query all emitting sources via \`(\&mut AudioSource, \&Transform)\` and update their spatial 3D audio tracks using their world position matrix with \`audio.update\_source(...)\`.

## **🖼️ Module 5: Immediate-Mode UI**

### **Prompt 5.1: Build Egui System Framework (src/ui/system.rs)**

Plaintext  
Create \`src/ui/system.rs\`. Implement \`UiSystem\` conforming to your engine's \`System\` trait.

Requirements:  
1\. \`UiSystem\` encapsulates a custom context wrapper field named \`context: UiContext\`.  
2\. Add a constructor \`pub fn new(render\_device: \&RenderDevice, window: \&Window) \-\> EngineResult\<Self\>\`.  
3\. Implement an explicit helper method \`pub fn handle\_event(\&mut self, event: \&winit::event::WindowEvent)\` that redirects raw input occurrences straight down to the underlying context wrapper layer.  
4\. Implement \`run(\&mut self, world: \&mut World)\`:  
   \- Call \`self.context.begin\_frame()\` to get the raw egui UI context object context.  
   \- Open an upper structural container overlay panel using \`egui::TopBottomPanel::top("top\_bar")\`. Add an engine heading label, compute current frame rendering speed metrics, and present it inside a text label block format string.  
   \- Open an explicit visual debug window widget container using \`egui::Window::new("Debug")\` to serve as a placeholders for structural scene metrics.  
   \- Close the interface canvas definition lifecycle step execution sequence by invoking \`self.context.end\_frame(world)\`.

## **📦 Module 6: Asset Pipeline Extensions & Core Base Library**

### **Prompt 6.1: Add Asset Loaders (src/assets/manager.rs)**

Plaintext  
Extend your existing \`AssetManager\` implementation file \`src/assets/manager.rs\` to include asset loading helpers using the \`image\` and \`gltf\` crates.

Implement two methods:  
1\. \`pub fn load\_texture(\&mut self, device: \&RenderDevice, path: \&Path) \-\> EngineResult\<AssetHandle\<Texture\>\>\`  
   \- Open and read raw data payloads from the path using \`image::open\`. Extract structural configuration width and height specs.  
   \- Construct a \`wgpu::Texture\` descriptor targeting 2D structures configured with \`TextureFormat::Rgba8UnormSrgb\`. Write data buffer streams onto the queue via \`device.queue.write\_texture\`.  
   \- Create standard linear sampler configurations and structural view handles. Cache the product, increment your allocation identification tracking sequence index registry maps, and return an explicit \`AssetHandle\`.

2\. \`pub fn load\_gltf(\&mut self, device: \&RenderDevice, path: \&Path) \-\> EngineResult\<Vec\<AssetHandle\<Mesh\>\>\>\`  
   \- Open and parse 3D structures through file formats using \`Gltf::open\`. Iteratively map mesh groupings down across underlying data subsets.  
   \- Read mesh primitives position locations, standard normal coordinate tracking offsets, and UV texture coordinate streams using \`primitive.reader\`. Assemble individual nodes together into custom \`Vertex\` arrays.  
   \- Construct full engine \`Mesh\` structural representations via standard buffers initialization sequences. Store objects inside the shared asset map framework layer, returning a collection of handles.

### **Prompt 6.2: Add Transform Propagation System (src/base\_lib/systems/transform\_system.rs)**

Plaintext  
Create \`src/base\_lib/systems/transform\_system.rs\`.  
1\. Create a modern component structure \`\#\[derive(Component, Default)\] pub struct GlobalTransform\` that wraps an internal transformation tracking matrix \`pub matrix: Mat4\`.  
2\. Implement a system type called \`TransformPropagationSystem\` implementing the \`System\` trait.  
3\. In its \`run\` execution logic loop, establish a clear entity data query scanning over components match sets of type \`(\&Transform, \&mut GlobalTransform)\`. For each match, invoke \`.to\_matrix()\` against the local tracking values and store the result matrix directly within the mapped global transform property field location.

## **🚀 Module 7: Core Runtime Loop Integration**

### **Prompt 7.1: Assemble Complete Engine Wrapper Loop (src/engine.rs)**

Plaintext  
Rewrite or complete the core runner framework interface orchestration logic inside \`src/engine.rs\`. 

Requirements:  
1\. The \`Engine\` instance struct wraps properties: \`config: EngineConfig\`, \`time: Time\`, \`world: World\`, \`scheduler: Scheduler\`, \`window: Window\`, \`renderer: Renderer\`, and \`asset\_manager: AssetManager\`.  
2\. Implement an execution setup phase asynchronous entry point method \`pub async fn new(config: EngineConfig) \-\> EngineResult\<Self\>\`:  
   \- Initialize the logging configuration tracking setups.  
   \- Construct raw time trackers, empty ECS context structures, and processing pipelines scheduler registries.  
   \- Bootstrap standard application windows through platform layer loops. Initialize high-performance \`RenderDevice\` wrappers and primary \`Renderer\` tracking logic structures.  
   \- Register shared engine configurations (\`Time\`, \`AssetManager\`) directly into resources maps.  
   \- Populate initialization workflows into schedulers mapping out specific sequential processing tasks tracking execution across: \`InputSystem\`, \`TransformPropagationSystem\`, \`PhysicsSystem\`, \`PhysicsSyncSystem\`, \`AudioSystem\`, \`RenderSystem\`, and \`UiSystem\`.  
3\. Implement \`pub fn run(mut self) \-\> EngineResult\<()\>\` using winit's event loop tracker execution blocks:  
   \- Route standard incoming OS interface message triggers down across internal component logic layers (e.g., process input events via \`InputState\`, window events close routines).  
   \- Upon matching frame clearing state execution flags (\`Event::MainEventsCleared\`), increment primary system loop timers. Run fixed game loop updates (\`while self.time.should\_fixed\_update()\`) by invoking system tasks iteratively across \`self.scheduler.run(\&mut self.world)\`.  
   \- Complete standard main loop workflows by emitting raw frame configuration rendering request updates via explicit window redraw queries.  
