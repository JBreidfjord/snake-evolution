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
                }

                if self.is_tail_collision(&cursor) {
                    // We want nearest tail, so only set if it hasn't been set yet
                    if vision[i * 3 + 1] == 0.0 {
                        vision[i * 3 + 1] = distance;
                    }
                }

                if self.is_wall_collision(&cursor) {
                    vision[i * 3 + 2] = distance;
                    break 'search_loop;
                }
            }
        }

        vision
    }

    fn is_food_collision(&self, position: &Position) -> bool {
        &self.game.food() == position
    }

    fn is_wall_collision(&self, position: &Position) -> bool {
        let boundary = 0..self.game.size();
        !boundary.contains(&position.x) || !boundary.contains(&position.y)
    }

    fn is_tail_collision(&self, position: &Position) -> bool {
        self.game.snake().contains(position)
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

#[cfg(test)]
mod tests {
    use super::*;

    mod vision {
        use super::*;

        #[test]
        fn test_vision() {
            let mut rng = rand::thread_rng();
            let snake = Snake::random(&mut rng, 3);
            let vision = snake.process_vision();
            assert_eq!(vision.len(), 24);
        }

        #[test]
        fn test_food_vision() {
            let mut rng = rand::thread_rng();
            let mut snake = Snake::random(&mut rng, 3);
            snake.game.set_food(Position { x: 0, y: 0 });
            let vision = snake.process_vision();
            assert_eq!(vision[0], 0.0, "Food Vision Up");
            assert_eq!(vision[3], 0.0, "Food Vision Down");
            assert_eq!(vision[6], 0.0, "Food Vision Left");
            assert_eq!(vision[9], 0.0, "Food Vision Right");
            assert_eq!(vision[12], 1.0, "Food Vision UpLeft");
            assert_eq!(vision[15], 0.0, "Food Vision UpRight");
            assert_eq!(vision[18], 0.0, "Food Vision DownLeft");
            assert_eq!(vision[21], 0.0, "Food Vision DownRight");

            snake.game.move_snake(Direction::Left);
            let vision = snake.process_vision();
            assert_eq!(vision[0], 1.0, "Food Vision Up");
            assert_eq!(vision[3], 0.0, "Food Vision Down");
            assert_eq!(vision[6], 0.0, "Food Vision Left");
            assert_eq!(vision[9], 0.0, "Food Vision Right");
            assert_eq!(vision[12], 0.0, "Food Vision UpLeft");
            assert_eq!(vision[15], 0.0, "Food Vision UpRight");
            assert_eq!(vision[18], 0.0, "Food Vision DownLeft");
            assert_eq!(vision[21], 0.0, "Food Vision DownRight");
        }

        #[test]
        fn test_tail_vision() {
            let mut rng = rand::thread_rng();
            let mut snake = Snake::random(&mut rng, 3);
            snake.game.set_food(Position { x: 0, y: 0 });
            snake.game.move_snake(Direction::Left);
            let vision = snake.process_vision();
            assert_eq!(vision[1], 0.0, "Tail Vision Up");
            assert_eq!(vision[4], 0.0, "Tail Vision Down");
            assert_eq!(vision[7], 0.0, "Tail Vision Left");
            assert_eq!(vision[10], 1.0, "Tail Vision Right");
            assert_eq!(vision[13], 0.0, "Tail Vision UpLeft");
            assert_eq!(vision[16], 0.0, "Tail Vision UpRight");
            assert_eq!(vision[19], 0.0, "Tail Vision DownLeft");
            assert_eq!(vision[22], 0.0, "Tail Vision DownRight");

            snake.game.move_snake(Direction::Up);
            let vision = snake.process_vision();
            assert_eq!(vision[1], 0.0, "Tail Vision Up");
            assert_eq!(vision[4], 1.0, "Tail Vision Down");
            assert_eq!(vision[7], 0.0, "Tail Vision Left");
            assert_eq!(vision[10], 0.0, "Tail Vision Right");
            assert_eq!(vision[13], 0.0, "Tail Vision UpLeft");
            assert_eq!(vision[16], 0.0, "Tail Vision UpRight");
            assert_eq!(vision[19], 0.0, "Tail Vision DownLeft");
            assert_eq!(vision[22], 1.0, "Tail Vision DownRight");
        }

        #[test]
        fn test_wall_vision() {
            let mut rng = rand::thread_rng();
            let mut snake = Snake::random(&mut rng, 3);
            let vision = snake.process_vision();
            assert_eq!(vision[2], 2.0, "Wall Vision Up");
            assert_eq!(vision[5], 2.0, "Wall Vision Down");
            assert_eq!(vision[8], 2.0, "Wall Vision Left");
            assert_eq!(vision[11], 2.0, "Wall Vision Right");
            assert_eq!(vision[14], 2.0, "Wall Vision UpLeft");
            assert_eq!(vision[17], 2.0, "Wall Vision UpRight");
            assert_eq!(vision[20], 2.0, "Wall Vision DownLeft");
            assert_eq!(vision[23], 2.0, "Wall Vision DownRight");

            snake.game.move_snake(Direction::Left);
            let vision = snake.process_vision();
            assert_eq!(vision[2], 2.0, "Wall Vision Up");
            assert_eq!(vision[5], 2.0, "Wall Vision Down");
            assert_eq!(vision[8], 1.0, "Wall Vision Left");
            assert_eq!(vision[11], 3.0, "Wall Vision Right");
            assert_eq!(vision[14], 1.0, "Wall Vision UpLeft");
            assert_eq!(vision[17], 2.0, "Wall Vision UpRight");
            assert_eq!(vision[20], 1.0, "Wall Vision DownLeft");
            assert_eq!(vision[23], 2.0, "Wall Vision DownRight");

            snake.game.move_snake(Direction::Down);
            let vision = snake.process_vision();
            assert_eq!(vision[2], 3.0, "Wall Vision Up");
            assert_eq!(vision[5], 1.0, "Wall Vision Down");
            assert_eq!(vision[8], 1.0, "Wall Vision Left");
            assert_eq!(vision[11], 3.0, "Wall Vision Right");
            assert_eq!(vision[14], 1.0, "Wall Vision UpLeft");
            assert_eq!(vision[17], 3.0, "Wall Vision UpRight");
            assert_eq!(vision[20], 1.0, "Wall Vision DownLeft");
            assert_eq!(vision[23], 1.0, "Wall Vision DownRight");
        }
    }
}
