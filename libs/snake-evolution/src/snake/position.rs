use rand::prelude::*;
use std::ops::Add;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub(crate) struct Position {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl Position {
    pub(crate) fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    pub(crate) fn random_within_size(rng: &mut dyn RngCore, size: isize) -> Position {
        Position {
            x: rng.gen_range(0..size),
            y: rng.gen_range(0..size),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
