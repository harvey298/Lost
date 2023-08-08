use bevy::{prelude::{Resource, Vec3}, utils::HashMap};
use crossbeam_channel::{Sender, Receiver, unbounded};
use lost_lib::{defaults::{Class, Player, CurrentActions}, messages::ServerMessage};

#[derive(Resource, Clone)]
pub struct Players {
    pub players: HashMap<String, Player>,
}

impl Players {
    pub fn default() -> Self {

        let map = HashMap::new();

        Players { players: map }
    }

    /// TODO: re-do and make better
    pub fn is_player_connected(&mut self, client_id: u64) -> bool {
        let client_id = format!("{client_id}");

        self.players.contains_key(&client_id)
    }

    pub fn modify_player_class(&mut self, client_id: u64, class: Class) {
        let client_id = format!("{client_id}");        

        let old_player = self.players.get(&client_id);
        let old_player = if old_player.is_none() { return } else { old_player.unwrap() };

        let player = Player{ 
            loc: old_player.loc, 
            class: class, 
            id: client_id.clone(), 
            name: old_player.name.clone(), 
            health: old_player.health, 
            inv: old_player.inv.clone(), 
            direction: old_player.direction,
            client_assinged_id: old_player.client_assinged_id.clone(),
            resource_amount: old_player.resource_amount, };

        self.players.insert(client_id, player);

    }

    pub fn fetch_player(&mut self, client_id: u64) -> Player {
        let client_id = format!("{client_id}");

        let player = self.players.get(&client_id);
        let player = if player.is_none() { todo!("Player not found! - Disconnect client"); } else { player.unwrap() };

        player.clone()
    }

    pub fn modify_player_pos(&mut self, client_id: u64, pos: Vec3, direction: Vec3) -> Option<Player> {
        
        let client_id = format!("{client_id}");        

        let old_player = self.players.get(&client_id);
        let old_player = if old_player.is_none() { return None } else { old_player.unwrap() };

        let player = Player{ 
            loc: pos, 
            class: old_player.class, 
            id: client_id.clone(), 
            name: old_player.name.clone(), 
            health: old_player.health, 
            inv: old_player.inv.clone(), 
            direction: direction,
            client_assinged_id: old_player.client_assinged_id.clone(),
            resource_amount: old_player.resource_amount, };

        self.players.insert(client_id, player.clone());

        return Some(player)
    }

    /// TODO: re-do and make better
    pub fn remove_player(&mut self, client_id: u64) {

        let client_id = format!("{client_id}");

        self.players.remove(&client_id);
    }

    pub fn register_new_player(&mut self, username: &str, id: u64, client_assinged_id: String) -> Player {

        let client_id = format!("{id}");

        let default_pos = Vec3{ x: 0.0, y: 0.5, z: 0.0 };

        let player = Player{ 
            loc: default_pos, 
            class: Class::Engineer, 
            id: client_id.clone(), 
            name: username.clone().to_owned(), 
            health: 100, 
            inv: Vec::new(), 
            direction: default_pos,
            client_assinged_id: client_assinged_id,
            resource_amount: 0, };

        self.players.insert(client_id, player.clone());

        return player;
    }

    pub fn known_players(&mut self) -> usize {
        self.players.len()
    }
}

#[derive(Resource, Clone)]
pub struct CurrentConnections {
    /// Server Id | Client ID
    pub id_maps: HashMap<u64, String>,
}

// #[derive(Resource, Clone)]
// pub struct Player {
//     pub loc: Vec3,
//     pub class: Class,
//     pub client_id: u64,
//     pub name: String,
// }


// #[derive(Resource)]
// pub struct PlayerPosQueneInput {
//     pub sender: Sender<Vec3>
// }

// #[derive(Resource)]
// pub struct PlayerPosQueneOutput {
//     pub reciver: Receiver<Vec3>
// }

#[derive(Resource, Clone)]
pub struct PlayerActionChannel {
    pub sender: Sender<(Player, CurrentActions)>,
    pub receiver: Receiver<(Player, CurrentActions)>,
}

// #[derive(Resource, Clone)]
// pub struct EntitySpawnChannel {
//     pub sender: Sender<ServerMessage>,
//     pub receiver: Receiver<ServerMessage>,
// }

