use std::fmt;

use bevy::prelude::Vec3;

use crate::defaults::{Class, LivingEntities, Enemy, AliveEnemy};

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Class::Fighter => { write!(f, "Fighter") },
            Class::Engineer => { write!(f, "Engineer") },
            Class::Chronowatcher => { write!(f, "Chronowatcher") },
            Class::Emperor => { write!(f, "Emperor") },
            Class::Mutant => { write!(f, "Mutant") },
            Class::Harbinger => write!(f, "Harbinger"),
            Class::Paladin => write!(f, "Paladin"),
            Class::Mystic => write!(f, "Mystic"),
        }
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Enemy::SkeletonArcher => { write!(f, "Skeleton Archer") },
            Enemy::Vawnite => { write!(f, "Vamnite DEBUG: lvl1") },
            Enemy::Debug => { write!(f, "Debug") },
            Enemy::None => { write!(f, "None") },

        }
    }
}

impl Enemy {
    pub fn make_alive(&self, id: String, loc: Vec3) -> AliveEnemy {

        AliveEnemy { id, r#type: *self, pos: loc, direction: Vec3 { x: 0.0, y: 0.0, z: 0.0 }  }

    }
}