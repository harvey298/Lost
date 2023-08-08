use bevy::{prelude::{Vec3, Resource, Entity, Component}, utils::HashMap, reflect::Reflect};
use crossbeam_channel::{Sender, Receiver};
use lost_lib::defaults::Player;

#[derive(Resource)]
pub struct PlayerPosSender {
    pub sender: Sender<Vec3>
}

#[derive(Resource)]
pub struct PlayerPosReciver {
    pub reciver: Receiver<Vec3>
}

#[derive(Resource)]
pub struct PlayerMoveSender {
    pub sender: Sender<(String,Vec3)>
}

#[derive(Resource)]
pub struct PlayerMoveReciver {
    pub reciver: Receiver<(String,Vec3)>
}

#[derive(Resource)]
pub struct GameReady {
    pub ready: bool
}

#[derive(Resource)]
pub struct Players {
    pub list: HashMap<String, Player>,
    pub body: HashMap<String, Entity>,
}

#[derive(Component, Reflect)]
pub struct PlayerBody {
    pub id: String
}