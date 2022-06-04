#[derive(PartialEq)]
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

    pub(crate) fn value(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(Direction::Up.value(), (0, -1));
        assert_eq!(Direction::Down.value(), (0, 1));
        assert_eq!(Direction::Left.value(), (-1, 0));
        assert_eq!(Direction::Right.value(), (1, 0));
        assert_eq!(Direction::UpLeft.value(), (-1, -1));
        assert_eq!(Direction::UpRight.value(), (1, -1));
        assert_eq!(Direction::DownLeft.value(), (-1, 1));
        assert_eq!(Direction::DownRight.value(), (1, 1));
    }
}
