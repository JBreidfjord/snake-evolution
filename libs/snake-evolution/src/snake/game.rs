use std::collections::{HashSet, VecDeque};

pub(crate) struct Game {
    size: isize,
    snake: VecDeque<isize>,
    food: isize,
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

        Game { size, snake, food }
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
            game.move_snake(Direction::Left);
            game.food = 0;
            assert_eq!(game.snake, VecDeque::from([4, 3]));

            game.move_snake(Direction::Down);
            assert_eq!(game.snake, VecDeque::from([3, 6]));

            game.move_snake(Direction::Right);
            assert_eq!(game.snake, VecDeque::from([6, 7]));

            game.move_snake(Direction::Up);
            assert_eq!(game.snake, VecDeque::from([7, 4]));
        }
    }
}
