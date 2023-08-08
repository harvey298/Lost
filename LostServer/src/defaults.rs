use crossbeam_channel::unbounded;

use crate::structs::{PlayerActionChannel};

impl Default for PlayerActionChannel {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { sender: tx, receiver: rx }
    }
}

