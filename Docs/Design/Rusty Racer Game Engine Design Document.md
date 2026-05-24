Game Engine Design Document  
   
Project Name: RustyRacer Engine  
Version: 1.0  
Date: 2026-05-25  
Author: Beginner Rust Developer  
Target Platform: Ubuntu 24.04 LTS  
Language: Rust  
Core Paradigm: Entity Component System (ECS)  
Primary Purpose: Learning project, lightweight, modular, racing-game focused, easy to extend  
   
   
   
1\. Executive Summary  
   
RustyRacer Engine is a lightweight 3D game engine written in Rust, designed as an educational project. It centers on a custom-built Entity Component System (ECS) with minimal dependencies, and includes pre-built components, systems, and tools. It uses  glam  for math,  rapier  for physics with a custom vehicle physics layer, spatial audio, multi-input support including force-feedback wheels, and a UI system. Networking is excluded initially but designed for future integration. The engine ships with a base library for general games and an extension library purpose-built for racing games.  
   
2\. Development Environment  
   
2.1 Base Environment  
   
\- OS: Ubuntu 24.04 LTS (64-bit)  
\- Package Manager: APT \+ Cargo  
\- Toolchain: Rust via  rustup  / Rust Rover  
\- Minimum Rust version: 1.75+ (stable channel)  
\- Target:  x86\_64-unknown-linux-gnu   
\- Build System: Cargo (default)  
\- IDE/Editor: VS Code \+ rust-analyzer, or Neovim with LSP  
\- Dependencies Installation:  
bash    
\# System libraries  
sudo apt install build-essential libssl-dev pkg-config libasound2-dev libudev-dev libxcb-randr0-dev libx11-dev libxext-dev libxi-dev libxcursor-dev  
   
   
2.2 Core Crates & Versions  
   
Purpose Crate Reasoning   
Math  glam  ≥ 0.28 Fast, simple, widely used, no heavy dependencies   
Physics  rapier3d  ≥ 0.21 High-performance, reliable, Rust-native, good docs   
ECS Base Custom-built — Learning focus, minimal dependencies, full control   
Audio  kira  ≥ 0.15 /  oddio  Spatial audio support, pure Rust   
Input  gilrs  ≥ 0.11 \+  sdl2  ≥ 0.35 Gamepad/FFB \+ cross-platform input abstraction   
Window/Render  wgpu  ≥ 0.20 \+  winit  ≥ 0.30 Modern graphics, cross-vendor, safe API   
UI  egui  ≥ 0.26 Immediate-mode, easy integration, lightweight   
Logging/Debug  log  \+  env\_logger  ≥ 0.10 Simple logging for development   
Serialization  serde  \+  json  ≥ 1.0 For data/assets, optional use   
   
   
   
3\. Architecture Overview  
   
3.1 Core Principles  
   
1. Modularity: Each subsystem is independent with clear interfaces.  
2. ECS-First: All game data and logic built around entities, components, systems.  
3. Minimal Dependencies: Only use essential crates; custom code where possible.  
4. Extensible: Designed so networking, new features, or platforms can be added later.  
5. Beginner-Friendly: Clean code structure, good documentation, consistent patterns.  
   
3.2 High-Level Architecture Diagram  
   
plaintext    
┌─────────────────────────────────────────────────────────────┐  
│                     Application Layer                       │  
│  ├─ Game Loop / Main Control Flow                          │  
│  ├─ Asset Manager / Resource Loader                        │  
│  └─ Configuration / Settings                                │  
├─────────────────────────────────────────────────────────────┤  
│                     Core Engine Layer                       │  
│  ├─ Custom ECS Framework (Entities/Components/Systems)     │  
│  ├─ Math Library (glam)                                    │  
│  ├─ Rendering System (wgpu \+ winit)                       │  
│  ├─ Physics System (Rapier \+ Custom Vehicle Layer)        │  
│  ├─ Audio System (Spatial Audio)                           │  
│  ├─ Input System (Keyboard/Mouse/Touch/Gamepad/FFB)        │  
│  └─ UI System (egui integration)                           │  
├─────────────────────────────────────────────────────────────┤  
│                     Extension Libraries                     │  
│  ├─ Base Game Components/Systems                           │  
│  └─ Racing Game Extension Library                          │  
├─────────────────────────────────────────────────────────────┤  
│                     Future Integration Points               │  
│  ├─ Networking Interface (hooks ready)                      │  
│  └─ Scripting / Editor API                                 │  
└─────────────────────────────────────────────────────────────┘  
   
   
3.3 Module Structure  
   
