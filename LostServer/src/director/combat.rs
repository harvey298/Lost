//! Controls How combat works and calcualtes dmg, etc
//! 

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use lost_lib::{defaults::{LivingEntities, EntityType, RawEntity, Enemy}, messages::ServerMessage, DoEngineerAbility3, HandleAbility3};

use lost_lib::defaults::PacketChannel;

use crate::structs::{PlayerActionChannel};

pub struct CombatDirector;

impl Plugin for CombatDirector {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default());
        app.add_system(user_action);
        
    }
}

fn user_action(
    action_channel: Res<PlayerActionChannel>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut all_ents: ResMut<LivingEntities>,
    packet_channel: Res<PacketChannel>,
) {

    match action_channel.receiver.try_recv() {
        Ok(o) => {

            let player = o.0;
            let actions = o.1;
            let player_loc= player.loc;
            let player_looking = player.direction;

            for action in actions.actions {

                match action {
                    // Ray Tracing code
                    lost_lib::defaults::Action::PrimaryFire => {
                        let max_toi = Real::MAX;
                        let solid = true;
                        let filter = QueryFilter::default();
            
                        if let Some((entity, toi)) = rapier_context.cast_ray(
                            player_loc, player_looking, max_toi, solid, filter
                        ) {
            
                            let id = entity.index();
            
                            if all_ents.enemies.contains_key(&format!("enemy:{id}")) {
                                let r#type = EntityType::Enemy;
                                let data = RawEntity{ id: format!("{id}"), pos: Vec3::Y, enemy_type: Enemy::None };
            
                                let msg = ServerMessage::EntityDespawn { r#type, data };
                                packet_channel.sender.send(msg).unwrap();
                                commands.entity(entity).despawn_recursive();
                                all_ents.enemies.remove(&format!("enemy:{id}"));
                            }
            
                        }
                    },
                    lost_lib::defaults::Action::AltFire => todo!(),
                    lost_lib::defaults::Action::Ability1 => todo!(),
                    lost_lib::defaults::Action::Ability2 => todo!(),
                    lost_lib::defaults::Action::Ability3 => {
                        HandleAbility3!(commands, player, packet_channel.clone());
                    },
                    lost_lib::defaults::Action::Crouch => todo!(),
                }

            }

            // match player.class {
                // lost_lib::defaults::Class::Fighter => todo!(),
                // lost_lib::defaults::Class::Engineer => {},
                // lost_lib::defaults::Class::Chronowatcher => todo!(),
                // lost_lib::defaults::Class::Emperor => todo!(),
                // lost_lib::defaults::Class::Mutant => todo!(),
                // lost_lib::defaults::Class::Harbinger => todo!(),
                // lost_lib::defaults::Class::Paladin => todo!(),
                // lost_lib::defaults::Class::Mystic => todo!(),
            // }

        },
        Err(_) => {},
    }
    
}

