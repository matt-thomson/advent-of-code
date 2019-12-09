mod command;
mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use structopt::StructOpt;

use command::Command;

include!(concat!(env!("OUT_DIR"), "/commands.rs"));
