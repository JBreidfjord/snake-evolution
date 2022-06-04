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

}
}
