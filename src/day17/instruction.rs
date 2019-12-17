#[derive(Debug)]
pub enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}
