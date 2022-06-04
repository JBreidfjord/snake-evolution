use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::evolution::brain::Brain;
use crate::snake::direction::Direction;
use crate::snake::game::Game;

pub(crate) struct Snake {
    pub(crate) game: Game,
    pub(crate) brain: Brain,
}

impl Snake {
    pub(crate) fn random(rng: &mut dyn RngCore, grid_size: isize) -> Snake {
        let mut game = Game::new(grid_size);
        let brain = Brain::random(rng);

        Snake { game, brain }
    }

    pub(crate) fn make_move(&mut self) {
        let vision = self.process_vision();
        let action = self.brain.nn.propagate(vision);
        let action_index = (0..action.len())
            .max_by_key(|i| OrderedFloat::from(action[*i]))
            .unwrap();
        let direction = Direction::from_index(action_index);

        self.game.move_snake(direction);
    }

    fn process_vision(&self) -> Vec<f32> {
        todo!("Implement sight")
    }

    pub(crate) fn fitness(&self) -> f32 {
        self.game.score() as f32 - self.game.step_count() as f32 * 0.01
    }
}
