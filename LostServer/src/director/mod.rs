mod combat;
mod world;
mod defaults;

use bevy::prelude::*;

use crate::structs::{ PlayerActionChannel};

use lost_lib::defaults::PacketChannel;

pub struct Director;

impl Plugin for Director {
    fn build(&self, app: &mut App) {
        
        app.insert_resource(PacketChannel::default());
        app.insert_resource(PlayerActionChannel::default());

        app.add_plugin(world::WorldDirector);
        app.add_plugin(combat::CombatDirector);

    }
}

#[derive(Component)]
pub struct SpawnedEntity {
    pub id: u64,
}