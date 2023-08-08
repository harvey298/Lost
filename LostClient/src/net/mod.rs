use std::{net::{UdpSocket, SocketAddr}, time::SystemTime};
use bevy::prelude::*;

use bevy_renet::renet::{RenetClient, ConnectionConfig, DefaultChannel};
use local_ip_address::local_ip;
use lost_lib::{messages::{ClientMessage, ServerMessage}, defaults::{Player, Class, LivingEntities, Action, CurrentActions}};
use smooth_bevy_cameras::LookTransform;

use crate::{GameStates, game::util::{PlayerPosReciver, GameReady, Players, PlayerBody, PlayerMoveSender}};

use bevy_text_mesh::prelude::*;

use self::util::{Me, EnemySpawnChannels};

pub mod util;

pub fn create_renet_client() -> RenetClient {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let client_id = current_time.as_millis() as u64;

    let mut client = RenetClient::new(ConnectionConfig::default());
    client
}

pub fn client_start_connection(mut client: ResMut<RenetClient>, mut game_state: ResMut<State<GameStates>>, mut commands: Commands,
    mut my_player: ResMut<Me>,
) {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();

    let name = "Dev".to_string();
    let id = format!("dev-dev-dev-dev-{n1}");

    my_player.id = id.to_string().clone();
    my_player.username = name.to_string().clone();

    let message = ClientMessage::ServerReady { username: name.clone(), id: id.clone() };

    let message = bincode::serialize(&message).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, message);

    let player = Player{ name, id: id.clone(), class: Class::Fighter, loc: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, health: 0, inv: Vec::new(), direction: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, client_assinged_id: id.clone(), resource_amount: 0 };

    commands.insert_resource(player);

    // game_state.push(GameStates::InGame);
    
    info!("Sent Handshake request");
}


