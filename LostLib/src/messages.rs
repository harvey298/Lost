//! The connect handshake:
//! Client      -> Server      -> Client   -> Server
//! ServerReady -> ClientReady -> Desynced -> Sync

//! In game data transfer
//! 
//! Client         -> Server
//! MovementUpdate -> Ack

//! In game data transfer (Server declines update)
//! 
//! Client         -> Server
//! MovementUpdate -> Sync

//! Client changes class 
//! 
//! Client      -> Server
//! ChangeClass -> Sync

//! Client changes class (Server declines update)
//! 
//! Client      -> Server
//! ChangeClass -> Ack

use serde::{Deserialize, Serialize};

pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;

use crate::defaults::{self, RawEntity, CurrentActions, WorldOject};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    /// A simple Ping to the server
    Ping,

    /// Checks to see if the server is ready
    ServerReady { username: String, id: String },

    /// Attempt to change class, the server will respond with a sync packet
    /// If the change is allowed the sync packet will contain the relevent data
    ChangeClass { class: defaults::Class },

    /// If the client thinks they are de-synced send this packet
    /// to get a sync packet, if no sync packet is recived send a ping
    /// if no ping is recived drop the connection
    /// otherwise re-send a sync packet
    /// 
    /// Also sent as the hand-shake
    DeSynced,

    /// Disconnect from the server safely
    Disconnect { reason: Option<String> },

    /// Sends a packet to update player pos
    MovementUpdate { pos: Vec3, direction: Vec3, actions: CurrentActions},
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Response to the ping
    Pong,

    /// A response to Server Ready
    /// 
    /// Contains player data
    /// The client is expected to respond with a de-sync packet otherwise the client is disconnected due to handshake failure
    ClientReady { server_ready: bool, server_client_id: u64 },

    /// A sync packet
    /// The server is the authority, so the client will listen
    Sync { player_data: defaults::Player, living_entites: defaults::LivingEntities },

    /// Send an Acknowledge to certain responses so the client doesn't timeout
    Ack,

    /// Kick a player from the server
    Kick { reason: String },

    /// Force the client to spawn an entity!
    /// 
    /// Clients can re-sync after an entity spawn
    EntitySpawn { r#type: defaults::EntityType, data: RawEntity },

    /// Force the client to de-spawn an entity
    EntityDespawn { r#type: defaults::EntityType, data: RawEntity },

    /// its like sync but light-weight and for 1 entity
    EntityChange { data: RawEntity },

    /// its like sync but light-weight and for 1 entity
    PlayerChange { id: u64, client_assigned_id: String, pos: Vec3 },

    /// Updates a world object, if a objects lifetime is 0, then the object is meant to die
    WorldObjectUpdate { data: WorldOject },
}