//! Does entity spawning, etc
//! Only for enemies!

use std::fmt::format;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{RigidBody, Collider};
use lost_lib::{defaults::{AliveEnemy, EntityType, RawEntity, Enemy, LivingEntities}, messages::ServerMessage, WORLD_ENTITY_SPAWN_HEIGHT};

use crate::{director::SpawnedEntity};

use lost_lib::defaults::PacketChannel;

use rand::thread_rng;
use rand::Rng;
pub struct WorldDirector;

// lazy_static! {
//     static ref HASHMAP: HashMap<u32, &'static str> = {
//         let mut m = HashMap::new();
//         m.insert(0, "foo");
//         m.insert(1, "bar");
//         m.insert(2, "baz");
//         m
//     };
// }

#[derive(Resource)]
pub struct EntityList {
    pub entites: HashMap<String, AliveEnemy>,

    /// ID : Entity Details
    pub simple_entites: HashMap<String, RawEntity>,
}

impl Plugin for WorldDirector {
    fn build(&self, app: &mut App) {

        app.insert_resource(EntityList::default());

        app.add_system(entity_spawner);
        app.add_system(entity_location_tracker);
        
    }
}

fn entity_location_tracker (
    entites: Query<(&SpawnedEntity, &Transform)>,
    mut ent_cache: ResMut<LivingEntities>,
    packet_channel: Res<PacketChannel>,
) {

    for (id, transform) in entites.into_iter() {

        let key = format!("enemy:{}",id.id);
        let simple_key = format!("{}",id.id);

        if let Some(ent) = ent_cache.enemies.get(&key) {

            let old_pos = ent.pos.floor();
            let new_pos = transform.translation.floor();

            if old_pos.to_string() == new_pos.to_string() { continue; }

            let data = RawEntity { id: simple_key.clone(), pos: transform.translation, enemy_type: ent.r#type };
            let msg = ServerMessage::EntityChange { data };

            packet_channel.sender.send(msg).unwrap();

            let new_ent = AliveEnemy{ id: simple_key, r#type: ent.r#type, pos: transform.translation, direction: ent.direction };

            ent_cache.enemies.insert(key, new_ent);
            
        }
    }
}

fn entity_spawner (
    mut commands: Commands,
    mut rank: Local<usize>,
    mut credits: Local<isize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_list: ResMut<EntityList>,
    packet_channel: Res<PacketChannel>,
    keys: Res<Input<KeyCode>>,
    // For net
    mut all_ents: ResMut<LivingEntities>,
) {
    if credits.clone() == 0 || keys.pressed(KeyCode::I) {
        *credits = 100;
    }

    if credits.clone() == 100 {

        let mut rng = thread_rng();
        let x: f32 = rng.gen_range(-10.0..10.0);
        let z: f32 = rng.gen_range(-10.0..10.0);

        let pos = Transform::from_xyz(x, WORLD_ENTITY_SPAWN_HEIGHT, z);

        let mut entity = commands.spawn(RigidBody::Dynamic);
        
        entity.insert(Collider::cuboid(1.0, 1.0, 1.0));

        entity.insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: pos,
            ..default()
        });

        let ent_id = entity.id();

        let index = ent_id.index();

        entity.insert(SpawnedEntity{ id: index as u64 });

        let ent_id = index.to_string();

        info!("Spawning enemy");

        let raw_entity = RawEntity{ id: ent_id.clone(), pos: pos.translation, enemy_type: Enemy::SkeletonArcher };

        let key = format!("enemy:{}",ent_id);

        all_ents.enemies.insert(key.clone(), raw_entity.clone().make_alive());

        entity_list.simple_entites.insert(key, raw_entity.clone());

        let msg = ServerMessage::EntitySpawn { r#type: EntityType::Enemy, data: raw_entity };

        packet_channel.sender.send(msg).unwrap();

        *credits -= 50;
    }  


}



impl Default for EntityList {
    fn default() -> Self {
        Self { entites: HashMap::new(), simple_entites: HashMap::new() }
    }
}

