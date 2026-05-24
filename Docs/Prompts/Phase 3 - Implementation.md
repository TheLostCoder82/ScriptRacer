Here is the markdown file transformed into a structured, execution-ready sequence of context-aware prompts for GitHub Copilot.  
To ensure the best results, open the relevant file mentioned at the top of each prompt in your editor before executing it so Copilot has the appropriate workspace context.

## **🛠️ Step 1 — Project Configuration**

### **Prompt 1: Dependency Updates**

**File to open:** Cargo.toml

Plaintext  
Modify the Cargo.toml file to add the Phase 3 racing layer dependencies underneath the existing dependencies. Ensure all features match the requirements exactly.

Add these crates with their configurations:  
\- gilrs (version 0.11, feature: "serde")  
\- sdl2 (version 0.35, features: \["serde", "force-feedback"\])  
\- udev (version 0.8, feature: \["serde"\])  
\- approx (version 0.5)  
\- rand (version 0.8)  
\- paste (version 1.0)  
\- navmesh (version 0.8)  
\- ordered-float (version 4.0)  
\- ron (version 0.8)  
\- typetag (version 0.2)

## **🚗 Step 2 — Vehicle Physics Foundations**

### **Prompt 2: Core Physics Module Mapping**

**File to open/create:** src/physics/vehicle/mod.rs

Plaintext  
Create the module entry point for our custom vehicle physics.   
Expose the following submodules publicly:  
\- components  
\- data  
\- suspension  
\- tires  
\- drivetrain  
\- aerodynamics  
\- system  
\- utils

Re-export these specific types for the engine API surface:  
\- From components: Vehicle, Wheel, DriveType, Gear, TransmissionType  
\- From data: VehicleConfig, PhysicsConstants, SurfaceType  
\- From system: VehiclePhysicsSystem

### **Prompt 3: Environment Constants & Vehicle Configuration Structs**

**File to open/create:** src/physics/vehicle/data.rs

Plaintext  
Implement the data models and environment structures for our vehicle physics simulation using standard primitive types (no custom macro overrides).