plaintext    
rustyracer-engine/  
├── Cargo.toml  
├── src/  
│   ├── lib.rs           \# Public API  
│   ├── core/            \# Core utilities, math, common types  
│   ├── ecs/             \# Custom ECS implementation  
│   ├── render/          \# Rendering subsystem  
│   ├── physics/         \# Physics \+ vehicle logic  
│   ├── audio/           \# Spatial audio  
│   ├── input/           \# Input management  
│   ├── ui/              \# UI system  
│   ├── assets/          \# Asset loading & management  
│   ├── utils/           \# Helpers, logging, error handling  
│   ├── base\_lib/        \# Prebuilt general-purpose components/systems  
│   └── racing\_lib/      \# Racing-specific extension library  
└── examples/            \# Demo projects  
   
   
   
   
4\. Entity Component System (Custom Built)  
   
4.1 Design Goals  
   
\- Lightweight, no external ECS crates ( specs ,  bevy\_ecs  etc.)  
\- Minimal dependencies: only  glam ,  log , and standard library  
\- Simple API, easy to understand and extend  
\- Fast iteration and query performance  
   
4.2 Core Concepts  
   
\- Entity: Unique ID (u64) representing a game object; just an identifier, no data.  
\- Component: Plain Rust structs holding data (position, velocity, mesh, etc.)  
\- System: Logic that operates on entities with specific component combinations  
\- World: Container that holds all entities, components, and resources  
\- Query: Filter to access only the components a system needs  
\- Resource: Global shared data (settings, asset store, physics world)  
   
4.3 Implementation Details  
   
Entity  
   
rust    
\#\[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)\]  
pub struct Entity(pub u64);  
   
   
Component Trait  
   
rust    
pub trait Component: Clone \+ Send \+ Sync \+ 'static {}  
// Auto-implement for all matching types  
impl\<T: Clone \+ Send \+ Sync \+ 'static\> Component for T {}  
   
   
Storage  
   
Use sparse sets for efficient component storage:  
   
rust    
struct ComponentStorage\<T: Component\> {  
    sparse: Vec\<Option\<usize\>\>,  
    dense: Vec\<Entity\>,  
    data: Vec\<T\>,  
}  
   
   
\- Fast add/remove/access  
\- Cache-friendly iteration  
   
World Structure  
   
rust    
pub struct World {  
    next\_entity\_id: u64,  
    components: HashMap\<TypeId, Box\<dyn Any\>\>,  
    resources: HashMap\<TypeId, Box\<dyn Any\>\>,  
}

impl World {  
    pub fn new() \-\> Self { /\* ... \*/ }  
    pub fn create\_entity(\&mut self) \-\> Entity { /\* ... \*/ }  
    pub fn add\_component\<T: Component\>(\&mut self, entity: Entity, comp: T) { /\* ... \*/ }  
    pub fn get\_component\<T: Component\>(\&self, entity: Entity) \-\> Option\<\&T\> { /\* ... \*/ }  
    pub fn add\_resource\<R: 'static \+ Send \+ Sync\>(\&mut self, res: R) { /\* ... \*/ }  
}  
   
   
System & Scheduler  
   
Systems are functions/structs that run each frame.  
   
