use super::image::Position;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Rotation {
    AntiClockwise,
    Clockwise,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    pub fn step(&self, position: &Position) -> Option<Position> {
        let (x, y) = *position;

        match self {
            Self::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Self::Down => Some((x, y + 1)),
            Self::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Self::Right => Some((x + 1, y)),
        }
    }

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

    pub fn all() -> &'static [Direction] {
        &DIRECTIONS
    }
}