1\. Define a \`PhysicsConstants\` struct:  
   \- fields: \`gravity\` (f32), \`timestep\` (f32), \`air\_density\` (f32)  
   \- Implement \`Default\` matching sea level metrics (9.81, 1/60.0, 1.225).

2\. Define a \`SurfaceType\` enum (Derive Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash):  
   \- Variants: Tarmac, WetTarmac, Gravel, Grass, Ice, Sand, Kerb  
   \- Implement methods: \`friction\_coeff(\&self) \-\> f32\` and \`rolling\_resistance(\&self) \-\> f32\` with specific scalar values matching standard surface friction behavior.

3\. Define \`DriveType\` (FrontWheelDrive, RearWheelDrive, AllWheelDrive) and \`TransmissionType\` (Manual, Automatic, Sequential).

4\. Define a \`Gear\` struct holding a \`ratio: f32\` and \`name: &'static str\`.

5\. Define a robust \`VehicleConfig\` struct tracking chassis metrics (mass, inertia, COM, track/wheelbase lengths, drag coefficients), engine performance bands (torque, power curves, RPM spectrum), transmission parameters, wheel dimensions, tire grip data, suspension mechanics, and steering constraints. Implement a sensible sports car configuration as its \`Default\` trait implementation.

### **Prompt 4: ECS Components for Vehicles and Wheels**

**File to open/create:** src/physics/vehicle/components.rs

Plaintext  
Create the structural ECS components representing our vehicle and individual wheels. Make sure they integrate cleanly with our prelude imports.

1\. Create a \`Wheel\` struct component containing fields for:  
   \- Configuration (local position vector, radius, width, steering/driven flags)  
   \- Suspension states (current travel displacement, spring force scalar, damper force scalar, last recorded position vector)  
   \- Tire state dynamics (angular velocity, longitudinal slip, lateral slip, current friction coefficient, associated \`SurfaceType\`)  
   \- Contact manifold data (is\_contact boolean, contact point, surface normal vector, penetration depth)  
   \- Implement a constructor \`new(local\_pos: Vec3, steering: bool, driven: bool)\` that initializes these metrics to static rest values.

2\. Create a \`Vehicle\` struct component acting as the runtime hub:  
   \- Include tracking configurations, a collection of \`wheels: Vec\<Wheel\>\`, engine states (RPM, torque), user controller inputs (throttle, brake, handbrake, steering), transmission state indices, motion states (velocity, acceleration, speed scalar), and a \`rigid\_body\_handle\` for Rapier3D linking.  
   \- Implement a constructor \`new(config: VehicleConfig, rigid\_body: rapier3d::dynamics::RigidBodyHandle)\` which automatically initializes 4 distinct default wheels array positions relative to the configuration's front/rear tracks and wheelbase specifications.

## **📐 Step 3 — Subsystem Calculators**

### **Prompt 5: Raycast Suspension Simulation**

**File to open/create:** src/physics/vehicle/suspension.rs

Plaintext  
Write the raycast suspension update logic function \`update\_suspension(vehicle: \&mut Vehicle, world\_transform: \&Transform, physics\_world: \&mut PhysicsWorld, dt: f32)\`.

For each wheel on the vehicle:  
1\. Calculate its baseline world space starting coordinate using the parent transform.  
2\. Formulate a downward raycast through Rapier3D's query pipeline spanning the length of the wheel radius combined with maximum suspension travel.  
3\. If no intersection occurs, reset its travel metrics, spring forces, and damping forces to 0.0.  
4\. If an intersection is detected:  
   \- Map out contact depths and derive compression travel values.  
   \- Calculate Spring Force using Hooke's Law ($F \= \-k \\cdot x$).  
   \- Determine localized dampening velocity along the vertical axis, implementing independent profiles for compression versus rebound dynamics ($F \= \-c \\cdot v$).  
   \- Calculate total combined vertical vector adjustments and apply it directly as a linear force at that specific coordinate against the Rapier3D rigid body container via \`apply\_force\_at\_point\`.

### **Prompt 6: Tire Friction Model (Pacejka Curve)**

**File to open/create:** src/physics/vehicle/tires.rs

Plaintext  
Implement a simplified Pacejka Magic Formula tire system to process handling forces.

1\. Write a helper function \`longitudinal\_force(slip: f32, load: f32, friction: f32) \-\> f32\`:  
   \- Calculate longitudinal wheel force based on slip behavior relative to a peak slip threshold (0.15), stiffness profile (10.0), and shape parameter (1.8).  
2\. Write a helper function \`lateral\_force(slip\_angle: f32, load: f32, friction: f32) \-\> f32\`:  
   \- Compute cornering friction threshold values over standard slip angles using distinct lateral constraints.  
3\. Write the primary execution system hook \`update\_tires(vehicle: \&mut Vehicle, transform: \&Transform, physics\_world: \&mut PhysicsWorld, dt: f32)\`:  
   \- For every wheel currently establishing ground contact, break down the rigid body's relative linear velocity components into distinct directional vectors (Forward vs. Lateral).  
   \- Compute slip ratios, capture aggregate vertical load distribution values from suspension states, evaluate combined lateral and longitudinal friction outputs, convert them back into unified world vectors, and register them back to the Rapier chassis object via \`apply\_force\_at\_point\`.

### **Prompt 7: Main Physics Integration Pipeline**

**File to open/create:** src/physics/vehicle/system.rs

Plaintext  
Build the orchestration system \`VehiclePhysicsSystem\` ensuring it complies with our engine's base \`System\` trait format.

In the execution logic loop:  
1\. Pull fixed delta execution step rates and retrieve reference frames to the shared physics context container.  
2\. Establish a query loop isolating all pairs of mutable \`Vehicle\` components alongside their spatial \`Transform\` components.  
3\. Execute the physics calculator steps in the following sequence:  
   \- \`update\_suspension\`  
   \- \`update\_engine\`  
   \- \`update\_transmission\`  
   \- \`update\_drivetrain\_forces\`  
   \- \`update\_tires\`  
   \- \`update\_aerodynamics\`  
   \- \`update\_steering\`  
4\. Post-execution, safely update the internal states of the engine components (linear velocities, angular variations, scalar calculations) based on current outputs pulled straight from Rapier's internal definitions.

## **🎮 Step 4 — Input & Hardware Layer**

### **Prompt 8: Force Feedback Subsystem**

**File to open/create:** src/input/racing/force\_feedback.rs

Plaintext  
Develop a Force Feedback control module using SDL2 haptic subsystem layers.

1\. Provide an explicit \`FfEffectType\` enum identifying different physical attributes:  
   \- \`Spring\` { stiffness: f32, deadzone: f32 }  
   \- \`Damper\` { coefficient: f32 }  
   \- \`Friction\` { strength: f32 }  
   \- \`Constant\` { force: f32, direction: f32 }  
   \- \`Vibration\` { frequency: f32, amplitude: f32, duration: f32 }  
2\. Build an \`FfEffect\` holding this wrapped profile structure.  
3\. Create a \`ForceFeedbackManager\` managing active haptic instances via unique numerical ID containers:  
   \- Implement \`new(device\_index: u32) \-\> EngineResult\<Self\>\` setting up automated operational connections down into underlying SDL2 sub-joystick systems.  
   \- Implement \`update\_from\_vehicle(\&mut self, vehicle: \&Vehicle, dt: f32) \-\> EngineResult\<()\>\` which clears active queues and loads 3 unified runtime conditions:  
     a) A progressive centering spring effect scaled alongside forward velocity metrics.  
     b) A damper resistance block tracking high velocity rotational changes.  
     c) A dynamic chatter vibration block fired when the wheel records severe slip differentials.

### **Prompt 9: Unified Input Strategy Orchestration**

**File to open/create:** src/input/racing/racing\_input.rs

Plaintext  
Create the automated control multiplexing framework tracking both basic input interfaces and specialized racing wheels.

1\. Design a \`RacingInputState\` struct component maintaining numeric floating fields (0.0 to 1.0 limits) tracking throttle, brake, clutch inputs, directional steering states (-1.0 to 1.0 ranges), alongside boolean flags for shifting gears, handbrake engagement, and resetting systems.  
2\. Build a dedicated \`RacingInputSystem\` component:  
   \- During \`initialize\`, try to detect an active racing wheel controller. If found, instantiate a new \`ForceFeedbackManager\`.  
   \- Within \`run\`, if a wheel controller is actively registered, route logic down to a private \`read\_wheel\_input\` routine. Otherwise, fall back to standard \`read\_keyboard\_input\` mapping strategies using keyboard layout assumptions.  
   \- Map these input states back to the current active \`Vehicle\` components in the scene graph while driving the updated delta parameters into the active FFB framework instance.

## **🏁 Step 5 — Racing Logic & Gameplay Template**

### **Prompt 10: Race Metadata and State Models**

**File to open/create:** src/racing\_lib/components.rs

Plaintext  
Establish core gameplay component definitions for managing standard track event conditions.

Implement the following component structs cleanly:  
1\. \`RaceInfo\`: Tracks target string metrics for tracks, lap configuration caps, current iteration phases, state machines (\`Starting\`, \`Racing\`, \`Finished\`, \`Disqualified\`), and elapsed time metrics.  
2\. \`LapTimer\`: Tracks granular lap metrics, option-wrapped tracking entries for historical session data, and sectors tracking arrays.  
3\. \`Checkpoint\`: Captures tracking indices, localized transform matrices, radial intersection thresholds, and special categorization parameters indicating if the instance acts as the start/finish loop line.  
4\. \`AiDriver\`: Holds basic tuning parameters (skill level scales, current navigation targets, aggressiveness offsets) alongside its own driver-assigned \`Vehicle\` simulator proxy footprint.  
5\. \`RacingCamera\`: Monitors camera view states (\`Cockpit\`, \`Chase\`, \`Orbit\`, \`RearView\`) pointing directly toward target identifier hooks while evaluating damping smoothing interpolation boundaries.

### **Prompt 11: Main Timing, Tracking, and Position System**

**File to open/create:** src/racing\_lib/race\_manager.rs

Plaintext  
Implement the analytical state machine logic tracking overall player standing structures.

Create a \`RaceManagerSystem\` implementing the base \`System\` trait wrapper:  
1\. Process timing sequences across the active \`RaceInfo\` entity blocks. Manage transitions from a 3.0-second countdown period into an active status flag, and switch states to complete when a competitor clears lap rules.  
2\. Write an internal sorting method named \`update\_positions(\&self, world: \&World)\`:  
   \- Query all competitors tracking their active \`LapTimer\` information combined with their global physical matrix coordinates.  
   \- Sort competition vectors using an calculated progression scoring standard: \`laps completed \+ distance along current lap segment\`.  
   \- Store these position indexes back as an accessible system resource matrix.

### **Prompt 12: Multi-mode Camera Controller**

**File to open/create:** src/racing\_lib/camera.rs

Plaintext  
Implement a specialized multi-view \`RacingCameraSystem\` matching our game system design guidelines.

Construct safe conditional processing blocks wrapping around the following distinct perspective settings:  
\- \`CameraMode::Cockpit\`: Snaps cameras inline directly inside cabin bounding zones, fixing rotations squarely along the car frame alignment forward orientations.  
\- \`CameraMode::Chase\`: Implements smooth linear target vector point lerping tracking comfortably behind target trailing coordinates. Implement dynamic field-of-view modification shifts that slightly widen the viewport view frustum as vehicle velocity increases.  
\- \`CameraMode::Orbit\`: Applies smooth angular geometric rotational loops wrapping slowly around parent vectors using active session timer clocks.  
\- \`CameraMode::RearView\`: Snaps camera view planes pointing backwards out from the rear of the vehicle.

### **Prompt 13: Functional Game Loop Integration Sandbox Template**

**File to open/create:** examples/racing\_game.rs

Plaintext  
Build a standalone executable environment tracking our entire Phase 3 feature footprint as a working integration proof-of-concept template.

In an async main loop entry wrapper:  
1\. Spin up an engine shell configuring typical display resolution rules under the title "RustyRacer — Racing Demo".  
2\. Programmatically scatter 4 sequential \`Checkpoint\` component definitions into the world scene graph layout creating a virtual tracking circuit.  
3\. Construct a standard player \`Vehicle\` rig by initializing Rapier3D rigid body configurations and binding a simple cuboid collider shape directly underneath it.  
4\. Spawn an engine camera instance initializing it into standard \`CameraMode::Chase\` operations tracking the newly spawned vehicle asset.  
5\. Manually register our system architecture types into our pipeline scheduler queues:  
   \- \`VehiclePhysicsSystem\`  
   \- \`RacingInputSystem\`  
   \- \`CheckpointSystem\`  
   \- \`LapTimerSystem\`  
   \- \`RaceManagerSystem\`  
   \- \`RacingCameraSystem\`  
   \- \`RacingHudSystem\`  
6\. Kick off standard continuous engine loop operations using \`engine.run()\`.  
