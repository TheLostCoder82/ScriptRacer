Here is the structured, step-by-step prompt sequence designed for GitHub Copilot.  
To maximize the quality of Copilot's output, this guide breaks down **Phase 4** into granular, isolated tasks. Each prompt provides complete contextual architecture, data structures, and precise constraints to ensure the generated code integrates seamlessly into your existing codebase.

## **🛠️ Module 1: Editor & Debug Tools Implementation**

### **Prompt 1.1: Debug Renderer & 3D Visualization System**

Plaintext  
Context: Building the editor and debug tooling for a custom 3D racing engine in Rust using \`wgpu\`, \`rapier3d\`, and an internal ECS architecture. 

Task: Write a complete implementation of \`src/editor/debug\_draw.rs\`. 

Requirements:  
1\. Define a \`DebugRenderer\` struct holding an internal \`debug\_draw::DebugDraw\` instance alongside boolean flags: \`enabled\`, \`draw\_physics\`, \`draw\_vehicles\`, and \`draw\_navmesh\`. Impl \`Default\`.  
2\. Implement \`draw\_axes(\&mut self, transform: \&Transform, size: f32)\` drawing lines for X (Red), Y (Green), and Z (Blue).  
3\. Implement \`draw\_colliders(\&mut self, physics\_world: \&PhysicsWorld)\` matching \`rapier3d::geometry::ShapeType::Ball\` (render sphere via \`Color::CYAN\`) and \`ShapeType::Cuboid\` (render cuboid via \`Color::YELLOW\`).  
4\. Implement \`draw\_vehicle\_debug(\&mut self, vehicle: \&Vehicle, transform: \&Transform)\` that visualizes suspension travel via lines, wheels via circles, and wheel contacts/normals via contact lines if \`wheel.is\_contact\` is true.  
5\. Implement \`render(\&mut self, renderer: \&mut Renderer, camera: \&Camera, transform: \&Transform)\` passing the command encoder.  
6\. Implement the \`System\` trait for \`DebugRenderer\` where \`run(\&mut self, world: \&mut World)\` queries \`PhysicsWorld\`, \`Vehicle\`, \`Transform\`, and \`Checkpoint\` components to feed the drawing pipeline.

Constraints: Use crisp error handling and avoid placeholders. Use standard math traits corresponding to \`glam\` or \`nalgebra\` based on your prelude.

### **Prompt 1.2: Performance Profiler Overlay**

Plaintext  
Context: Integrating real-time frame profiling using \`puffin\` and \`puffin\_egui\` inside our custom engine context.

Task: Write the full code for \`src/editor/profiler\_ui.rs\`.

Requirements:  
1\. Create a \`ProfilerUi\` struct tracking whether the UI overlay is \`enabled\` and if sub-windows are \`open\_windows\`.  
2\. Provide an associated \`init()\` function turning on puffin scopes via \`puffin::set\_scopes\_on(true)\`.  
3\. Provide a \`draw(\&mut self, ctx: \&egui::Context)\` function creating an \`egui::Window\` titled "Performance Profiler".  
4\. Within the window, render \`puffin\_egui::profiler\_ui(ui)\`, frame time calculations, FPS metrics from the context history, and a collapsing header displaying \`puffin\_egui::stream\_graph\_ui(ui)\`.  
5\. Provide a simple mutable toggle function. Include a doc example using \`puffin::profile\_scope\!\`.

### **Prompt 1.3: In-Game Developer Console**

Plaintext  
Context: Implementing an interactive developer console using \`egui\` to execute engine runtime state updates.

Task: Write the implementation for \`src/editor/console.rs\`.

Requirements:  
1\. Define \`Console\`, \`ConsoleMessage\`, and a \`LogLevel\` enum (\`Info\`, \`Warning\`, \`Error\`, \`Success\`).  
2\. \`Console\` must contain a \`messages\` queue (\`VecDeque\`), an \`input\_buffer\` string, a command \`history\` queue, a visibility toggle, and configuration values for limits.  
3\. Implement \`log(\&mut self, text: impl Into\<String\>, level: LogLevel)\` extracting engine duration timestamping from your global resource clock.  
4\. Implement \`process\_command(\&mut self, world: \&mut World)\` parsing split tokens:  
   \- "help": prints available commands.  
   \- "list\_entities": queries total entities.  
   \- "toggle\_gravity": fetches \`PhysicsWorld\` as a mutable resource and toggles its Y vector value between absolute physics variables and zero.  
5\. Implement \`draw(\&mut self, ctx: \&egui::Context)\` setting an anchor layout window with a scrollable vertical history field tracking colored rows depending on \`LogLevel\`, plus a text input block handling execution on Enter keypresses.

## **🚀 Module 2: Advanced Asset Pipeline**

### **Prompt 2.1: Asset Compression Pipeline**

Plaintext  
Context: Writing an off-thread storage optimizer using \`lz4\_flex\` and \`zstd\` codecs.

Task: Write the full code for \`src/assets/pipeline/compression.rs\`.

Requirements:  
1\. Define a \`CompressionAlgorithm\` enum supporting \`None\`, \`LZ4\`, and \`ZSTD\`.  
2\. Implement a \`compress(\&self, data: &\[u8\]) \-\> EngineResult\<Vec\<u8\>\>\` method routing matching branches to respective encoder functions with sensible compression levels.  
3\. Implement a \`decompress(\&self, data: &\[u8\], original\_size: usize) \-\> EngineResult\<Vec\<u8\>\>\` restoring bytes.  
4\. Define a \`CompressedAsset\` structure tracking serialization attributes: \`algorithm\`, raw \`compressed\_data\`, \`original\_size\`, and a \`hash\` calculated via \`fxhash::hash64\`.  
5\. Implement verification validation methods inside raw slice instantiations checking structural parity post-decompression, raising an error on hash mismatches.

### **Prompt 2.2: Async Stream Processing & Map Partitioning**

Plaintext  
Context: Constructing an asynchronous asset streaming queue utilizing Tokio channels and spatial camera coordinates.

Task: Create \`src/assets/pipeline/streaming.rs\`.

Requirements:  
1\. Declare a \`StreamPriority\` enum sorted deterministically by ordering rules: \`Critical\`, \`High\`, \`Medium\`, \`Low\`.  
2\. Build structural definitions for a \`StreamRequest\` containing path, \`AssetHandle\`, and priority, along with an orchestration \`AssetStreamer\` manager.  
3\. In \`AssetStreamer::new\`, spin up an explicit background worker using \`tokio::spawn\` receiving path commands over an MPSC boundary, performing asynchronous file reads via \`tokio::fs::read\`, and channeling raw bytes back out.  
4\. Provide non-blocking \`request\` allocation checks against an internal tracking tracking pool (\`HashSet\`).  
5\. Write an engine \`System\` implementation for \`StreamingSystem\` extracting camera transformations, dynamically evaluating asset priority distances, and applying raw bytes back into \`AssetManager\`.

## **🌐 Module 3: Multiplayer Networking with Rollback Netcode**

### **Prompt 3.1: Network Protocol & Channel Mappings**

Plaintext  
Context: Standardizing binary network serialization for raw client/server architectures using \`renet\` and \`serde\`.

Task: Create \`src/networking/messages.rs\`.

Requirements:  
1\. Establish a unified \`NetworkMessage\` enum mapping out variants for handshakes (\`Handshake\`, \`HandshakeAccepted\`, \`HandshakeRejected\`), player state synchronization parameters (containing frame numbers, positional vectors, rotation quaternions, velocity arrays, engine RPM, gear index), and structured world text arrays.  
2\. Declare a public static array configuration definition (\`CHANNELS: &\[renet::ChannelConfig\]\`) detailing three separate execution layers:  
   \- Channel 0: ReliableOrdered for system orchestration.  
   \- Channel 1: UnreliableSequenced optimized for time-critical physics inputs.  
   \- Channel 2: Unreliable raw distribution arrays.

### **Prompt 3.2: Deterministic Rollback Orchestrator**

Plaintext  
Context: Implementing frame-accurate deterministic rollback netcode for the physics and vehicle synchronization layers.

Task: Generate \`src/networking/rollback.rs\`.

Requirements:  
1\. Define a bounded historic state capture struct \`SavedState\` caching the simulation frame index, vector allocations of \`Vehicle\` elements, clones of the \`PhysicsWorld\`, and matched input indices.  
2\. Construct the primary \`RollbackSystem\` utilizing a historic ring buffer tracking elements up to a fixed maximum history length (e.g., 8 frames).  
3\. Write \`save\_state(\&mut self, world: \&World)\` copying snapshot updates.  
4\. Write \`rollback(\&mut self, target\_frame: u32, world: \&mut World) \-\> EngineResult\<()\>\` looking up historic parameters, forcing deep state mutations back onto the physics systems and query layouts, and rolling back internal frame configurations.  
5\. Create an \`advance\_frame(\&mut self, world: \&mut World)\` function updating system states frame-by-frame. Combine everything under an integrated \`System\` trait implementation checking \`NetworkState\` for required rollbacks.

### **Prompt 3.3: Client Transport Interface**

Plaintext  
Context: Creating the \`renet\` network client interface for local connections.

Task: Implement \`src/networking/client.rs\`.

Requirements:  
1\. Define a \`GameClient\` management struct capturing an instantiated \`RenetClient\`, local ID parameters, destination addresses, and state flags.  
2\. Implement connection bootstrapping operations parsing target sockets and profiles.  
3\. Expose a explicit binary transmission routine \`send\_input(\&mut self, input: \&RacingInputState, frame: u32) \-\> EngineResult\<()\>\` using \`bincode\` packet serialization targeting the high-performance network channel.  
4\. Implement a comprehensive update processing cycle parsing incoming byte packets, executing state modifications depending on confirmation types, and rewriting local transformation coordinates for network entities.

## **⚡ Module 4: Performance Optimizations**

### **Prompt 4.1: Octree Spatial Partitioning System**

Plaintext  
Context: Building an internal hierarchical 3D spatial index for fast view-frustum culling and optimization queries.

Task: Write the source code for \`src/optimization/spatial\_partition.rs\`.

Requirements:  
1\. Create a private structural implementation layout representing a localized \`OctreeNode\` tracking dimensional boundary boxes (\`Aabb\`), child array linkages, and tracked array identifiers (\`Entity\`).  
2\. Implement explicit spatial math intersections on structural \`Aabb\` bounds arrays.  
3\. Build the public wrapper orchestration framework labeled \`Octree\`, defining insert routines that verify capacity properties before triggering subdivision operations into localized quadrant coordinates.  
4\. Provide search query mechanics extraction routines matching bounding volumes.  
5\. Write an associated frame loop execution block for \`SpatialPartitionSystem\` processing all components with \`Transform\` and \`MeshRenderer\`, populating a clean visibility collection list, and registering it as a resource named \`VisibleEntities\`.

### **Prompt 4.2: Work-Stealing Job System Execution Framework**

Plaintext  
Context: Designing a lightweight parallel data-processing execution layer over our custom thread pool engine using Rayon.

Task: Complete the source code file \`src/optimization/threading.rs\`.

Requirements:  
1\. Create a thread management driver structure named \`JobSystem\` containing an internal \`rayon::ThreadPool\`.  
2\. Supply constructor initialization tools mapping system core bounds counts efficiently to the target builder thread architecture.  
3\. Expose execution scoping abstractions \`run\<F, R\>(\&self, f: F) \-\> R\` mapping closures into deep executor pools.  
4\. Provide generic inline iterative loops \`par\_for\_each\<T, F\>(\&self, items: \&mut \[T\], f: F)\` for multi-threaded iterations over sliced data arrays.  
5\. Construct a mock system representation structure \`ParallelPhysicsSystem\` utilizing the runtime systems to perform parallel vehicle coordinate physics tracking across multiple worker threads.

## **📝 Module 5: Build Scripts & Tooling Pipeline**

### **Prompt 5.1: Asset Verification Build Script**

Plaintext  
Context: Writing a Cargo build compilation dependency asset processing pipeline layer matching target operating parameters.

Task: Provide the complete implementation for \`build.rs\` at the root of the project workspace.

Requirements:  
1\. Map standard execution logic handling automatic shader validation processing changes. Monitor your local WGSL asset directories via \`cargo:rerun-if-changed\` outputs.  
2\. Read the source file structure, capture runtime asset inputs, and automatically process configuration outputs targeting designated profile output fields.  
3\. Dynamically read environment parameters tracking host output variables (\`OUT\_DIR\`) to copy project asset dependencies into compilation target directories automatically.  
