use std::collections::{HashSet, VecDeque};

use crate::snake::direction::Direction;

const MIN_SNAKE_LENGTH: usize = 2;

#[derive(Clone)]
pub(crate) struct Game {
    size: isize,
    snake: VecDeque<isize>,
    food: isize,
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
        let center = if size % 2 == 0 {
            size / 2
        } else {
            size / 2 + 1
        };
        let center_square = size * (center - 1) + center - 1;

        let mut snake = VecDeque::with_capacity(size.pow(2).try_into().unwrap());
        snake.push_back(center_square);

        let snake_set: HashSet<isize> = snake.iter().cloned().collect();
        let board_set: HashSet<isize> = HashSet::from_iter((0..size.pow(2)).collect::<Vec<_>>());
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

        let (x, y) = direction.value();
        let mut head = *self.snake.back().unwrap();

        // Check if next move would cause snake to collide with wall
        if (head % self.size == self.size - 1 && direction == Direction::Right)
            || (head % self.size == 0 && direction == Direction::Left)
        {
            self.game_over()
        }

        // Add x and y values to current head index
        // y is multiplied by grid size to shift index by a whole row
        head += x + y * self.size;
        self.snake.push_back(head);

        self.step();
    }

    fn step(&mut self) {
        self.step_count += 1;
        self.remaining_moves -= 1;

        if self.remaining_moves == 0 {
            self.game_over()
        }

        let head = *self.snake.back().unwrap();

        // Check if snake has moved off grid vertically
        if !(0..self.size.pow(2)).contains(&head) {
            self.game_over()
        }

        // Check if snake moved into itself
        if self.snake.iter().filter(|i| *i == &head).count() > 1 {
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
        let snake_set: HashSet<isize> = self.snake.iter().cloned().collect();
        let board_set: HashSet<isize> =
            HashSet::from_iter((0..self.size.pow(2)).collect::<Vec<_>>());
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

        let range = (0..self.size.pow(2)).step_by(self.size.try_into().unwrap());
        for y in range {
            out += "|\n|";
            for x in 0..self.size {
                let i = y + x;
                out += match i {
                    n if n == head => " \u{25A1} ",
                    n if n == self.food => " \u{2022} ",
                    n if self.snake.contains(&n) => " \u{25A0} ",
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

    pub(crate) fn snake(&self) -> &VecDeque<isize> {
        &self.snake
    }

    pub(crate) fn food(&self) -> isize {
        self.food
    }

    pub(crate) fn size(&self) -> isize {
        self.size
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
            assert_eq!(game.snake, VecDeque::from([44]));

            let game = Game::new(3);
            assert_eq!(game.snake, VecDeque::from([4]));
        }

        #[test]
        fn test_initial_food_placement() {
            let game = Game::new(3);
            assert_ne!(game.food, 4);
            assert!((0..9).contains(&game.food));

            let game = Game::new(10);
            assert_ne!(game.food, 44);
            assert!((0..100).contains(&game.food));
        }
    }

    mod move_snake {
        use super::*;

        #[test]
        fn test_move() {
            let mut game = Game::new(3);
            game.food = 0;

            game.move_snake(Direction::Left);
            assert_eq!(game.snake, VecDeque::from([4, 3]));

            game.move_snake(Direction::Down);
            assert_eq!(game.snake, VecDeque::from([3, 6]));

            game.move_snake(Direction::Right);
            assert_eq!(game.snake, VecDeque::from([6, 7]));

            game.move_snake(Direction::Up);
            assert_eq!(game.snake, VecDeque::from([7, 4]));
        }

        #[test]
        fn test_wall_collisions() {
            let mut game = Game::new(3);
            game.food = 0;
            game.move_snake(Direction::Left);
            game.move_snake(Direction::Left);
            assert!(game.finished);

            let mut game = Game::new(3);
            game.food = 0;
            game.move_snake(Direction::Up);
            game.move_snake(Direction::Up);
            assert!(game.finished);

            let mut game = Game::new(3);
            game.food = 0;
            game.move_snake(Direction::Right);
            game.move_snake(Direction::Right);
            assert!(game.finished);

            let mut game = Game::new(3);
            game.food = 0;
            game.move_snake(Direction::Down);
            game.move_snake(Direction::Down);
            assert!(game.finished);
        }

        #[test]
        fn test_snake_collision() {
            let mut game = Game::new(3);
            game.food = 0;

            game.move_snake(Direction::Left);
            game.move_snake(Direction::Right);
            assert!(game.finished);
        }

        #[test]
        fn test_food_collision() {
            let mut game = Game::new(3);
            game.food = 0;

            game.move_snake(Direction::Left);
            game.move_snake(Direction::Up);
            assert_eq!(game.snake.len(), 3);
            assert_eq!(game.snake, VecDeque::from([4, 3, 0]));
            assert_eq!(game.score, 1);
        }
    }

    mod food {
        use super::*;

        #[test]
        fn test_place_food() {
            let mut game = Game::new(3);
            game.food = 3;
            game.move_snake(Direction::Left);

            assert!((0..9).contains(&game.food));
            assert_ne!(game.food, 4);
            assert_ne!(game.food, 3);
        }
    }
}
