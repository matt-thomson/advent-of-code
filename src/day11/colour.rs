#[derive(Debug, PartialEq)]
pub enum Colour {
    Black,
    White,
}

impl Colour {
    pub fn to_i64(&self) -> i64 {
        match *self {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

impl From<i64> for Colour {
    fn from(input: i64) -> Colour {
        match input {
            0 => Colour::Black,
            1 => Colour::White,
            x => panic!("unknown colour {}", x),
        }
    }
}
