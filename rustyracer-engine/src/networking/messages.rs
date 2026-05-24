//! Network Protocol & Channel Mappings

use serde::{Serialize, Deserialize};
use renet::ChannelConfig;
use glam::{Vec3, Quat};

/// Network message types for client/server communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // Handshake messages
    Handshake {
        client_id: u64,
        version: String,
    },
    HandshakeAccepted {
        server_id: u64,
    },
    HandshakeRejected {
        reason: String,
    },
    
    // Player state synchronization
    PlayerState {
        frame: u32,
        position: Vec3,
        rotation: Quat,
        velocity: Vec3,
        angular_velocity: Vec3,
        rpm: f32,
        gear: i32,
        throttle: f32,
        brake: f32,
        steering: f32,
        handbrake: bool,
    },
    
    // World state updates
    WorldState {
        frame: u32,
        entities: Vec<EntityState>,
    },
    
    // Chat messages
    ChatMessage {
        text: String,
        sender_id: u64,
    },
}

/// Entity state for network synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub entity_id: u64,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
}

/// Network channel configurations
pub const CHANNELS: &[ChannelConfig] = &[
    // Channel 0: ReliableOrdered for system orchestration
    ChannelConfig {
        channel_id: 0,
        max_memory_usage_bytes: 1024 * 1024,
        reliability: renet::ChannelReliability::ReliableOrdered,
    },
    // Channel 1: UnreliableSequenced for time-critical physics inputs
    ChannelConfig {
        channel_id: 1,
        max_memory_usage_bytes: 2 * 1024 * 1024,
        reliability: renet::ChannelReliability::UnreliableSequenced,
    },
    // Channel 2: Unreliable for raw distribution arrays
    ChannelConfig {
        channel_id: 2,
        max_memory_usage_bytes: 512 * 1024,
        reliability: renet::ChannelReliability::Unreliable,
    },
];

/// Get the channel ID for reliable messages
pub const fn reliable_channel() -> u8 {
    0
}

/// Get the channel ID for physics input messages
pub const fn physics_input_channel() -> u8 {
    1
}

/// Get the channel ID for unreliable broadcast messages
pub const fn unreliable_channel() -> u8 {
    2
}
