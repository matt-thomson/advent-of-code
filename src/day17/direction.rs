use super::image::Position;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    pub fn all() -> &'static [Direction] {
        &DIRECTIONS
    }
}
