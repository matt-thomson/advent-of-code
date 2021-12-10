use std::env;

use day10::Problem;

fn main() {
    let path = env::args().nth(1).unwrap();
    let problem = Problem::new(path);

    println!("Part 1: {}", problem.part_one());
}
