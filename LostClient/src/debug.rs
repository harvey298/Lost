use bevy::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin};

use crate::{game::{util::PlayerBody, camera::CameraState}, net::util::SpawnedEntity};

pub struct Debug;

impl Plugin for Debug {
    fn build(&self, app: &mut App) {
    
        app
        .add_plugin(WorldInspectorPlugin::default())

        .register_type::<PlayerBody>()

        .register_type::<CameraState>()

        .register_type::<SpawnedEntity>()

        ;
    }
}
