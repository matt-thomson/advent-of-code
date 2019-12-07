mod command;
mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use command::Command;

fn main() {
    let command = Command::from_args();
    println!("{}", command.part_one());
    println!("{}", command.part_two());
}
