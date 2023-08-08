use bevy::{prelude::*, utils::{HashMap, Instant}};
use serde::{Deserialize, Serialize};
use crossbeam_channel::{Sender, Receiver, unbounded};

use crate::messages::ServerMessage;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Class {
    Fighter,
    Engineer,
    Chronowatcher,
    Emperor,
    Mutant,
    Harbinger,
    Paladin,
    Mystic,
}

/// Every Item the game current has
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]

pub enum Item {
    Test,    
}

/// Data about the player
#[derive(Debug, Clone, Deserialize, Serialize, Resource, PartialEq)]
pub struct Player {
    /// Player name
    pub name: String,
    /// Unique player id (assinged by the server)
    pub id: String,
    /// The players class
    pub class: Class,
    /// The Players location
    pub loc: Vec3,
    /// The players health
    pub health: isize,
    /// The players inventory
    pub inv: Vec<Item>,
    /// Where the placing is facing
    pub direction: Vec3,
    /// Unique player id (assinged by the client)
    pub client_assinged_id: String,
    /// If classes have a resource, an example would be Engineer's nanobots
    pub resource_amount: usize,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Enemy {
    /// Skeleton Archer
    SkeletonArcher,

    /// Vawntite
    /// TODO: Complete
    Vawnite,

    /// Debug - Also a null type
    Debug,

    /// Filler Object
    /// The client will despawn any entity with the type None 
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AliveEnemy {
    pub id: String,
    pub r#type: Enemy,
    pub pos: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, Clone, Deserialize, Serialize, Resource)]
pub enum EntityType  {
    Player,
    Enemy,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, Resource)]
pub struct RawEntity  {
    pub id: String,
    pub pos: Vec3,
    pub enemy_type: Enemy,
}

#[derive(Debug, Clone, Deserialize, Serialize, Resource, Component)]
pub struct WorldOject  {
    /// The Object ID
    pub id: String,
    /// The Position
    pub pos: Vec3,
    /// The size
    pub size: Vec3,
    /// The Owner is spawned by a player
    pub owner: Option<String>,
    /// If the object has a lifetime
    pub lifetime: Option<u64>,
}

impl RawEntity {
    pub fn make_alive(&self) -> AliveEnemy {

        AliveEnemy{ id: self.id.clone(), r#type: self.enemy_type, pos: self.pos, direction: Vec3 { x: 0.0, y: 0.0, z: 0.0 } }

    }
}

/// TODO: Skins?
#[derive(Debug, Clone, Deserialize, Serialize, Resource, PartialEq)]
pub struct LivingEntities {

    /// ID | Player
    pub players: HashMap<String,Player>,
    /// ID | Enemy
    pub enemies: HashMap<String,AliveEnemy>,
}

impl LivingEntities {
    pub fn default() -> Self{
        Self { players: HashMap::new(), enemies: HashMap::new() }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrentActions {
   pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Action {
    PrimaryFire,
    AltFire,
    Ability1,
    Ability2,
    Ability3,
    Crouch,
}


#[derive(Debug, Clone, Deserialize, Serialize, Component )]
pub struct WorldObject {
    /// World Objects position
    pub pos: Vec3,
    /// ID of the world object
    pub id: String,
    /// If a player can break the world object
    pub breakable: bool,
    /// The Asset ID of the world object
    pub mesh_id: String,
    /// If the world object was spawned by a player this will be the
    /// players server assinged ID
    /// If the world object was spawned by the server this will be the server's IP, Port, ID, and Name all hashed together
    pub owner: String,
}


impl Default for PacketChannel {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { sender: tx, receiver: rx }
    }
}

#[derive(Resource, Clone)]
pub struct PacketChannel {
    pub sender: Sender<ServerMessage>,
    pub receiver: Receiver<ServerMessage>,
}