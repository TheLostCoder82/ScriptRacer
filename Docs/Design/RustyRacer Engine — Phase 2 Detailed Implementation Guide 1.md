RustyRacer Engine — Phase 2 Detailed Implementation Guide  
   
Phase 2 Goal: Integrate core engine subsystems: rendering, physics, audio, UI, and full asset pipeline. Build production-ready base components/systems and finalize the core runtime loop.  
Timeline: Weeks 5–8 | Skill Level: Beginner–Intermediate Rust | Target OS: Ubuntu 24.04 LTS  
Prerequisite: Completed Phase 1 codebase  
   
   
   
🎯 Phase 2 Overview  
   
By the end of this phase you will have a fully functional engine runtime with:  
✅ Hardware-accelerated 3D rendering pipeline (wgpu \+ winit)  
✅ Full 3D physics integration with Rapier3D  
✅ Spatial audio system with  kira   
✅ Immediate-mode UI system with  egui   
✅ Complete asset loading, caching, and hot-reload system  
✅ Base component library (transforms, meshes, materials, lights, rigidbodies)  
✅ Base system library (transform propagation, rendering, physics step, audio update, input handling)  
✅ Stable main/game loop with fixed timestep and frame pacing  
   
   
   
📦 Step 1 — Update Dependencies  
   
Update  Cargo.toml  to add new required crates:  
   
toml    
\[package\]  
name \= "rustyracer-engine"  
version \= "0.1.0"  
edition \= "2021"

\[dependencies\]  
\# \--- PHASE 1 DEPENDENCIES (keep these) \---  
glam \= { version \= "0.28", features \= \["std", "bytemuck", "serde"\] }  
winit \= { version \= "0.30", features \= \["x11", "wayland", "serde"\] }  
serde \= { version \= "1.0", features \= \["derive"\] }  
serde\_json \= "1.0"  
log \= "0.4"  
env\_logger \= "0.10"  
thiserror \= "1.0"  
anyhow \= "1.0"  
hashbrown \= { version \= "0.14", features \= \["serde"\] }  
once\_cell \= "1.19"

\# \--- PHASE 2 NEW DEPENDENCIES \---  
\# Rendering  
wgpu \= { version \= "0.20", features \= \["vulkan", "metal", "dx12", "webgpu", "spirv-shader-passthrough"\] }  
wgpu-profiler \= "0.13"  
bytemuck \= { version \= "1.14", features \= \["derive"\] }  
image \= { version \= "0.24", features \= \["png", "jpeg", "tga", "bmp"\] }  
gltf \= { version \= "1.1", features \= \["serde"\] } \# GLTF model loading

\# Physics  
rapier3d \= { version \= "0.21", features \= \["f32", "serde"\] }

\# Audio  
kira \= { version \= "0.15", features \= \["symphonia", "serde", "std"\] }  
symphonia \= { version \= "0.5", features \= \["mp3", "ogg", "wav", "flac"\] }

\# UI  
egui \= "0.26"  
egui-wgpu \= { version \= "0.26", features \= \["wgpu-0-20"\] }  
egui-winit \= { version \= "0.26", features \= \["winit-0-30"\] }

\# Async/File loading  
tokio \= { version \= "1.35", features \= \["full"\] }  
walkdir \= "2.4"  
hotlib \= "0.1" \# Hot-reload support  
   
   
Install new dependencies:  
   
bash    
cargo update  
   
   
   
   
🎨 Step 2 — Rendering Subsystem Implementation  
   
We’ll build a modern, modular rendering pipeline using  wgpu .  
   
2.1 Add Render Module Structure  
   
plaintext    
src/render/  
├── mod.rs  
├── device.rs         \# WGPU device/queue/instance wrapper  
├── swapchain.rs      \# Window surface & frame management  
├── resources.rs      \# Mesh, Material, Texture, Shader types  
├── pipeline.rs       \# Render pipeline definitions  
├── camera.rs         \# Camera component & logic  
├── renderer.rs       \# Main rendering logic  
├── systems.rs        \# RenderSystem integration with ECS  
└── shaders/          \# WGSL shader files  
    ├── basic.vert.wgsl  
    ├── basic.frag.wgsl  
    └── ui.frag.wgsl  
   
   
2.2 src/render/mod.rs  
   
rust    
//\! 3D Rendering subsystem using wgpu  
pub mod device;  
pub mod swapchain;  
pub mod resources;  
pub mod pipeline;  
pub mod camera;  
pub mod renderer;  
pub mod systems;

pub use device::RenderDevice;  
pub use resources::{Mesh, Material, Texture, Shader};  
pub use camera::{Camera, Projection};  
pub use renderer::Renderer;  
pub use systems::RenderSystem;  
   
   
2.3 src/render/device.rs — Core Render Device  
   
rust    
use crate::core::EngineResult;  
use wgpu::\*;

/// Wrapper around core wgpu objects  
\#\[derive(Debug)\]  
pub struct RenderDevice {  
    pub instance: Instance,  
    pub adapter: Adapter,  
    pub device: Device,  
    pub queue: Queue,  
}

