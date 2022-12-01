use super::direction::Direction;

#[derive(Debug)]
pub enum Pixel {
    OpenSpace,
    Scaffold,
    Robot(Direction),
}

impl Pixel {
    pub fn from(input: char) -> Self {
        match input {
            '#' => Self::Scaffold,
            '.' => Self::OpenSpace,
            '^' => Self::Robot(Direction::Up),
            'v' => Self::Robot(Direction::Down),
            '<' => Self::Robot(Direction::Left),
            '>' => Self::Robot(Direction::Right),
            _ => panic!("unknown pixel {}", input),
        }
    }
}
