#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Direction {
    pub fn as_i64(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    pub fn all() -> &'static [Direction] {
        &DIRECTIONS
    }

    pub fn opposite(&self) -> &Direction {
        match self {
            Direction::North => &Direction::South,
            Direction::South => &Direction::North,
            Direction::West => &Direction::East,
            Direction::East => &Direction::West,
        }
    }
}