impl RenderDevice {  
    /// Initialize wgpu backend  
    pub async fn new() \-\> EngineResult\<Self\> {  
        log::info\!("Initializing rendering device");  
          
        let instance \= Instance::new(InstanceDescriptor {  
            backends: Backends::VULKAN | Backends::METAL | Backends::DX12,  
            ..Default::default()  
        });

        let adapter \= instance.request\_adapter(\&RequestAdapterOptions {  
            power\_preference: PowerPreference::HighPerformance,  
            compatible\_surface: None,  
            force\_fallback\_adapter: false,  
        }).await.ok\_or\_else(|| anyhow::anyhow\!("No suitable GPU adapter found"))?;

        let (device, queue) \= adapter.request\_device(  
            \&DeviceDescriptor {  
                label: Some("Main Device"),  
                required\_features: Features::empty(),  
                required\_limits: Limits::default(),  
                memory\_hints: MemoryHints::default(),  
            },  
            None  
        ).await?;

        Ok(Self { instance, adapter, device, queue })  
    }

    /// Create shader module from WGSL source  
    pub fn create\_shader\_module(\&self, source: \&str, label: \&str) \-\> ShaderModule {  
        self.device.create\_shader\_module(ShaderModuleDescriptor {  
            label: Some(label),  
            source: ShaderSource::Wgsl(source.into()),  
        })  
    }  
}  
   
   
2.4 src/render/resources.rs — GPU Resources  
   
rust    
use crate::prelude::\*;  
use super::device::RenderDevice;  
use wgpu::\*;  
use bytemuck::{Pod, Zeroable};

/// Vertex format: position \+ normal \+ UV  
\#\[repr(C)\]  
\#\[derive(Debug, Clone, Copy, Pod, Zeroable)\]  
pub struct Vertex {  
    pub position: Vec3,  
    pub normal: Vec3,  
    pub tex\_coords: Vec2,  
}

impl Vertex {  
    pub fn desc\<'a\>() \-\> VertexBufferLayout\<'a\> {  
        VertexBufferLayout {  
            array\_stride: std::mem::size\_of::\<Vertex\>() as u64,  
            step\_mode: VertexStepMode::Vertex,  
            attributes: &\[  
                VertexAttribute { format: VertexFormat::Float32x3, offset: 0, shader\_location: 0 },  
                VertexAttribute { format: VertexFormat::Float32x3, offset: 12, shader\_location: 1 },  
                VertexAttribute { format: VertexFormat::Float32x2, offset: 24, shader\_location: 2 },  
            \],  
        }  
    }  
}

/// GPU Mesh resource  
\#\[derive(Debug, Clone)\]  
pub struct Mesh {  
    pub name: String,  
    pub vertex\_buffer: Buffer,  
    pub index\_buffer: Buffer,  
    pub index\_count: u32,  
}

impl Mesh {  
    /// Create mesh from vertex/index data  
    pub fn new(device: \&RenderDevice, name: \&str, vertices: &\[Vertex\], indices: &\[u32\]) \-\> Self {  
        let vertex\_buffer \= device.device.create\_buffer\_init(\&BufferInitDescriptor {  
            label: Some(\&format\!("{}\_vertices", name)),  
            contents: bytemuck::cast\_slice(vertices),  
            usage: BufferUsages::VERTEX | BufferUsages::COPY\_DST,  
        });

        let index\_buffer \= device.device.create\_buffer\_init(\&BufferInitDescriptor {  
            label: Some(\&format\!("{}\_indices", name)),  
            contents: bytemuck::cast\_slice(indices),  
            usage: BufferUsages::INDEX | BufferUsages::COPY\_DST,  
        });

        Self {  
            name: name.to\_string(),  
            vertex\_buffer,  
            index\_buffer,  
            index\_count: indices.len() as u32,  
        }  
    }  
}

/// Texture resource  
\#\[derive(Debug, Clone)\]  
pub struct Texture {  
    pub texture: wgpu::Texture,  
    pub view: TextureView,  
    pub sampler: Sampler,  
    pub size: (u32, u32),  
}

/// Material: shader \+ textures \+ uniforms  
\#\[derive(Debug, Clone)\]  
pub struct Material {  
    pub name: String,  
    pub shader: String,  
    pub base\_color: Vec4,  
    pub base\_color\_texture: Option\<AssetHandle\<Texture\>\>,  
    pub pipeline\_id: usize,  
}

// \--- ECS Components \---  
\#\[derive(Component, Debug, Clone)\]  
pub struct MeshRenderer {  
    pub mesh: AssetHandle\<Mesh\>,  
    pub material: AssetHandle\<Material\>,  
    pub visible: bool,  
}

\#\[derive(Component, Debug, Clone)\]  
pub struct DirectionalLight {  
    pub direction: Vec3,  
    pub color: Vec3,  
    pub intensity: f32,  
}

\#\[derive(Component, Debug, Clone)\]  
pub struct PointLight {  
    pub position: Vec3,  
    pub color: Vec3,  
    pub intensity: f32,  
    pub radius: f32,  
}  
   
   
2.5 src/render/camera.rs  
   
