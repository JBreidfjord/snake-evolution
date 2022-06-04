use ordered_float::OrderedFloat;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::evolution::brain::Brain;
use crate::snake::direction::Direction;
use crate::snake::game::Game;

pub(crate) struct Snake {
    pub(crate) game: Game,
    pub(crate) brain: Brain,
    pub(crate) history: Vec<String>,
}

impl Snake {
    pub(crate) fn random(rng: &mut dyn RngCore, grid_size: isize) -> Snake {
        let game = Game::new(grid_size);
        let brain = Brain::random(rng);

        Snake {
            game,
            brain,
            history: Vec::new(),
        }
    }

    pub(crate) fn make_move(&mut self) {
        let vision = self.process_vision();
        let action = self.brain.nn.propagate(vision);
        let action_index = (0..action.len())
            .max_by_key(|i| OrderedFloat::from(action[*i]))
            .unwrap();
        let direction = Direction::from_index(action_index);

        self.game.move_snake(direction);
        self.history.push(self.game.display());
    }

    fn process_vision(&self) -> Vec<f32> {
        let head = *self.game.snake().back().unwrap();
        let mut vision = vec![0.0; 24];
        for (i, direction) in Direction::iter().enumerate() {
            let mut cursor = head;
            let (x, y) = direction.value();

            let mut distance = 0.0;
            'search_loop: loop {
                cursor += x + y * self.game.size();
                distance += 1.0;

                if cursor == self.game.food() {
                    vision[i * 3] = distance;
                } else if self.game.snake().contains(&cursor) {
                    vision[i * 3 + 1] = distance;
                } else if (0..self.game.size().pow(2)).contains(&cursor) {
                    vision[i * 3 + 2] = distance;
                    break 'search_loop;
                }
            }
        }

        vision
    }

    pub(crate) fn fitness(&self) -> f32 {
        self.game.score() as f32 - self.game.step_count() as f32 * 0.01
    }
}
