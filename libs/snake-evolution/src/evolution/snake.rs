use rand::prelude::*;

use crate::evolution::brain::Brain;
use crate::snake::game::Game;

pub(crate) struct Snake {
    pub(crate) game: Game,
    pub(crate) brain: Brain,
}

impl Snake {
    pub(crate) fn random(rng: &mut dyn RngCore, grid_size: usize) -> Snake {
        todo!("Create random snake")
    }
}