rust    
use crate::prelude::\*;

\#\[derive(Debug, Clone, Copy, PartialEq, Eq)\]  
pub enum Projection {  
    Perspective,  
    Orthographic,  
}

\#\[derive(Component, Debug, Clone)\]  
pub struct Camera {  
    pub projection: Projection,  
    pub fov\_y: f32,  
    pub near: f32,  
    pub far: f32,  
    pub viewport: (u32, u32),  
    pub active: bool,  
}

impl Default for Camera {  
    fn default() \-\> Self {  
        Self {  
            projection: Projection::Perspective,  
            fov\_y: 60.0\_f32.to\_radians(),  
            near: 0.1,  
            far: 1000.0,  
            viewport: (1280, 720),  
            active: true,  
        }  
    }  
}

impl Camera {  
    /// Build projection matrix  
    pub fn build\_projection\_matrix(\&self) \-\> Mat4 {  
        let (width, height) \= self.viewport;  
        let aspect \= width as f32 / height as f32;

        match self.projection {  
            Projection::Perspective \=\> Mat4::perspective\_rh(self.fov\_y, aspect, self.near, self.far),  
            Projection::Orthographic \=\> {  
                let half\_h \= 10.0;  
                let half\_w \= half\_h \* aspect;  
                Mat4::orthographic\_rh(-half\_w, half\_w, \-half\_h, half\_h, self.near, self.far)  
            }  
        }  
    }

    /// Build view matrix from transform  
    pub fn build\_view\_matrix(\&self, transform: \&Transform) \-\> Mat4 {  
        Mat4::look\_at\_rh(  
            transform.position,  
            transform.position \+ transform.forward(),  
            transform.up()  
        )  
    }

    /// Build combined view-projection matrix  
    pub fn build\_view\_projection(\&self, transform: \&Transform) \-\> Mat4 {  
        self.build\_projection\_matrix() \* self.build\_view\_matrix(transform)  
    }  
}

/// Uniform buffer for camera data  
\#\[repr(C)\]  
\#\[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)\]  
pub struct CameraUniform {  
    view\_proj: Mat4,  
    position: Vec3,  
    \_padding: f32,  
}

impl CameraUniform {  
    pub fn new(camera: \&Camera, transform: \&Transform) \-\> Self {  
        Self {  
            view\_proj: camera.build\_view\_projection(transform),  
            position: transform.position,  
            \_padding: 0.0,  
        }  
    }  
}  
   
   
2.6 src/render/systems.rs — ECS Integration  
   
rust    
use crate::prelude::\*;  
use super::renderer::Renderer;

/// Main rendering system — runs every frame  
\#\[derive(Debug)\]  
pub struct RenderSystem {  
    renderer: Renderer,  
}

impl RenderSystem {  
    pub fn new(renderer: Renderer) \-\> Self {  
        Self { renderer }  
    }  
}

impl System for RenderSystem {  
    fn run(\&mut self, world: \&mut World) {  
        // Get resources  
        let assets \= world.get\_resource::\<AssetManager\>().unwrap();  
          
        // Get active camera  
        let mut camera\_query \= Query::\<(\&Camera, \&Transform)\>::new(world);  
        let Some((camera, cam\_transform)) \= camera\_query.iter().next() else {  
            return;  
        };

        // Collect visible renderables  
        let mut renderables \= Vec::new();  
        let mut mesh\_query \= Query::\<(\&Transform, \&MeshRenderer)\>::new(world);  
          
        for (transform, renderer) in mesh\_query.iter() {  
            if \!renderer.visible { continue; }  
            renderables.push((  
                transform.to\_matrix(),  
                assets.get(renderer.mesh).unwrap(),  
                assets.get(renderer.material).unwrap()  
            ));  
        }

        // Collect lights  
        let mut dir\_lights \= Vec::new();  
        let mut point\_lights \= Vec::new();  
          
        for (\_, light) in Query::\<\&DirectionalLight\>::new(world).iter() {  
            dir\_lights.push(\*light);  
        }  
        for (transform, light) in Query::\<(\&Transform, \&PointLight)\>::new(world).iter() {  
            let mut l \= \*light;  
            l.position \= transform.position;  
            point\_lights.push(l);  
        }

        // Render frame  
        if let Err(e) \= self.renderer.render(  
            camera, cam\_transform, \&renderables, \&dir\_lights, \&point\_lights  
        ) {  
            log::error\!("Render error: {}", e);  
        }  
    }  
}  
   
   
2.7 Add Shader Files  
   
src/render/shaders/basic.vert.wgsl  
   
wgsl    
struct CameraUniform {  
    view\_proj: mat4x4\<f32\>,  
    position: vec3\<f32\>,  
};  
@binding(0) @group(0) var\<uniform\> camera: CameraUniform;

struct ModelUniform {  
    model: mat4x4\<f32\>,  
};  
@binding(1) @group(1) var\<uniform\> model: ModelUniform;