pub fn client_connection(mut client: ResMut<RenetClient>, keyboard: Res<Input<KeyCode>>, mut commands: Commands, mut player: ResMut<Player>,
    mut rx: ResMut<PlayerPosReciver>,

    mut player_move_q: ResMut<PlayerMoveSender>,
    
    mut game_ready: ResMut<GameReady>,

    // For Entity spawning
    // TODO: Add a channel to cause spawning
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,

    // For ignoring broadcasts about me
    mut my_player: ResMut<Me>,

    // Everyone but me :( (and me i think)
    mut all_ents: ResMut<LivingEntities>,

    // Everyone and me!
    mut all_players: ResMut<Players>,

    // Asset server
    asset_server: Res<AssetServer>,

    // Mouse buttons
    buttons: Res<Input<MouseButton>>,

    // Entity Spawner
    enemy_spawner: Res<EnemySpawnChannels>,

    mut camera: Query<(&mut LookTransform, &mut Transform), With<Camera3d>>,
) {
    // info!("Net tick");
    
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

    if keyboard.just_pressed(KeyCode::P) {
        let ping_message = bincode::serialize(&ClientMessage::DeSynced).unwrap();
        client.send_message(DefaultChannel::ReliableOrdered, ping_message);
        info!("Re-Syncing");
    }

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap_or(ServerMessage::Pong);
        match server_message {
            ServerMessage::Pong => {
                info!("Got pong!");
            }

            ServerMessage::ClientReady { server_ready, server_client_id } => {
                if !server_ready { client.disconnect(); return }
                info!("The Server has invited me! yay");

                my_player.server_id = server_client_id;

                let message = bincode::serialize(&ClientMessage::DeSynced).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, message);
                info!("Syncing with Server!");
            },
            ServerMessage::Sync { player_data, living_entites } => {
                // TODO Sync

                info!("Synced with server! Results:");

                info!(" Class: {}!",player_data.class);
                info!(" Health: {}!",player.health);

                let fresh_players = living_entites.players.clone();

                info!(" Enemies: {}",living_entites.enemies.len());

                *all_ents = living_entites.clone();

                // Enemy spawner
                for (id, enemy) in living_entites.enemies {

                    enemy_spawner.sender.send((id, enemy)).unwrap();
                    // let pos = enemy.pos;

                    // let ent = commands.spawn(PbrBundle {
                    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    //     transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                    //     ..Default::default()
                    // });
                }
                
                // Player spawner
                for (server_id,fresh_player) in fresh_players {

                    let new_id = fresh_player.client_assinged_id.clone();

                    if new_id == my_player.id { continue; }
                    info!("Updating players!");

                    // Name Tag spawn

                    let username = fresh_player.name.clone();
                    // TODO: Name tag on player
                    let name_tag = commands.spawn(TextMeshBundle {
                        // \nClass: Black Person\nHealth: 100
                        text_mesh: TextMesh::new_with_color(&username, font.clone(), Color::rgb(1., 1., 0.)),
                        transform: Transform::from_xyz(-1., 1.75, 0.),
                        ..Default::default()
                    });

                    let name_tag_id = name_tag.id();

                    if !all_players.list.contains_key(&new_id) {

                        let mut ent = commands.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(fresh_player.loc.x, fresh_player.loc.y, fresh_player.loc.z),
                            ..Default::default()
                        });

                        // ent.with_children(|parent | {

                        // });

                        ent.add_child(name_tag_id.clone());

                        ent.insert(PlayerBody{ id: new_id.clone() });

                        let ent = ent.id();

                        all_players.list.insert(new_id.clone(), fresh_player);
                        all_players.body.insert(new_id, ent);
                        continue;
                    }

                    let body = all_players.body.get(&new_id).unwrap();

                    commands.entity(*body).despawn();

                    let mut ent = commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_xyz(fresh_player.loc.x, fresh_player.loc.y, fresh_player.loc.z),
                        ..Default::default()
                    });

                    ent.add_child(name_tag_id.clone());

                    ent.insert(PlayerBody{ id: new_id.clone() });

                    let ent = ent.id();

                    all_players.body.insert(new_id, ent);

                }
            
                *player = player_data;

                *game_ready = GameReady{ ready: true };

            },
            ServerMessage::Ack => {
                // info!("Server ack-ed our req!");
            },
            ServerMessage::Kick { reason } => todo!(),
            
            ServerMessage::EntitySpawn { r#type, data  } => {

                info!("{} | {}",data.id,my_player.id);

                if data.id == my_player.id {
                    info!("Detected New Player has joined the server! I am de-synced");

                    let message = bincode::serialize(&ClientMessage::DeSynced).unwrap();
                    client.send_message(DefaultChannel::ReliableOrdered, message);

                    return
                }

                match r#type {
                    lost_lib::defaults::EntityType::Player => {

                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(data.pos.x, data.pos.y, data.pos.z),
                            ..Default::default()
                        }).insert(PlayerBody{ id: data.id });

                    },
                    lost_lib::defaults::EntityType::Enemy => {
                        enemy_spawner.sender.send((data.clone().id, data.make_alive())).unwrap();

                    },
                    lost_lib::defaults::EntityType::Other => todo!(),
                }

            },
            ServerMessage::PlayerChange { id, client_assigned_id, pos } => {
                if client_assigned_id == my_player.id { continue; }
                if id == my_player.server_id { continue; }

                // info!("Causing movement update");

                player_move_q.sender.send((client_assigned_id,pos)).unwrap();
            },
            ServerMessage::EntityDespawn { r#type, data } => {

                match r#type {
                    lost_lib::defaults::EntityType::Player => todo!(),
                    lost_lib::defaults::EntityType::Enemy => {

                        enemy_spawner.sender.send((data.clone().id, data.make_alive())).unwrap();

                    },
                    lost_lib::defaults::EntityType::Other => todo!(),
                }

            },

            ServerMessage::EntityChange { data } => {

                enemy_spawner.sender.send((data.clone().id, data.make_alive())).unwrap();

            },
            ServerMessage::WorldObjectUpdate { data } => todo!(),
        }
    }

    // Movement handler
    if game_ready.ready {
        match rx.reciver.try_recv() {
            Ok(o) => {
                
                let ability1 = KeyCode::Q;
                let ability2 = KeyCode::E;
                let ability3 = KeyCode::R;
                let crouch = KeyCode::LControl;

                let mut buffer = Vec::new();

                if keyboard.just_released(ability1) { buffer.push(Action::Ability1) }
                if keyboard.just_released(ability2) { buffer.push(Action::Ability2) }
                if keyboard.just_released(ability3) { buffer.push(Action::Ability3) }
                if keyboard.just_released(crouch) { buffer.push(Action::Crouch) }

                if buttons.pressed(MouseButton::Right) { buffer.push(Action::AltFire) }
                if buttons.pressed(MouseButton::Left) { buffer.push(Action::PrimaryFire) }

                let target = camera.get_single().unwrap().0.target;

                // info!("{:?}",target);

                let message = bincode::serialize(&ClientMessage::MovementUpdate { pos: o, direction: target, actions: CurrentActions { actions: buffer } }).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, message);
            },
            Err(e) => {  }, // warn!("{:?}",e)
        }
    }

    // info!("End net tick");
}