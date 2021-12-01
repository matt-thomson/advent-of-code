use day01::Day01;

use clap::Parser;

fn main() {
    let problem = Day01::parse();

    println!("Part 1: {}", problem.part_one());
    println!("Part 2: {}", problem.part_two());
}
