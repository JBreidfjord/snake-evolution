use rand::prelude::*;

use crate::snake::game::Game;

pub(crate) struct Snake {}

impl Snake {
    pub(crate) fn random(rng: &mut dyn RngCore, grid_size: usize) -> Snake {
        todo!("Create random snake")
    }
}
