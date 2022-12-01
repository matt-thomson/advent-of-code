use super::rotation::Rotation;
use super::state::Point;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn(&self, rotation: Rotation) -> Direction {
        match (self, rotation) {
            (Direction::Up, Rotation::AntiClockwise) => Direction::Left,
            (Direction::Up, Rotation::Clockwise) => Direction::Right,

            (Direction::Right, Rotation::AntiClockwise) => Direction::Up,
            (Direction::Right, Rotation::Clockwise) => Direction::Down,

            (Direction::Down, Rotation::AntiClockwise) => Direction::Right,
            (Direction::Down, Rotation::Clockwise) => Direction::Left,

            (Direction::Left, Rotation::AntiClockwise) => Direction::Down,
            (Direction::Left, Rotation::Clockwise) => Direction::Up,
        }
    }

    pub fn step(&self, point: &Point) -> Point {
        let (x, y) = *point;

        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}
