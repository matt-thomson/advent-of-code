mod command;
mod day01;
mod day02;
mod day03;

use command::Command;

fn main() {
    let command = Command::from_args();
    println!("{}", command.part_one());
    println!("{}", command.part_two());
}