rust    
pub trait System: Send \+ Sync \+ 'static {  
    fn run(\&mut self, world: \&mut World);  
}

pub struct Scheduler {  
    systems: Vec\<Box\<dyn System\>\>,  
}

impl Scheduler {  
    pub fn add\_system\<S: System\>(\&mut self, system: S) { /\* ... \*/ }  
    pub fn run(\&mut self, world: \&mut World) {  
        for sys in \&mut self.systems { sys.run(world); }  
    }  
}  
   
   
4.4 Prebuilt Base Components  
   
rust    
// Common transform  
\#\[derive(Component, Clone, Debug)\]  
pub struct Transform {  
    pub position: glam::Vec3,  
    pub rotation: glam::Quat,  
    pub scale: glam::Vec3,  
}

// Rendering  
\#\[derive(Component, Clone, Debug)\]  
pub struct MeshRenderer {  
    pub mesh\_handle: AssetHandle\<Mesh\>,  
    pub material\_handle: AssetHandle\<Material\>,  
    pub visible: bool,  
}

// Physics  
\#\[derive(Component, Clone, Debug)\]  
pub struct RigidBody {  
    pub handle: rapier3d::dynamics::RigidBodyHandle,  
}  
\#\[derive(Component, Clone, Debug)\]  
pub struct Collider {  
    pub handle: rapier3d::geometry::ColliderHandle,  
}

// Audio  
\#\[derive(Component, Clone, Debug)\]  
pub struct AudioSource {  
    pub clip: AssetHandle\<AudioClip\>,  
    pub volume: f32,  
    pub looping: bool,  
    pub play\_on\_spawn: bool,  
}  
\#\[derive(Component, Clone, Debug)\]  
pub struct AudioListener {  
    pub active: bool,  
}

// Input  
\#\[derive(Component, Clone, Debug)\]  
pub struct PlayerController {  
    pub input\_map: InputMap,  
    pub enabled: bool,  
}  
   
   
4.5 Prebuilt Base Systems  
   
\-  TransformPropagationSystem : Updates global transforms from parent/child hierarchy  
\-  RenderSystem : Collects visible meshes and draws them  
\-  PhysicsStepSystem : Advances Rapier simulation and updates transforms  
\-  AudioSystem : Updates source positions relative to listener  
\-  InputSystem : Reads input devices and writes values to components/resources  
\-  UISystem : Draws UI elements and handles interaction  
   
   
   
5\. Math Subsystem  
   
5.1 Choice & Usage  
   
\- Crate:  glam  — SIMD-accelerated, minimal, consistent API  
\- Types used:  
\-  Vec2 ,  Vec3 ,  Vec4  — positions, directions, colors  
\-  Mat3 ,  Mat4  — rotations, projections, transforms  
\-  Quat  — rotations (preferred over Euler angles)  
\-  Rect ,  Angle  — helper types  
\- Integration: All components and systems use  glam  types directly; no wrapper layers to keep things simple  
\- Helper functions: Common operations like  look\_at ,  lerp ,  slerp , coordinate space conversion  
   
5.2 Coordinate System  
   
\- Right-handed: \+X \= right, \+Y \= up, \-Z \= forward  
\- Consistent across rendering, physics, and game logic  
   
   
   
6\. Physics Subsystem  
   
6.1 Base Physics Engine  
   
\- Engine: Rapier3D — open-source, Rust-native, stable, well-maintained  
\- Capabilities: Rigid bodies, colliders, joints, gravity, raycasts, character controllers  
\- Integration Flow:  
\- ECS  RigidBody / Collider  components store Rapier handles  
\-  PhysicsWorld  resource holds the Rapier  PhysicsPipeline ,  RigidBodySet ,  ColliderSet  etc.  
\-  PhysicsStepSystem  runs fixed timestep updates (typically 60Hz)  
\- Transform data is synced between ECS and Rapier each frame  
   
6.2 Custom Vehicle Physics Overlay  
   
