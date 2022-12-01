#[derive(Debug, PartialEq)]
pub enum Rotation {
    AntiClockwise,
    Clockwise,
}

impl From<i64> for Rotation {
    fn from(input: i64) -> Rotation {
        match input {
            0 => Rotation::AntiClockwise,
            1 => Rotation::Clockwise,
            x => panic!("unknown rotation {}", x),
        }
    }
}
