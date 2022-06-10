use ordered_float::OrderedFloat;
use rand::prelude::*;
use strum::IntoEnumIterator;

use lib_genetic_algorithm::Chromosome;

use crate::evolution::brain::Brain;
use crate::snake::direction::Direction;
use crate::snake::game::Game;
use crate::snake::position::Position;
use crate::GAME_GRID_SIZE;

#[derive(Clone)]
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
        if self.game.finished() {
            return;
        }

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
            let mut distance = 0.0;

            'search_loop: loop {
                distance += 1.0;
                cursor += direction.value();

                if self.is_food_collision(&cursor) {
                    vision[i * 3] = distance;
                } else if self.is_tail_collision(&cursor) {
                    // We want nearest tail, so only set if it hasn't been set yet
                    if vision[i * 3 + 1] == 0.0 {
                        vision[i * 3 + 1] = distance;
                    }
                } else if self.is_wall_collision(&cursor) {
                    vision[i * 3 + 2] = distance;
                    break 'search_loop;
                }
            }
        }

        vision
    }

    fn is_food_collision(&self, position: &Position) -> bool {
        todo!("Implement food collision check")
    }

    fn is_wall_collision(&self, position: &Position) -> bool {
        todo!("Implement wall collision check")
    }

    fn is_tail_collision(&self, position: &Position) -> bool {
        todo!("Implement tail collision check")
    }

    pub(crate) fn fitness(&self) -> f32 {
        self.game.score() as f32 * 10.0 + self.game.step_count() as f32 * 0.01
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome) -> Snake {
        let game = Game::new(GAME_GRID_SIZE);
        let brain = Brain::from_chromosome(chromosome);
        Snake {
            game,
            brain,
            history: Vec::new(),
        }
    }
}
