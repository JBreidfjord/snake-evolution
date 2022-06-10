use std::collections::{HashSet, VecDeque};

use crate::snake::direction::Direction;
use crate::snake::position::Position;

const MIN_SNAKE_LENGTH: usize = 2;

#[derive(Clone)]
pub(crate) struct Game {
    size: isize,
    snake: VecDeque<Position>,
    food: Position,
    score: usize,
    step_count: usize,
    remaining_moves: usize,
    finished: bool,
}

impl Game {
    pub(crate) fn new(mut size: isize) -> Game {
        if size < 3 {
            size = 3
        }

        // Calculate center square for initial snake placement
        let center = size / 2;
        let center_square = Position::new(center, center);

        let mut snake = VecDeque::with_capacity(size.pow(2).try_into().unwrap());
        snake.push_back(center_square);

        let snake_set: HashSet<Position> = snake.iter().cloned().collect();
        let board_set: HashSet<Position> = HashSet::from_iter(
            (0..size.pow(2))
                .map(|i| Position::new(i, i))
                .collect::<Vec<_>>(),
        );
        let valid_squares = &board_set - &snake_set;
        let food = *valid_squares.iter().next().unwrap();

        Game {
            size,
            snake,
            food,
            score: 0,
            step_count: 0,
            remaining_moves: 200,
            finished: false,
        }
    }

    pub(crate) fn move_snake(&mut self, direction: Direction) {
        if self.finished {
            return;
        }

        let mut head = *self.snake.back().unwrap();

        // Check if next move would cause snake to collide with wall
        if (head.x == self.size && direction == Direction::Right)
            || (head.x == 0 && direction == Direction::Left)
        {
            self.game_over()
        }

        head += direction.value();
        self.snake.push_back(head);

        self.step();
    }

    fn step(&mut self) {
        let head = *self.snake.back().unwrap();

        // Check if snake has moved off grid vertically
        if !(0..self.size).contains(&head.y) {
            self.game_over()
        }

        // Check if snake moved into itself
        if self.snake.iter().filter(|i| *i == &head).count() > 1 {
            self.game_over()
        }

        self.step_count += 1;
        self.remaining_moves -= 1;

        if self.remaining_moves == 0 {
            self.game_over()
        }

        // Check if snake found the food
        if head == self.food {
            self.score += 1;
            self.remaining_moves += 100;
            self.place_food();
        } else if self.snake.len() > MIN_SNAKE_LENGTH {
            self.snake.pop_front();
        }
    }

    fn place_food(&mut self) {
        let snake_set: HashSet<Position> = self.snake.iter().cloned().collect();
        let board_set: HashSet<Position> = HashSet::from_iter(
            (0..self.size.pow(2))
                .map(|i| Position::new(i, i))
                .collect::<Vec<_>>(),
        );
        let valid_squares = &board_set - &snake_set;
        self.food = *valid_squares.iter().next().unwrap();
    }

    fn game_over(&mut self) {
        self.finished = true;
    }

    pub(crate) fn display(&self) -> String {
        let head = *self.snake.back().unwrap();

        let mut out = String::from("|");
        for _ in 0..self.size {
            out += "---"
        }

        for y in 0..self.size {
            out += "|\n|";
            for x in 0..self.size {
                let position = Position::new(x, y);
                out += match position {
                    p if p == head => " \u{25A1} ",
                    p if p == self.food => " \u{2022} ",
                    p if self.snake.contains(&p) => " \u{25A0} ",
                    _ => "   ",
                };
            }
        }

        out += "|\n|";
        for _ in 0..self.size {
            out += "---"
        }
        out += "|\n";

        out += &format!("Score: {}", self.score);

        out
    }

    pub(crate) fn finished(&self) -> bool {
        self.finished
    }

    pub(crate) fn score(&self) -> usize {
        self.score
    }

    pub(crate) fn step_count(&self) -> usize {
        self.step_count
    }

    pub(crate) fn snake(&self) -> &VecDeque<Position> {
        &self.snake
    }

    pub(crate) fn food(&self) -> Position {
        self.food
    }

    pub(crate) fn size(&self) -> isize {
        self.size
    }

    pub(crate) fn set_food(&mut self, food: Position) {
        assert!(!self.snake.contains(&food));
        assert!((0..self.size).contains(&food.x));
        assert!((0..self.size).contains(&food.y));
        self.food = food;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn test_initial_snake_placement() {
            let game = Game::new(10);
            assert_eq!(game.snake, VecDeque::from([Position { x: 5, y: 5 }]));

            let game = Game::new(3);
            assert_eq!(game.snake, VecDeque::from([Position { x: 1, y: 1 }]));
        }

        #[test]
        fn test_initial_food_placement() {
            let size = 3;
            let game = Game::new(size);
            assert_ne!(game.food, Position { x: 1, y: 1 });
            assert!((0..size).contains(&game.food.x));
            assert!((0..size).contains(&game.food.y));

            let size = 10;
            let game = Game::new(size);
            assert_ne!(game.food, Position { x: 5, y: 5 });
            assert!((0..size).contains(&game.food.x));
            assert!((0..size).contains(&game.food.y));
        }
    }

    mod move_snake {
        use super::*;

        #[test]
        fn test_move() {
            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };

            game.move_snake(Direction::Left);
            assert_eq!(
                game.snake,
                VecDeque::from([Position { x: 1, y: 1 }, Position { x: 0, y: 1 }])
            );

            game.move_snake(Direction::Down);
            assert_eq!(
                game.snake,
                VecDeque::from([Position { x: 0, y: 1 }, Position { x: 0, y: 2 }])
            );

            game.move_snake(Direction::Right);
            assert_eq!(
                game.snake,
                VecDeque::from([Position { x: 0, y: 2 }, Position { x: 1, y: 2 }])
            );

            game.move_snake(Direction::Up);
            assert_eq!(
                game.snake,
                VecDeque::from([Position { x: 1, y: 2 }, Position { x: 1, y: 1 }])
            );
        }

        #[test]
        fn test_wall_collisions() {
            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };
            game.move_snake(Direction::Left);
            game.move_snake(Direction::Left);
            assert!(game.finished);

            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };
            game.move_snake(Direction::Up);
            game.move_snake(Direction::Up);
            assert!(game.finished);

            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };
            game.move_snake(Direction::Right);
            game.move_snake(Direction::Right);
            assert!(game.finished);

            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };
            game.move_snake(Direction::Down);
            game.move_snake(Direction::Down);
            assert!(game.finished);
        }

        #[test]
        fn test_snake_collision() {
            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };

            game.move_snake(Direction::Left);
            game.move_snake(Direction::Right);
            assert!(game.finished);
        }

        #[test]
        fn test_food_collision() {
            let mut game = Game::new(3);
            game.food = Position { x: 0, y: 0 };

            game.move_snake(Direction::Left);
            game.move_snake(Direction::Up);
            assert_eq!(game.snake.len(), 3);
            assert_eq!(
                game.snake,
                VecDeque::from([
                    Position { x: 1, y: 1 },
                    Position { x: 0, y: 1 },
                    Position { x: 0, y: 0 }
                ])
            );
            assert_eq!(game.score, 1);
        }
    }

    mod food {
        use super::*;

        #[test]
        fn test_place_food() {
            let size = 3;
            let mut game = Game::new(size);
            game.food = Position { x: 0, y: 1 };
            game.move_snake(Direction::Left);

            assert!((0..3).contains(&game.food.x));
            assert!((0..3).contains(&game.food.y));
            assert!(!game.snake.contains(&game.food));
        }
    }
}
