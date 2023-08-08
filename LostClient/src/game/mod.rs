use bevy::{prelude::*, utils::HashMap};
// use bevy_rapier3d::{rapier::prelude::{RigidBodyBuilder, ColliderBuilder}, prelude::{RigidBody, GravityScale}};
use crossbeam_channel::unbounded;
use lost_lib::{WORLD_PLANE_VEC, CLIENT_Y_SYNC};

use crate::{GameStates, net::{client_connection, util::entity_handle}, AllEntites};

use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

use self::util::{PlayerPosSender, PlayerPosReciver, GameReady, Players, PlayerMoveReciver, PlayerMoveSender, PlayerBody};

pub mod util;
pub mod camera;

use bevy_rapier3d::prelude::*;
pub struct Game;

#[derive(Debug, Component)]
pub struct Camera3d;

#[derive(Debug, Component)]
pub struct Body;

impl Plugin for Game {
    fn build(&self, app: &mut App) {

        app.add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default());

        app
        .add_system(load.in_schedule(OnEnter(GameStates::InGame)))

        .add_system(client_connection.in_set(OnUpdate(GameStates::InGame)));

        // .add_system(modify_body_gravity_scale.in_schedule(OnExit(GameStates::InGame)));

        // app.add_system_set(
        //     SystemSet::on_enter(GameStates::InGame)
        //         .with_system(load)
        // );
        
        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(client_connection)
        // );

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(modify_body_gravity_scale)
        // );

        // For sending movement updates
        let (tx, rx) = unbounded();

        let tx = PlayerPosSender{ sender: tx };

        app.insert_resource(tx);

        let rx = PlayerPosReciver{ reciver: rx };

        app.insert_resource(rx);


        // For updating positions
        let (tx, rx) = unbounded();

        let tx = PlayerMoveSender{ sender: tx };

        app.insert_resource(tx);

        let rx = PlayerMoveReciver{ reciver: rx };

        app.insert_resource(rx);

        // For All players
        let players = HashMap::new();
        let bodies = HashMap::new();
        app.insert_resource(Players{ list: players, body: bodies });

        // Storage for all entites
        // TODO: Remove
        app.insert_resource(lost_lib::defaults::LivingEntities::default());

        app.insert_resource(GameReady{ready: false});

        app

        .add_system(entity_handle.in_set(OnUpdate(GameStates::InGame)))

        .add_system(camera_control.in_set(OnUpdate(GameStates::InGame)))

        .add_system(update_player_entities.in_set(OnUpdate(GameStates::InGame)));

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(camera_control)
        // );

        // // Update players
        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(update_player_entities)
        // );

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(entity_handle)
        // );


    }
}

fn load(mut commands: Commands, assets: Res<AssetServer>, mut ent_ids: ResMut<AllEntites>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_ready: ResMut<GameReady>) {

    let mut new_plane = WORLD_PLANE_VEC;
    new_plane.y += CLIENT_Y_SYNC as f32;

    let new_light = Vec3{ x: 4.0, y: (8.0+CLIENT_Y_SYNC as f32), z: 4.0 };

    // Plane
    commands.spawn(RigidBody::Dynamic)
    .insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0, ..Default::default() })), // TODO: Check this out
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(new_plane),
        ..default()
    }).insert(Ccd::enabled());

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(new_light),
        ..default()
    });

    info!("Spawning Player body");

    // cube
    // commands.spawn(RigidBody::Dynamic)
    // .insert(TransformBundle::from(Transform::from_xyz(5.0, 5.0, 0.0)))
    // .insert(Velocity {
    //     linvel: Vec3::new(1.0, 2.0, 3.0),
    //     angvel: Vec3::new(0.2, 0.0, 0.0),
    // })
    // .insert(GravityScale(0.5))
    // .insert(Sleeping::disabled())
    // .insert(Ccd::enabled());


    // Camera
    commands.spawn(RigidBody::Dynamic)
        .insert(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(0.0, (1.0 + CLIENT_Y_SYNC as f32), 0.0),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.0, (1.0 + CLIENT_Y_SYNC as f32), 0.0) // TODO: Check this out
        )).insert(Camera3d)

        .insert(GravityScale(0.5))
        .insert(ColliderMassProperties::Density(2.0));


}

fn camera_control(mut cameras: Query<&Transform, With<Camera3d>>, mut tx: ResMut<PlayerPosSender>) {

    // info!("Sending data");

    match cameras.get_single_mut() {
        Ok(cam) => {
            let x = cam.translation.x;
            let y = cam.translation.y;
            let z = cam.translation.z;

            let pos = Vec3::new(x, y, z);

            tx.sender.try_send(pos).unwrap();
        },
        Err(_) => todo!(),
    }

    // while let Ok(cam) = cameras.get_single_mut() {

    //     let x = cam.translation.x;
    //     let y = cam.translation.y;
    //     let z = cam.translation.z;

    //     let pos = Vec3::new(x, y, z);

    //     tx.sender.try_send(pos).unwrap();
    // }
    
}

fn update_player_entities(mut bodies: Query<(&mut Transform, &PlayerBody)>, mut update: ResMut<PlayerMoveReciver>) {
    // info!("Checking for player update!");

    match update.reciver.try_recv() {
        Ok(msg) => {

            let x = msg.1.x;
            let y = msg.1.y;
            let z = msg.1.z;
        
            for (mut body, id) in bodies.iter_mut() {
        
                // info!("{} | {}",id.id, msg.0);
                if id.id != msg.0 { continue; }

                // info!("Moving");
        
                body.translation.x = x;
                body.translation.y = y;
                body.translation.z = z;
        
            }

        },
        // TODO: Fix this warning
        Err(e) => {  },
    }



}

// fn modify_body_gravity_scale(mut grav_scale: Query<&mut GravityScale>) {
//     for mut grav_scale in grav_scale.iter_mut() {
//         grav_scale.0 = 2.0;
//     }
// }