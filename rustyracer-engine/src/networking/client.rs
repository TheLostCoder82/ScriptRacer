//! Client Transport Interface using renet

use crate::core::error::EngineResult;
use crate::input::racing::RacingInputState;
use renet::{RenetClient, ClientAuthentication, NetcodeClientTransport};
use std::net::UdpSocket;
use std::time::Duration;

/// Game client management struct
pub struct GameClient {
    pub client: RenetClient,
    pub local_id: u64,
    pub server_addr: Option<String>,
    pub connected: bool,
}

impl GameClient {
    /// Create a new game client
    pub fn new(local_id: u64) -> Self {
        let client = RenetClient::new(renet::ConnectionConfig {
            server_channels_count: 3,
            client_channels_count: 3,
            ..Default::default()
        });
        
        Self {
            client,
            local_id,
            server_addr: None,
            connected: false,
        }
    }
    
    /// Connect to a server
    pub fn connect(&mut self, server_addr: &str, player_name: &str) -> EngineResult<()> {
        use std::net::SocketAddr;
        
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let server_addr: SocketAddr = server_addr.parse()?;
        
        let authentication = ClientAuthentication::Unsecure {
            client_id: self.local_id,
            protocol_id: 0,
            server_addr,
            user_data: Some(player_name.as_bytes().to_vec()),
        };
        
        let transport = NetcodeClientTransport::new(
            self.local_id,
            authentication,
            socket,
        )?;
        
        self.server_addr = Some(server_addr.to_string());
        self.connected = true;
        
        // Send handshake
        use crate::networking::messages::NetworkMessage;
        let message = NetworkMessage::Handshake {
            client_id: self.local_id,
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        let bytes = bincode::serialize(&message)?;
        self.client.send_message(crate::networking::messages::reliable_channel(), bytes);
        
        Ok(())
    }
    
    /// Send input to server
    pub fn send_input(&mut self, input: &RacingInputState, frame: u32) -> EngineResult<()> {
        use crate::networking::messages::NetworkMessage;
        
        let message = NetworkMessage::PlayerState {
            frame,
            position: input.position,
            rotation: input.rotation,
            velocity: input.velocity,
            angular_velocity: input.angular_velocity.unwrap_or(glam::Vec3::ZERO),
            rpm: input.rpm,
            gear: input.gear,
            throttle: input.throttle,
            brake: input.brake,
            steering: input.steering,
            handbrake: input.handbrake,
        };
        
        let bytes = bincode::serialize(&message)?;
        self.client.send_message(crate::networking::messages::physics_input_channel(), bytes);
        
        Ok(())
    }
    
    /// Update client and process incoming messages
    pub fn update(&mut self) -> EngineResult<()> {
        // Process incoming packets
        while let Some(message) = self.client.recv_message() {
            // Deserialize and handle message
            if let Ok(msg) = bincode::deserialize::<NetworkMessage>(&message) {
                match msg {
                    NetworkMessage::HandshakeAccepted { server_id } => {
                        log::info!("Connected to server {}", server_id);
                    }
                    NetworkMessage::HandshakeRejected { reason } => {
                        log::error!("Connection rejected: {}", reason);
                        self.connected = false;
                    }
                    NetworkMessage::WorldState { frame, entities } => {
                        // Update local entity states from server
                        for entity_state in entities {
                            // Would update transform components
                            log::debug!("Received world state for frame {}: {} entities", frame, entities.len());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Disconnect from server
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.server_addr = None;
    }
}

use crate::networking::messages::NetworkMessage;
