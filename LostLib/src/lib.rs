use bevy::prelude::Vec3;

include!(concat!(env!("OUT_DIR"), "/defaults.rs"));

pub const WORLD_PLANE_VEC: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
pub const WORLD_PLANE_SIZE: f32 = 400.0;
pub const WORLD_ENTITY_SPAWN_HEIGHT: f32 = 2.0;

pub mod messages;
pub mod defaults;
pub mod formatters;
pub mod classes;
pub mod helpers;