struct VertexInput {  
    @location(0) position: vec3\<f32\>,  
    @location(1) normal: vec3\<f32\>,  
    @location(2) tex\_coords: vec2\<f32\>,  
};

struct VertexOutput {  
    @location(0) world\_pos: vec3\<f32\>,  
    @location(1) normal: vec3\<f32\>,  
    @location(2) tex\_coords: vec2\<f32\>,  
    @builtin(position: clip\_pos: vec4\<f32\>  
};

@vertex  
fn main(input: VertexInput) \-\> VertexOutput {  
    var output: VertexOutput;  
    output.world\_pos \= (model.model \* vec4\<f32\>(input.position, 1.0)).xyz;  
    output.normal \= normalize((model.model \* vec4\<f32\>(input.normal, 0.0)).xyz);  
    output.tex\_coords \= input.tex\_coords;  
    output.clip\_pos \= camera.view\_proj \* vec4\<f32\>(output.world\_pos, 1.0);  
    return output;  
}  
   
   
src/render/shaders/basic.frag.wgsl  
   
wgsl    
struct FragmentInput {  
    @location(0) world\_pos: vec3\<f32\>,  
    @location(1) normal: vec3\<f32\>,  
    @location(2) tex\_coords: vec2\<f32\>,  
};

struct Light {  
    direction: vec3\<f32\>,  
    color: vec3\<f32\>,  
    intensity: f32,  
};  
@binding(2) @group(0) var\<uniform\> light: Light;

@binding(0) @group(2) var base\_color: vec4\<f32\>;  
@binding(1) @group(2) var base\_texture: texture\_2d\<f32\>;  
@binding(2) @group(2) var base\_sampler: sampler;

@fragment  
fn main(input: FragmentInput) \-\> @location(0) vec4\<f32\> {  
    let norm \= normalize(input.normal);  
    let diffuse \= max(dot(norm, \-light.direction), 0.0) \* light.intensity;  
    let color \= textureSample(base\_texture, base\_sampler, input.tex\_coords) \* base\_color;  
    return vec4\<f32\>(color.rgb \* light.color \* diffuse, color.a);  
}  
   
   
   
   
🧱 Step 3 — Physics Subsystem Implementation  
   
Integrate Rapier3D and build the physics pipeline.  
   
3.1 Add Physics Module Structure  
   
plaintext    
src/physics/  
├── mod.rs  
├── world.rs          \# Rapier world wrapper  
├── components.rs     \# RigidBody, Collider components  
├── systems.rs        \# Physics step & sync systems  
└── vehicle/          \# Custom vehicle physics (prep for racing lib)  
    ├── mod.rs  
    ├── components.rs  
    └── system.rs  
   
   
3.2 src/physics/mod.rs  
   
rust    
//\! Physics subsystem powered by Rapier3D  
pub mod world;  
pub mod components;  
pub mod systems;  
pub mod vehicle;

pub use world::PhysicsWorld;  
pub use components::{RigidBody, Collider, RigidBodyType};  
pub use systems::{PhysicsSystem, PhysicsSyncSystem};  
   
   
3.3 src/physics/components.rs  
   
rust    
use crate::prelude::\*;  
use rapier3d::{dynamics, geometry};

/// Rigid body type: static / dynamic / kinematic  
\#\[derive(Debug, Clone, Copy, PartialEq, Eq)\]  
pub enum RigidBodyType {  
    Static,  
    Dynamic,  
    KinematicPositionBased,  
    KinematicVelocityBased,  
}

impl From\<RigidBodyType\> for dynamics::RigidBodyType {  
    fn from(ty: RigidBodyType) \-\> Self {  
        match ty {  
            RigidBodyType::Static \=\> dynamics::RigidBodyType::Static,  
            RigidBodyType::Dynamic \=\> dynamics::RigidBodyType::Dynamic,  
            RigidBodyType::KinematicPositionBased \=\> dynamics::RigidBodyType::KinematicPositionBased,  
            RigidBodyType::KinematicVelocityBased \=\> dynamics::RigidBodyType::KinematicVelocityBased,  
        }  
    }  
}

/// ECS Component: Rigid body reference  
\#\[derive(Component, Debug, Clone, Copy)\]  
pub struct RigidBody {  
    pub handle: dynamics::RigidBodyHandle,  
    pub body\_type: RigidBodyType,  
    pub mass: f32,  
}

/// ECS Component: Collider shape reference  
\#\[derive(Component, Debug, Clone, Copy)\]  
pub struct Collider {  
    pub handle: geometry::ColliderHandle,  
    pub shape: ColliderShape,  
}

/// Collider shape types  
\#\[derive(Debug, Clone, Copy)\]  
pub enum ColliderShape {  
    Ball(f32),  
    Cuboid(Vec3),  
    Capsule { radius: f32, height: f32 },  
    Cylinder { radius: f32, height: f32 },  
    TriMesh,  
}  
   
   
3.4 src/physics/world.rs  
   
rust    
use crate::prelude::\*;  
use rapier3d::prelude::\*;

/// Wrapper around Rapier physics world  
\#\[derive(Debug, Clone)\]  
pub struct PhysicsWorld {  
    pub gravity: Vec3,  
    pub integration\_parameters: IntegrationParameters,  
    pub rigidbody\_set: RigidBodySet,  
    pub collider\_set: ColliderSet,  
    pub impulse\_joint\_set: ImpulseJointSet,  
    pub multibody\_joint\_set: MultibodyJointSet,  
    pub island\_manager: IslandManager,  
    pub broad\_phase: BroadPhase,  
    pub narrow\_phase: NarrowPhase,  
    pub physics\_pipeline: PhysicsPipeline,  
    pub query\_pipeline: QueryPipeline,  
}

impl Default for PhysicsWorld {  
    fn default() \-\> Self {  
        Self {  
            gravity: Vec3::new(0.0, \-9.81, 0.0),  
            integration\_parameters: IntegrationParameters::default(),  
            rigidbody\_set: RigidBodySet::new(),  
            collider\_set: ColliderSet::new(),  
            impulse\_joint\_set: ImpulseJointSet::new(),  
            multibody\_joint\_set: MultibodyJointSet::new(),  
            island\_manager: IslandManager::new(),  
            broad\_phase: BroadPhase::new(),  
            narrow\_phase: NarrowPhase::new(),  
            physics\_pipeline: PhysicsPipeline::new(),  
            query\_pipeline: QueryPipeline::new(),  
        }  
    }  
}

impl PhysicsWorld {  
    /// Step simulation forward by fixed timestep  
    pub fn step(\&mut self, delta\_time: f32) {  
        self.integration\_parameters.dt \= delta\_time;  
          
        self.physics\_pipeline.step(  
            \&self.gravity,  
            \&self.integration\_parameters,  
            \&mut self.island\_manager,  
            \&mut self.broad\_phase,  
            \&mut self.narrow\_phase,  
            \&mut self.rigidbody\_set,  
            \&mut self.collider\_set,  
            \&mut self.impulse\_joint\_set,  
            \&mut self.multibody\_joint\_set,  
            \&mut CcdSolver::default(),  
            \&mut EventQueue::new(),  
            &(),  
            &(),  
        );

        self.query\_pipeline.update(\&self.rigidbody\_set, \&self.collider\_set);  
    }

