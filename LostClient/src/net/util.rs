use bevy::{prelude::*, utils::HashMap, ecs::system::EntityCommands};
use bevy_text_mesh::{TextMeshBundle, TextMesh, TextMeshFont};
use crossbeam_channel::{Sender, Receiver, unbounded};
use lost_lib::defaults::{LivingEntities, AliveEnemy, Enemy};



#[derive(Debug, Resource)]
pub struct Me {
    pub username: String,
    pub id: String,
    pub server_id: u64
}

/// Handles Entity spawning/despawning
/// Also entity movement
pub fn entity_handle (
    mut all_ents: ResMut<LivingEntities>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    channel: Res<EnemySpawnChannels>,
    mut entity_list: Local<HashMap<String, AliveEnemy>>,
    mut entities: Query<(&SpawnedEntity, &mut Transform, Entity)>,
    asset_server: Res<AssetServer>,
    key: Res<Input<KeyCode>>,
) {

    // entities.into_iter().len()

    if (entity_list.is_empty() && channel.receiver.is_empty() ) || key.just_released(KeyCode::O) {
        info!("Remove forgotten entities!");
        entity_list.clear();

        for (spawned_entity, mut ent_transform, true_ent) in entities.iter_mut() {
                        
            commands.get_entity(true_ent).unwrap().despawn_recursive();
        }

        return
    }

    if channel.receiver.is_empty() { return; }

    if let Ok(e) = channel.receiver.try_recv() {

        let id = e.0;
        let mut ent = e.1;

        let id = if id.contains("enemy") { id.split(":").last() } else { Some(id.as_str()) };

        let id = id.unwrap().to_string();

        let pos = ent.pos;

        // info!("{:?}",entity_list);

        // info!("{} | {}", entities.into_iter().len(), entity_list.len());

        if !entity_list.contains_key(&id) {

            // info!("New entity spawning with id: {id}");

            let text = format!("{} | {}",ent.r#type, id.clone());

            let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

            let name_tag = commands.spawn(TextMeshBundle {
                // \nClass: Black Person\nHealth: 100
                text_mesh: TextMesh::new_with_color(&text, font.clone(), Color::rgb(1., 1., 0.)),
                transform: Transform::from_xyz(-1., 1.75, 0.),
                ..Default::default()
            });

            let name_tag = name_tag.id();

            let mut entity = commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..default()
            });

            entity.add_child(name_tag);

            entity.insert(SpawnedEntity{ id: ent.clone().id.parse().unwrap() });

            entity_list.insert(ent.clone().id, ent);

        } else {

            let id: u64 = id.parse().unwrap();

            for (spawned_entity, mut ent_transform, true_ent) in entities.iter_mut() {

                if id != spawned_entity.id { continue; }

                if ent.r#type == Enemy::None {
                
                    if let Some(_) = entity_list.remove(&format!("{}",id)) {
                        // Entities are removed!
                        // info!("Despawning! | {id}");
                        commands.get_entity(true_ent).unwrap().despawn_recursive();
                        
                    }
                    
                } else {

                    ent_transform.translation = pos.clone();

                    entity_list.insert(ent.clone().id, ent.clone());

                }
            }

        }

    }

}

#[derive(Debug, Resource)]
pub struct EnemySpawnChannels {
    pub sender: Sender<(String, AliveEnemy)>,
    pub receiver: Receiver<(String, AliveEnemy)>,
}


impl Default for EnemySpawnChannels {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { sender: tx, receiver: rx }
    }
}

#[derive(Component, Reflect)]
pub struct SpawnedEntity {
    pub id: u64,
}