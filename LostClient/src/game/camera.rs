use std::{thread::{self, spawn}, time::{Duration, Instant}, sync::Arc};

use bevy::{input::mouse::MouseMotion, window::PrimaryWindow};

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::window::CursorGrabMode;
use smooth_bevy_cameras::{LookTransform, LookAngles, controllers::fps::ControlEvent};

use crate::GameStates;

#[derive(Debug, Resource, Reflect)]
pub struct CameraState {
    pub moveable: bool,
}

pub struct FPSCamera;

/// The camera behaves as if its the body
/// Since the client doesn't actually render its own body
/// Other clients to render the body though
/// So this plugin is meant to handle movement
impl Plugin for FPSCamera {
    fn build(&self, app: &mut App) {

        app

        .add_system(cursor_grab_system.in_set(OnUpdate(GameStates::InGame)))

        .add_system(keyboard_input.in_set(OnUpdate(GameStates::InGame)))

        .add_system(gravity_handle.in_set(OnUpdate(GameStates::InGame)));

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(cursor_grab_system)
        // );

        // app.insert_resource(CameraState{ moveable: false });

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(keyboard_input)
        // );

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(gravity_handle)
        // );

        app.insert_resource(Gravity::default());

        // app.add_system_set(
        //     SystemSet::on_update(GameStates::InGame)
        //         .with_system(mouse_movement)
        // );
        
    }
}

fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut camera_state: ResMut<CameraState>,
) {
    let mut window = windows.get_single_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        // window.set_cursor_grab_mode(CursorGrabMode::Confined);

        window.cursor.grab_mode.set(Box::new(CursorGrabMode::Locked));
        window.cursor.visible = false;

        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        // window.set_cursor_grab_mode(CursorGrabMode::Locked);
        // also hide the cursor
        // window.set_cursor_visibility(false);

        camera_state.moveable = !window.cursor.visible;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode.set(Box::new(CursorGrabMode::None));
        window.cursor.visible = true;

        camera_state.moveable = !window.cursor.visible;
    }

    // info!("{}",camera_state.moveable);

}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut camera: Query<(&mut LookTransform, &mut Transform), With<Camera3d>>,
    time: Res<Time>,
    camera_state: Res<CameraState>,
    mut events: EventWriter<ControlEvent>,
    mut jumped: ResMut<Gravity>,
) {

    if !camera_state.moveable { return; }

    let (mut cam, mut transform) = camera.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::Space) && jumped.jumped == false {

        let units = Vec3{ x: 0.0, y: 5.0, z: 0.0 };

        jumped.camera_start_pos = Some(cam.target);

        events.send(ControlEvent::TranslateEye(2.0 * units));
    
        jumped.jumped = true;
        jumped.units_left_to_fall = units;
    }

    if keys.just_pressed(KeyCode::P) {
        // let mut new_cam = cam.bypass_change_detection();

        // transform.translation.y = 10.0;
        // new_cam.eye.y = 2.0;
    }
}

fn gravity_handle(
    time: Res<Time>,
    camera_state: Res<CameraState>,
    mut events: EventWriter<ControlEvent>,
    mut camera: Query<&mut LookTransform, With<Camera3d>>,
    mut jumped: ResMut<Gravity>,
    mut last_time: Local<LocalInstant>
) {

    if last_time.time.is_none() { last_time.time = Some(Instant::now()) }
    if jumped.units_left_to_fall.y <= 0.0 { last_time.time = None;jumped.falling = false;jumped.jumped = false; return}

    if last_time.time.unwrap().elapsed() >= Duration::from_millis(900) {

        let units = Vec3 { x: 0.0, y: 5.0, z: 0.0 };

        events.send(ControlEvent::TranslateEye(-(2.0 * units)));
        last_time.time = None;
        jumped.units_left_to_fall.y = 0.0;
    }
    // camera.get_single_mut().unwrap().bypass_change_detection();
   

}

fn mouse_movement(
    mut motion_evr: EventReader<MouseMotion>, 
    mut camera: Query<&mut LookTransform, With<Camera3d>>,
    camera_state: Res<CameraState>,
) {

    if !camera_state.moveable { return; }

    let mut cam = camera.get_single_mut().unwrap();

    for ev in motion_evr.iter() {
        // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

        let mut angles = LookAngles::from_vector(cam.look_direction().unwrap());
        angles.add_pitch(ev.delta.y);
        angles.add_yaw(ev.delta.x);

        cam.target = cam.eye + 1.0 * cam.radius() * angles.unit_vector();
        break
    }    

}

pub struct LocalInstant {
    pub time: Option<Instant>
}
impl Default for LocalInstant {
    fn default() -> Self {
        Self { time: None }
    }
}


#[derive(Debug, Resource)]
pub struct Gravity {
    jumped: bool,
    falling: bool,
    time_left_to_fall: Option<Instant>,
    units_left_to_fall: Vec3,
    camera_start_pos: Option<Vec3>,
}

impl Default for Gravity {
    fn default() -> Self {
        Self { jumped: false, falling: false, time_left_to_fall: None, units_left_to_fall: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, camera_start_pos: None }
    }
}