    /// Create new rigid body  
    pub fn create\_rigidbody(\&mut self, ty: RigidBodyType, position: Vec3, rotation: Quat, mass: f32) \-\> RigidBody {  
        let rb\_desc \= RigidBodyBuilder::new(ty.into())  
            .translation(position.into())  
            .rotation(rotation.into())  
            .mass(mass)  
            .build();

        let handle \= self.rigidbody\_set.insert(rb\_desc);  
        RigidBody { handle, body\_type: ty, mass }  
    }

    /// Create collider and attach to rigid body  
    pub fn create\_collider(\&mut self, shape: ColliderShape, rb: \&RigidBody, friction: f32, restitution: f32) \-\> Collider {  
        let shape \= match shape {  
            ColliderShape::Ball(r) \=\> SharedShape::ball(r),  
            ColliderShape::Cuboid(v) \=\> SharedShape::cuboid(v.x, v.y, v.z),  
            ColliderShape::Capsule { radius, height } \=\> SharedShape::capsule(Vec3::Y \* height \* 0.5, Vec3::Y \* \-height \* 0.5, radius),  
            ColliderShape::Cylinder { radius, height } \=\> SharedShape::cylinder(height \* 0.5, radius),  
            ColliderShape::TriMesh \=\> unimplemented\!(),  
        };

        let collider\_desc \= ColliderBuilder::new(shape)  
            .friction(friction)  
            .restitution(restitution)  
            .build();

        let handle \= self.collider\_set.insert\_with\_parent(collider\_desc, rb.handle, \&mut self.rigidbody\_set);  
        Collider { handle, shape }  
    }  
}  
   
   
3.5 src/physics/systems.rs  
   
rust    
use crate::prelude::\*;  
use super::world::PhysicsWorld;

/// Run physics simulation step  
\#\[derive(Debug, Default)\]  
pub struct PhysicsSystem;

impl System for PhysicsSystem {  
    fn run(\&mut self, world: \&mut World) {  
        let time \= world.get\_resource::\<Time\>().unwrap();  
        let physics \= world.get\_resource\_mut::\<PhysicsWorld\>().unwrap();

        // Run fixed timestep updates  
        let dt \= time.fixed\_delta\_time();  
        physics.step(dt);  
    }

