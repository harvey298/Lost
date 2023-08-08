use bevy::{log::LogPlugin, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use bevy_renet::renet::transport::{ServerAuthentication, ServerConfig};
use director::{Director, SpawnedEntity};
use local_ip_address::local_ip;

use lost_lib::{PORT, PROTOCOL_VERSION, messages::*, defaults::{LivingEntities, EntityType, RawEntity, Enemy}, WORLD_PLANE_VEC, WORLD_PLANE_SIZE };
use structs::{CurrentConnections, PlayerActionChannel};

use lost_lib::defaults::PacketChannel;

mod structs;
mod defaults;
mod director;

#[macro_use]
extern crate lazy_static;

use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

fn create_renet_server() -> RenetServer {
    // let current_time = SystemTime::now()
    //     .duration_since(SystemTime::UNIX_EPOCH)
    //     .unwrap();

    /* Public hosting, requires port forwarding
    let rt = tokio::runtime::Runtime::new().unwrap();
    let public_ip = rt.block_on(public_ip::addr()).unwrap();
    let server_addr = SocketAddr::new(public_ip, 42069);
    */

    // let server_addr = SocketAddr::new(local_ip().unwrap(), PORT.try_into().unwrap());
    // info!("Creating Server! {:?}", server_addr);

    // let server_config =
    //     ServerConfig {max_clients: 64, protocol_id: PROTOCOL_VERSION, public_addr: server_addr, authentication: ServerAuthentication::Unsecure};

    let connection_config = ConnectionConfig::default();

    // let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), PORT.try_into().unwrap());
    // let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    RenetServer::new(connection_config)
}

fn main() {
    let mut app = App::new();

        // .add_plugins(DefaultPlugins.set(LogPlugin {
        //     filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
        //     level: bevy::log::Level::DEBUG,
        // }))

        // DefaultPlugins | MinimalPlugins
        app.add_plugins(DefaultPlugins)
        .add_plugin(Director)
        // .add_plugin(LogPlugin::default())
        .add_plugin(RenetServerPlugin)
        .insert_resource(structs::Players::default())
        .insert_resource(lost_lib::defaults::LivingEntities::default())
        .insert_resource(create_renet_server())
        .add_system(server_events)
        .add_startup_system(add_camera)
        .add_system(server_ping);

        let id_maps = HashMap::new();
        let connections = CurrentConnections{ id_maps: id_maps };
        app.insert_resource(connections);

    app.run();
}

/// Adds the camera & the plane
fn add_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(-Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Plane spawns here
    commands
    .spawn(Collider::cuboid(WORLD_PLANE_SIZE/2.0, 1.0, WORLD_PLANE_SIZE/2.0))
    .insert(TransformBundle::from(Transform::from_translation(WORLD_PLANE_VEC)));
}

fn server_events(mut events: EventReader<ServerEvent>, mut players: ResMut<structs::Players>, mut all_ents: ResMut<LivingEntities>) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id} => info!("Connected {}!", client_id),
            ServerEvent::ClientDisconnected{ client_id, reason } => {
                players.remove_player(*client_id);

                all_ents.players.remove(&format!("{client_id}"));

                info!("Disconnected {}! Players Connected: {}", client_id, players.known_players())
            },
        }
    }
}

fn server_ping(mut server: ResMut<RenetServer>, mut players: ResMut<structs::Players>, mut all_ents: ResMut<LivingEntities>,
    mut connections: ResMut<CurrentConnections>,
    packet_channel: Res<PacketChannel>,
    action_channel: Res<PlayerActionChannel>,
    mut entities: Query<(&SpawnedEntity, &mut Transform)>
) {


    
    

    match packet_channel.receiver.try_recv() {
        Ok(o) => {
            let res = bincode::serialize(&o).unwrap();
            server.broadcast_message(DefaultChannel::ReliableOrdered, res);
        },
        Err(_) => {},
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let client_message = bincode::deserialize(&message).unwrap();
            match client_message {

                ClientMessage::Ping => {
                    info!("Got ping from {}!", client_id);
                    let pong = bincode::serialize(&ServerMessage::Pong).unwrap();
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, pong);
                }
                
                ClientMessage::ServerReady { username, id } => {
                    // TODO: Check banlist here

                    info!("Shaking {}'s hand back!",client_id);

                    let player = players.register_new_player(&username, client_id, id.clone());

                    connections.id_maps.insert(client_id, id.clone());

                    let broadcast_msg = ServerMessage::EntitySpawn { r#type: EntityType::Player, data: RawEntity{ id: id, pos: player.loc, enemy_type: Enemy::Debug } };
                    let broadcast_msg = bincode::serialize(&broadcast_msg).unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, broadcast_msg);

                    all_ents.players.insert(format!("{client_id}"), player);

                    let res = bincode::serialize(&ServerMessage::ClientReady { server_ready: true, server_client_id: client_id }).unwrap();
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, res);
                },

                ClientMessage::ChangeClass { class } => {
                    // TODO: Check if allowed
                    players.modify_player_class(client_id, class);

                    let res = bincode::serialize(&ServerMessage::Ack).unwrap();
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, res);
                },

                ClientMessage::DeSynced => {

                    let player = players.fetch_player(client_id);

                    // all_ents.enemies.get("o").unwrap().pos 

                    for key in all_ents.enemies.clone().keys().into_iter() {

                        let mut enemy = all_ents.enemies.get(key).unwrap().clone();

                        let enemy_id = enemy.id.split(":").last().unwrap();
                        let enemy_id: u64 = enemy_id.parse().unwrap();

                        for enemy_entity in entities.into_iter() {
                            let my_id = enemy_entity.0;

                            if my_id.id != enemy_id { continue; }

                            enemy.pos = enemy_entity.1.translation;
                            break;
                        }
                        all_ents.enemies.insert(key.to_string(), enemy);

                    }

                    // all_ents

                    let res = bincode::serialize(&ServerMessage::Sync { player_data: player, living_entites: all_ents.clone() }).unwrap();
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, res);
                    // info!("Rendering World For client {}",client_id);
                },

                ClientMessage::Disconnect { reason } => {

                    server.disconnect(client_id);
                    info!("disconnecting client {} with reason {:?}", client_id, reason);
                },

                ClientMessage::MovementUpdate { pos, direction, actions } => {
                    // TODO: anti-cheat
                    if !connections.id_maps.contains_key(&client_id) { server.disconnect(client_id); return }

                    // info!("{:?}",actions);

                    let player = players.modify_player_pos(client_id, pos, direction);

                    let client_assinged_id = connections.id_maps.get(&client_id).unwrap();
                    
                    let broadcast_msg = ServerMessage::PlayerChange { id: client_id, client_assigned_id: client_assinged_id.to_string(), pos: pos };

                    let broadcast_msg = bincode::serialize(&broadcast_msg).unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, broadcast_msg);

                    if actions.actions.len() != 0 && player.is_some() {

                        let player = player.unwrap();

                        action_channel.sender.send((player, actions)).unwrap();
                    }

                    let res = bincode::serialize(&ServerMessage::Ack).unwrap();
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, res);
                },

            }
        }
    }
}