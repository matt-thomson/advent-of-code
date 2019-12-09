use std::fmt::Display;

pub trait Command {
    type Output: Display;

    fn part_one(&self) -> Self::Output;
    fn part_two(&self) -> Self::Output;

    fn run(&self) {
        println!("{}", self.part_one());
        println!("{}", self.part_two());
    }
}