Since Rapier does not include built-in vehicle simulation, we will build a modular, reusable vehicle system on top.  
   
Core Components  
   
rust    
\#\[derive(Component, Clone, Debug)\]  
pub struct Vehicle {  
    pub chassis\_mass: f32,  
    pub wheel\_count: usize,  
    pub wheel\_base: f32,       // Distance between front/rear axles  
    pub track\_width: f32,       // Distance between left/right wheels  
    pub center\_of\_mass: Vec3,  
    pub drive\_type: DriveType,  // FWD / RWD / AWD  
    pub max\_engine\_torque: f32,  
    pub max\_steer\_angle: f32,  
    pub brake\_force: f32,  
}

\#\[derive(Clone, Debug)\]  
pub enum DriveType { FWD, RWD, AWD }

\#\[derive(Component, Clone, Debug)\]  
pub struct Wheel {  
    pub radius: f32,  
    pub width: f32,  
    pub suspension\_travel: f32,  
    pub spring\_stiffness: f32,  
    pub damping: f32,  
    pub grip\_factor: f32,  
    pub is\_steering: bool,  
    pub is\_driven: bool,  
    pub local\_position: Vec3,  
    // Runtime state  
    pub current\_suspension\_offset: f32,  
    pub contact\_point: Option\<Vec3\>,  
}  
   
   
Core Systems  
   
\-  VehiclePhysicsSystem :  
1. Suspension calculation: raycast/sweep test to find ground contact, apply spring/damper forces  
2. Tire friction: apply longitudinal/lateral forces based on slip and friction model (Pacejka magic formula simplified)  
3. Drivetrain: distribute engine torque to wheels, simulate gear ratios and inertia  
4. Steering: apply steering forces and geometry  
5. Sync computed forces to Rapier rigid body  
\-  VehicleInputSystem : Maps player input (throttle, brake, steer) to vehicle state  
\-  VehicleAudioSystem : Engine RPM-based sound modulation, tire screech sounds  
   
Physics Model Notes  
   
\- Start with simplified, stable model — prioritise fun/playability over absolute realism  
\- Use raycasts for suspension and ground detection  
\- Tuneable parameters exposed in components for easy adjustment  
\- Support different vehicle types: karts, sports cars, trucks  
   
6.3 Racing-Specific Physics Extensions  
   
\- Surface friction modifiers (tarmac, gravel, grass, ice)  
\- Aerodynamics: downforce/drag forces based on speed and angle  
\- Damage simulation: deformation, performance reduction  
\- Tyre wear and heat effects  
   
   
   
7\. Rendering Subsystem  
   
7.1 Technology  
   
\- API:  wgpu  — modern, cross-platform graphics (Vulkan on Linux)  
\- Window/Events:  winit  — cross-platform window and input handling  
\- Shader Language: WGSL (native wgpu shading language)  
\- Asset Formats: GLTF/GLB for models/textures, PNG/JPEG/TGA for images, KTX2 for compressed textures  
   
7.2 Core Structure  
   
\- RenderDevice: Wrapper around wgpu logical device and queues  
\- Renderer: Main interface for drawing commands  
\- RenderPipeline: Predefined pipelines for opaque/transparent/debug geometry  
\- Camera Component:  
   
rust    
\#\[derive(Component, Clone, Debug)\]  
pub struct Camera {  
    pub projection: Projection,  
    pub fov\_y: f32,  
    pub near: f32,  
    pub far: f32,  
    pub viewport: (u32, u32),  
    pub active: bool,  
}

\#\[derive(Clone, Debug)\]  
pub enum Projection { Perspective, Orthographic }  
   
   
\- RenderSystem:  
\- Collects all  MeshRenderer  \+  Transform  \+ active  Camera   
\- Frustum culling  
\- Sort objects (opaque front-to-back, transparent back-to-front)  
\- Record command buffers and submit to GPU  
   
7.3 Features  
   
