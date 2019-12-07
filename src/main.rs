mod command;
mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use structopt::StructOpt;

use command::Command;

include!(concat!(env!("OUT_DIR"), "/commands.rs"));

fn main() {
    let command = parse_args();
    println!("{}", command.part_one());
    println!("{}", command.part_two());
}