    fn initialize(\&mut self, world: \&mut World) {  
        world.add\_resource(PhysicsWorld::default());  
    }  
}

/// Sync transforms ↔ physics bodies  
\#\[derive(Debug, Default)\]  
pub struct PhysicsSyncSystem;

impl System for PhysicsSystem {  
    fn run(\&mut self, world: \&mut World) {  
        let physics \= world.get\_resource::\<PhysicsWorld\>().unwrap();

        // Physics → Transform (dynamic bodies)  
        let mut query \= Query::\<(\&RigidBody, \&mut Transform)\>::new(world);  
        for (rb, mut transform) in query.iter() {  
            if rb.body\_type \!= RigidBodyType::Dynamic { continue; }  
              
            let rb\_ref \= physics.rigidbody\_set.get(rb.handle).unwrap();  
            let pos \= rb\_ref.translation();  
            let rot \= rb\_ref.rotation();  
              
            transform.position \= Vec3::new(pos.x, pos.y, pos.z);  
            transform.rotation \= Quat::from\_xyzw(rot.x, rot.y, rot.z, rot.w);  
        }

        // Transform → Physics (kinematic/static bodies)  
        let mut query \= Query::\<(\&mut RigidBody, \&Transform)\>::new(world);  
        for (mut rb, transform) in query.iter() {  
            if rb.body\_type \== RigidBodyType::Dynamic { continue; }  
              
            let rb\_mut \= physics.rigidbody\_set.get\_mut(rb.handle).unwrap();  
            rb\_mut.set\_translation(transform.position.into(), true);  
            rb\_mut.set\_rotation(transform.rotation.into(), true);  
        }  
    }  
}  
   
   
   
   
🔊 Step 4 — Audio Subsystem Implementation  
   
Build spatial audio with  kira .  
   
4.1 Add Audio Module Structure  
   
plaintext    
src/audio/  
├── mod.rs  
├── manager.rs        \# Kira backend wrapper  
├── components.rs     \# AudioSource, AudioListener  
└── system.rs         \# Audio update system  
   
   
4.2 src/audio/mod.rs  
   
rust    
//\! Spatial audio subsystem powered by Kira  
pub mod manager;  
pub mod components;  
pub mod system;

pub use manager::AudioManager;  
pub use components::{AudioSource, AudioListener, AudioClip};  
pub use system::AudioSystem;  
   
   
4.3 src/audio/components.rs  
   
rust    
use crate::prelude::\*;  
use kira::track::TrackId;

/// Audio clip asset  
\#\[derive(Debug, Clone)\]  
pub struct AudioClip {  
    pub duration: f32,  
    pub frame\_count: usize,  
}

/// ECS Component: Audio source emitter  
\#\[derive(Component, Debug, Clone)\]  
pub struct AudioSource {  
    pub clip: AssetHandle\<AudioClip\>,  
    pub volume: f32,  
    pub pitch: f32,  
    pub looping: bool,  
    pub play\_on\_spawn: bool,  
    pub range: f32,  
    pub is\_playing: bool,  
    pub track\_id: Option\<TrackId\>,  
}

impl Default for AudioSource {  
    fn default() \-\> Self {  
        Self {  
            clip: AssetHandle::new(0),  
            volume: 1.0,  
            pitch: 1.0,  
            looping: false,  
            play\_on\_spawn: false,  
            range: 50.0,  
            is\_playing: false,  
            track\_id: None,  
        }  
    }  
}

/// ECS Component: Audio listener (ears)  
\#\[derive(Component, Debug, Clone)\]  
pub struct AudioListener {  
    pub active: bool,  
}

impl Default for AudioListener {  
    fn default() \-\> Self {  
        Self { active: true }  
    }  
}  
   
   
4.4 src/audio/system.rs  
   
rust    
use crate::prelude::\*;  
use super::manager::AudioManager;

/// Update 3D positions of audio sources/listener  
\#\[derive(Debug, Default)\]  
pub struct AudioSystem;

impl System for AudioSystem {  
    fn run(\&mut self, world: \&mut World) {  
        let audio \= world.get\_resource\_mut::\<AudioManager\>().unwrap();

        // Update listener position/orientation  
        let mut listener\_query \= Query::\<(\&AudioListener, \&Transform)\>::new(world);  
        if let Some((\_, transform)) \= listener\_query.iter().next() {  
            audio.set\_listener(transform.position, transform.forward(), transform.up());  
        }

        // Update audio sources  
        let mut source\_query \= Query::\<(\&mut AudioSource, \&Transform)\>::new(world);  
        for (source, transform) in source\_query.iter\_mut() {  
            audio.update\_source(source, transform.position);  
        }  
    }

    fn initialize(\&mut self, world: \&mut World) {  
        world.add\_resource(AudioManager::new().unwrap());  
    }  
}  
   
   
   
   
🖼️ Step 5 — UI Subsystem Implementation  
   
Integrate  egui  for in-game UI and tools.  
   
5.1 Add UI Module Structure  
   
plaintext    
src/ui/  
├── mod.rs  
├── context.rs        \# Egui integration  
├── components.rs     \# UI element components  
└── system.rs         \# UI rendering & interaction  
   
   
5.2 src/ui/mod.rs  
   
rust    
//\! Immediate-mode UI system using egui  
pub mod context;  
pub mod components;  
pub mod system;

pub use context::UiContext;  
pub use system::UiSystem;  
   
   
5.3 src/ui/system.rs  
   
rust    
use crate::prelude::\*;  
use super::context::UiContext;

/// UI system: handles input, layout, and rendering  
\#\[derive(Debug)\]  
pub struct UiSystem {  
    context: UiContext,  
}

impl UiSystem {  
    pub fn new(render\_device: \&RenderDevice, window: \&Window) \-\> EngineResult\<Self\> {  
        Ok(Self { context: UiContext::new(render\_device, window)? })  
    }

