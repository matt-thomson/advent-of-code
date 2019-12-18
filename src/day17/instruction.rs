#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}