\- Basic Phong lighting model (directional, point, spot lights)  
\- Simple shadow mapping  
\- Texturing, normal mapping, color tinting  
\- Debug drawing: lines, spheres, boxes, normals, physics colliders  
\- UI overlay rendering  
   
7.4 Racing-Specific Rendering  
   
\- Skid marks / tire tracks  
\- Particle effects: exhaust smoke, dust, sparks  
\- Dashboard/HUD rendering  
\- Rear-view mirror / multi-camera support  
   
   
   
8\. Audio Subsystem  
   
8.1 Requirements  
   
\- 3D spatial audio: sounds change volume/pitch/panning based on position and listener  
\- Support for common formats: WAV, OGG, MP3  
\- Streaming and fully loaded audio  
\- Low-latency playback  
\- Control over volume, pitch, and attenuation  
\- Easy integration with ECS  
   
8.2 Implementation  
   
\- Library:  kira  — modern Rust audio library with built-in spatialisation  
\- Core Components:  
\-  AudioSource : Defines sound clip, position, loop, volume, pitch  
\-  AudioListener : Defines the "ears" of the game world, usually attached to main camera or player vehicle  
\- Audio System:  
\- Reads positions from  Transform  components  
\- Updates emitter/listener transforms every frame  
\- Manages playback instances, stops sounds that go out of range  
\- Handles resource management and cleanup  
\- Spatialisation: Uses distance attenuation, directional cones, and Doppler effect  
   
8.3 Racing-Specific Audio  
   
\- Engine sounds with RPM-based pitch modulation  
\- Tire slip sounds dependent on surface type and slip speed  
\- Collision/impact sounds based on mass and velocity  
\- Wind noise at high speeds  
\- Gear change sounds  
   
   
   
9\. Input Subsystem  
   
9.1 Requirements  
   
\- Support multiple input devices: keyboard, mouse, touch, gamepads, racing wheels with force feedback  
\- Abstracted input mapping: bind logical actions to physical inputs  
\- Support analog and digital inputs  
\- Allow rebinding controls  
\- Easy integration with ECS and game logic  
\- Network-ready structure (separate input reading from consumption)  
   
9.2 Implementation  
   
\- Libraries:  
\-  gilrs : Gamepad/joystick abstraction  
\-  sdl2 : For racing wheel support and force feedback  
\-  winit : Keyboard/mouse/touch events  
\- Core Abstractions:  
   
rust    
// Logical game actions (e.g. accelerate, brake, steer\_left)  
\#\[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)\]  
pub enum Action {  
    Accelerate, Brake, Steer, Handbrake, ShiftUp, ShiftDown, ResetCamera,  
}

pub struct InputMap {  
    bindings: HashMap\<Action, Vec\<InputBinding\>\>,  
}

pub enum InputBinding {  
    Key(winit::keyboard::KeyCode),  
    MouseButton(winit::event::MouseButton),  
    MouseAxis { axis: MouseAxis, scale: f32 },  
    GamepadButton(gilrs::Button),  
    GamepadAxis(gilrs::Axis, f32, f32), // axis, deadzone, scale  
    RacingWheelButton(u32),  
    RacingWheelAxis(u32, f32, f32),  
}

// Input state storage  
pub struct InputState {  
    pub values: HashMap\<Action, f32\>, // \-1.0..1.0 or 0.0..1.0  
    pub just\_pressed: HashSet\<Action\>,  
    pub just\_released: HashSet\<Action\>,  
}  
   
   
\- Input System:  
\- Polls events each frame  
\- Applies deadzones, scaling, and inversion  
\- Updates global  InputState  resource  
\- Writes input values to  PlayerController  or vehicle components  
   
9.3 Racing Wheel & Force Feedback  
   