    /// Handle input events  
    pub fn handle\_event(\&mut self, event: \&winit::event::WindowEvent) {  
        self.context.handle\_event(event);  
    }  
}

impl System for UiSystem {  
    fn run(\&mut self, world: \&mut World) {  
        // Begin frame  
        let ctx \= self.context.begin\_frame();

        // Draw common UI  
        egui::TopBottomPanel::top("top\_bar").show(ctx, |ui| {  
            ui.heading("RustyRacer Engine");  
            ui.label(format\!("FPS: {:.1}", world.get\_resource::\<Time\>().unwrap().elapsed\_time()));  
        });

        // Draw debug window  
        egui::Window::new("Debug").show(ctx, |ui| {  
            ui.label("Debug info here");  
        });

        // End and render  
        self.context.end\_frame(world);  
    }  
}  
   
   
   
   
📦 Step 6 — Complete Asset Pipeline  
   
Extend the asset manager to support real loading, parsing, and hot-reloading.  
   
6.1 Update src/assets/manager.rs  
   
rust    
use crate::prelude::\*;  
use std::path::Path;  
use image::GenericImageView;  
use gltf::Gltf;

impl AssetManager {  
    /// Load texture from image file  
    pub fn load\_texture(\&mut self, device: \&RenderDevice, path: \&Path) \-\> EngineResult\<AssetHandle\<Texture\>\> {  
        let img \= image::open(path)?;  
        let dimensions \= img.dimensions();  
        let rgba \= img.to\_rgba8();

        let size \= wgpu::Extent3d {  
            width: dimensions.0,  
            height: dimensions.1,  
            depth\_or\_array\_layers: 1,  
        };

        let texture \= device.device.create\_texture(\&wgpu::TextureDescriptor {  
            label: Some(path.to\_str().unwrap()),  
            size,  
            mip\_level\_count: 1,  
            sample\_count: 1,  
            dimension: wgpu::TextureDimension::D2,  
            format: wgpu::TextureFormat::Rgba8UnormSrgb,  
            usage: wgpu::TextureUsages::TEXTURE\_BINDING | wgpu::TextureUsages::COPY\_DST,  
            view\_formats: &\[\],  
        });

        device.queue.write\_texture(  
            texture.as\_image\_copy(),  
            \&rgba,  
            wgpu::ImageDataLayout {  
                offset: 0,  
                bytes\_per\_row: Some(4 \* dimensions.0),  
                rows\_per\_image: Some(dimensions.1),  
            },  
            size,  
        );

        let view \= texture.create\_view(\&wgpu::TextureViewDescriptor::default());  
        let sampler \= device.device.create\_sampler(\&wgpu::SamplerDescriptor {  
            address\_mode\_u: wgpu::AddressMode::ClampToEdge,  
            address\_mode\_v: wgpu::AddressMode::ClampToEdge,  
            address\_mode\_w: wgpu::AddressMode::ClampToEdge,  
            mag\_filter: wgpu::FilterMode::Linear,  
            min\_filter: wgpu::FilterMode::Nearest,  
            mipmap\_filter: wgpu::FilterMode::Nearest,  
            ..Default::default()  
        });

        let tex \= Texture { texture, view, sampler, size: dimensions };  
        let id \= self.next\_id;  
        self.next\_id \+= 1;  
        self.assets.insert(id, Box::new(tex));  
        self.path\_map.insert(path.to\_string\_lossy().to\_string(), id);  
          
        Ok(AssetHandle::new(id))  
    }

