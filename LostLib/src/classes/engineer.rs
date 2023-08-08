use bevy::prelude::*;

use crate::defaults::PacketChannel;

pub const ABILITY_1_COST: usize = 10;

#[macro_export]
macro_rules! DoEngineerAbility3 {
    () => { warn!("Engineer Ability 3 Not Setup Correctly!") };

    ($commands:expr, $player:expr, $packet_channel:expr) => {
        info!("Fortifcation Protcol Activated!");

        $commands.spawn(Collider::cuboid(1.0, 5.0, 1.0)).insert(TransformBundle::from(Transform::from_translation($player.loc)));
    };

}