\- Support for common wheels (Logitech G29/G920, Thrustmaster, Fanatec)  
\- Force feedback effects:  
\- Spring force (centering resistance)  
\- Damper force (resistance to turning)  
\- Friction / road feel  
\- Collision/impact vibration  
\- Understeer/oversteer feedback  
\- API:  fn set\_force\_feedback(\&mut self, effect: FfEffect) \-\> Result\<(), InputError\>   
   
9.4 Extensibility  
   
\- Input layer is completely separated from game logic  
\- All inputs stored as logical actions → easy to add new devices later  
\- State recorded with timestamps → ready for rollback/netcode  
   
   
   
10\. UI Subsystem  
   
10.1 Requirements  
   
\- Immediate-mode UI for development tools, menus, and in-game HUD  
\- Simple, easy-to-use API  
\- Text rendering, buttons, sliders, checkboxes, windows, text input  
\- Support for different resolutions and scaling  
\- Integrates cleanly with wgpu and the main render loop  
   
10.2 Implementation  
   
\- Library:  egui  \+  egui-wgpu  \+  egui-winit  — lightweight immediate-mode UI  
\- Integration:  
\-  UiSystem  runs after input and before rendering  
\- Receives input events and builds UI state  
\- Generates vertex/index buffers that the main renderer draws  
\- Core Components/Resources:  
\-  UiContext : Global UI state  
\-  HudElement : Component for in-world or screen-space UI elements  
\- Example Usage:  
   
rust    
fn draw\_hud(ui: \&mut egui::Ui, state: \&GameState) {  
    ui.label(format\!("Speed: {:.1} km/h", state.speed \* 3.6));  
    ui.label(format\!("RPM: {:.0}", state.engine\_rpm));  
    ui.label(format\!("Gear: {}", state.current\_gear));  
}  
   
   
10.3 Racing-Specific UI  
   
\- Speedometer, tachometer, gear indicator  
\- Lap times, position, race status  
\- Minimap / track map  
\- Menu system: main menu, pause menu, settings, car selection  
   
   
   
11\. Asset Management  
   
11.1 Design  
   
\- Centralised system for loading, storing, and referencing game assets  
\- Reference counting handles ( AssetHandle\<T\> ) to avoid duplication  
\- Support for synchronous and asynchronous loading  
\- Hot-reloading for development  
   
11.2 Asset Types  
   
\- Meshes, materials, textures  
\- Audio clips  
\- Shaders  
\- Scenes/prefabs  
\- Script/data files  
   
11.3 Structure  
   
rust    
\#\[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)\]  
pub struct AssetHandle\<T\> { pub id: u64, \_marker: std::marker::PhantomData\<T\> }

pub struct AssetManager {  
    meshes: HashMap\<u64, Mesh\>,  
    materials: HashMap\<u64, Material\>,  
    textures: HashMap\<u64, Texture\>,  
    audio\_clips: HashMap\<u64, AudioClip\>,  
    next\_id: u64,  
}

impl AssetManager {  
    pub fn load\_mesh(\&mut self, path: \&str) \-\> Result\<AssetHandle\<Mesh\>, Error\> { /\* ... \*/ }  
    // Similar methods for other asset types  
}  
   
   
   
   
12\. Racing Game Extension Library  
   
This is a separate crate/module built on top of the base engine, containing racing-specific logic.  
   
12.1 Racing-Specific Components  
   
rust    
// Race metadata  
\#\[derive(Component, Clone, Debug)\]  
pub struct RaceInfo {  
    pub track\_length: f32,  
    pub total\_laps: u32,  
    pub current\_lap: u32,  
    pub start\_position: u32,  
}

\#\[derive(Component, Clone, Debug)\]  
pub struct LapTimer {  
    pub current\_lap\_time: f32,  
    pub best\_lap\_time: Option\<f32\>,  
    pub last\_lap\_time: Option\<f32\>,  
    pub sector\_times: \[Option\<f32\>; 3\],  
}

\#\[derive(Component, Clone, Debug)\]  
pub struct Checkpoint {  
    pub index: u32,  
    pub transform: Transform,  
    pub radius: f32,  
    pub is\_start\_finish: bool,  
}