    /// Load GLTF/GLB model  
    pub fn load\_gltf(\&mut self, device: \&RenderDevice, path: \&Path) \-\> EngineResult\<Vec\<AssetHandle\<Mesh\>\>\> {  
        let gltf \= Gltf::open(path)?;  
        let mut meshes \= Vec::new();

        for mesh in gltf.meshes() {  
            for primitive in mesh.primitives() {  
                let reader \= primitive.reader(|\_| None);  
                let positions: Vec\<\[f32;3\]\> \= reader.read\_positions().unwrap().collect();  
                let normals: Vec\<\[f32;3\]\> \= reader.read\_normals().unwrap().collect();  
                let tex\_coords: Vec\<\[f32;2\]\> \= reader.read\_tex\_coords(0).unwrap().into\_f32().collect();  
                let indices: Vec\<u32\> \= reader.read\_indices().unwrap().into\_u32().collect();

                let vertices: Vec\<Vertex\> \= positions.into\_iter()  
                    .zip(normals)  
                    .zip(tex\_coords)  
                    .map(|((p, n), t)| Vertex { position: Vec3::from(p), normal: Vec3::from(n), tex\_coords: Vec2::from(t) })  
                    .collect();

                let mesh \= Mesh::new(device, mesh.name().unwrap\_or("unnamed"), \&vertices, \&indices);  
                let id \= self.next\_id;  
                self.next\_id \+= 1;  
                self.assets.insert(id, Box::new(mesh));  
                meshes.push(AssetHandle::new(id));  
            }  
        }

        Ok(meshes)  
    }  
}  
   
   
6.2 Add Transform Propagation System  
   
rust    
// src/base\_lib/systems/transform\_system.rs  
use crate::prelude::\*;

/// Update global transforms from local \+ parent hierarchy  
\#\[derive(Debug, Default)\]  
pub struct TransformPropagationSystem;

impl System for TransformPropagationSystem {  
    fn run(\&mut self, world: \&mut World) {  
        // Simple single-level transform propagation  
        let mut query \= Query::\<(\&Transform, \&mut GlobalTransform)\>::new(world);  
        for (local, mut global) in query.iter() {  
            global.matrix \= local.to\_matrix();  
        }  
    }  
}

\#\[derive(Component, Debug, Clone, Copy, Default)\]  
pub struct GlobalTransform {  
    pub matrix: Mat4,  
}  
   
   
🚀 Step 7 — Main Runtime Integration  
   
Combine all subsystems into a working engine loop.  
   
7.1 src/engine.rs  
   
rust    
use crate::prelude::\*;  
use crate::platform::Window;  
use crate::render::Renderer;  
use crate::physics::PhysicsSystem;  
use crate::audio::AudioSystem;  
use crate::ui::UiSystem;

/// Main engine instance  
\#\[derive(Debug)\]  
pub struct Engine {  
    pub config: EngineConfig,  
    pub time: Time,  
    pub world: World,  
    pub scheduler: Scheduler,  
    pub window: Window,  
    pub renderer: Renderer,  
    pub asset\_manager: AssetManager,  
}

impl Engine {  
    /// Initialize engine  
    pub async fn new(config: EngineConfig) \-\> EngineResult\<Self\> {  
        utils::logging::init\_logging(\&config);  
        log::info\!("Initializing engine...");

        // Core  
        let time \= Time::new();  
        let mut world \= World::new();  
        let mut scheduler \= Scheduler::new();

        // Platform  
        let event\_loop \= EventLoop::new();  
        let window \= Window::new(config.window.clone(), \&event\_loop)?;

        // Rendering  
        let render\_device \= RenderDevice::new().await?;  
        let renderer \= Renderer::new(\&render\_device, \&window)?;

        // Asset manager  
        let mut asset\_manager \= AssetManager::new();

        // Add resources  
        world.add\_resource(time.clone());  
        world.add\_resource(asset\_manager.clone());

        // Add systems  
        scheduler.add\_system(InputSystem::default());  
        scheduler.add\_system(TransformPropagationSystem::default());  
        scheduler.add\_system(PhysicsSystem::default());  
        scheduler.add\_system(PhysicsSyncSystem::default());  
        scheduler.add\_system(AudioSystem::default());  
        scheduler.add\_system(RenderSystem::new(renderer.clone()));  
        scheduler.add\_system(UiSystem::new(\&render\_device, \&window)?);

        scheduler.initialize(\&mut world);

        Ok(Self { config, time, world, scheduler, window, renderer, asset\_manager })  
    }

    /// Run main game loop  
    pub fn run(mut self) \-\> EngineResult\<()\> {  
        log::info\!("Starting main loop");  
        let event\_loop \= EventLoop::new();

        event\_loop.run(move |event, \_, control\_flow| {  
            control\_flow.set\_poll();

            match event {  
                Event::WindowEvent { event, .. } \=\> {  
                    match event {  
                        WindowEvent::CloseRequested \=\> control\_flow.set\_exit(),  
                        \_ \=\> {  
                            // Pass event to input & UI systems  
                            if let Some(mut input) \= self.world.get\_resource\_mut::\<InputState\>() {  
                                input.process\_event(\&event);  
                            }  
                            // self.world.get\_resource\_mut::\<UiSystem\>().unwrap().handle\_event(\&event);  
                        }  
                    }  
                }

                Event::MainEventsCleared \=\> {  
                    // Update timing  
                    self.time.update();  
                    self.world.add\_resource(self.time.clone());

                    // Run fixed timestep updates  
                    while self.time.should\_fixed\_update() {  
                        self.scheduler.run(\&mut self.world);  
                    }

                    // Render  
                    self.window.inner().request\_redraw();  
                }

                \_ \=\> {}  
            }  
        })  
    }  
}  
   
   
✅ Phase 2 Completion Checklist  
   
Rendering pipeline working: can draw colored/textured 3D objects  
Camera system with perspective/orthographic projection  
Physics simulation working: rigid bodies, gravity, collision  
Spatial audio system: 3D sound positioning  
UI system: windows, text, buttons, debug overlays  
Asset manager loads images, models, and audio files  
ECS base components library complete  
Stable game loop with fixed timestep  
All subsystems integrated and communicating correctly  
