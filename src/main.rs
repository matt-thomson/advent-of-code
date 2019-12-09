mod intcode;
mod problem;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use structopt::StructOpt;

use problem::Problem;

include!(concat!(env!("OUT_DIR"), "/problems.rs"));