\#\[derive(Component, Clone, Debug)\]  
pub struct VehicleConfig {  
    pub engine: EngineSpec,  
    pub transmission: TransmissionSpec,  
    pub suspension: SuspensionConfig,  
    pub tires: TireCompound,  
}  
   
   
12.2 Racing-Specific Systems  
   
\-  RaceManagerSystem : Tracks laps, positions, race state  
\-  CheckpointSystem : Detects checkpoint crossing, updates lap times  
\-  VehicleSetupSystem : Applies configuration changes to physics  
\-  AiDriverSystem : Basic racing AI logic (path following, overtaking, braking)  
\-  TrackRenderSystem : Handles track meshes, barriers, and environment objects  
\-  LeaderboardSystem : Tracks and sorts player/AI positions  
   
12.3 Racing Game Template  
   
A starter project demonstrating:  
   
\- Car physics setup  
\- Basic track with checkpoints  
\- Lap timing  
\- Camera follow  
\- Input mapping  
\- HUD  
   
   
   
13\. Networking Readiness  
   
Although networking is excluded from the initial version, the engine is designed to make future integration straightforward.  
   
13.1 Key Design Choices  
   
\- Separation of Concerns: No networking code in core systems; game logic is written deterministically or with clear state boundaries  
\- State Serialization: All core components implement  Serialize / Deserialize  via  serde   
\- Input/Output Abstraction: Input and simulation are separated — input can come from local or remote sources  
\- Time Management: Fixed timestep simulation makes it easier to implement rollback or client-side prediction  
\- Architecture Boundaries:  
\- Clear "Game State" object that can be transmitted  
\- Systems operate on data, not on network connections directly  
\- No direct coupling between game objects and transport layer  
   
13.2 Future Integration Points  
   
\- Add a  NetworkSystem  that synchronises entities/components  
\- Add a transport layer using  quinn  or  tokio   
\- Add message types and state replication logic  
\- Add client/server roles and authority checks  
   
   
   
14\. Development Roadmap  
   
Phase 1: Core Foundation (Weeks 1–4)  
   
\- ✅ Set up development environment  
\- ✅ Implement custom ECS  
\- ✅ Basic math library integration  
\- ✅ Asset manager basics  
\- ✅ Window creation and input handling  
   
Phase 2: Core Engine Systems (Weeks 5–8)  
   
\- ✅ Rendering system with wgpu  
\- ✅ Integration of Rapier physics  
\- ✅ Audio system with spatial support  
\- ✅ UI system integration  
\- ✅ Base component/system library  
   
Phase 3: Vehicle & Racing Physics (Weeks 9–12)  
   
\- ✅ Build custom vehicle physics overlay  
\- ✅ Racing-specific components and systems  
\- ✅ Implement racing wheel \+ FFB support  
\- ✅ Basic racing game template  
   
Phase 4: Polish & Documentation (Weeks 13–16)  
   
\- ✅ Improve performance and error handling  
\- ✅ Write documentation and examples  
\- ✅ Add debug tools  
\- ✅ Prepare for future extension  
   
   
   
15\. Coding Conventions  
   
\- Follow standard Rust API guidelines  
\- Use meaningful, descriptive names  
\- Prefer composition over inheritance  
\- Document all public functions, structs, and traits  
\- Keep functions short and single-purpose  
\- Use  Result / Option  instead of panics where possible  
\- Format code with  cargo fmt  and lint with  cargo clippy   
   
   
   
16\. Conclusion  
   
RustyRacer Engine is a learning-focused, modular 3D game engine built in Rust. It uses a custom lightweight ECS, industry-standard libraries for math and physics, and adds racing-specific capabilities through a dedicated extension library. While initially lacking networking support, the architecture has been carefully designed to make future integration easy. It is an ideal starting point for understanding game engine architecture and Rust game development.  
   
 