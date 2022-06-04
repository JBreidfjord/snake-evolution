use std::collections::{HashSet, VecDeque};

pub(crate) struct Game {
    size: isize,
    snake: VecDeque<isize>,
    food: isize,
}
