#[derive(Debug)]
pub enum Pixel {
    OpenSpace,
    Scaffold,
    Robot,
}

impl Pixel {
    pub fn from(input: char) -> Self {
        match input {
            '#' => Self::Scaffold,
            '.' => Self::OpenSpace,
            '^' => Self::Robot,
            _ => panic!("unknown pixel {}", input),
        }
    }
}
