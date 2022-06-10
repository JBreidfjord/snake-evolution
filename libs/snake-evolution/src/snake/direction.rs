use strum_macros::EnumIter;

use crate::snake::position::Position;

#[derive(PartialEq, EnumIter)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub(crate) fn from_index(index: usize) -> Direction {
        match index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Index must be <= 3"),
        }
    }

    pub(crate) fn value(&self) -> Position {
        match *self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
            Direction::UpLeft => Position { x: -1, y: -1 },
            Direction::UpRight => Position { x: 1, y: -1 },
            Direction::DownLeft => Position { x: -1, y: 1 },
            Direction::DownRight => Position { x: 1, y: 1 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(Direction::Up.value(), Position { x: 0, y: -1 });
        assert_eq!(Direction::Down.value(), Position { x: 0, y: 1 });
        assert_eq!(Direction::Left.value(), Position { x: -1, y: 0 });
        assert_eq!(Direction::Right.value(), Position { x: 1, y: 0 });
        assert_eq!(Direction::UpLeft.value(), Position { x: -1, y: -1 });
        assert_eq!(Direction::UpRight.value(), Position { x: 1, y: -1 });
        assert_eq!(Direction::DownLeft.value(), Position { x: -1, y: 1 });
        assert_eq!(Direction::DownRight.value(), Position { x: 1, y: 1 });
    }
}
