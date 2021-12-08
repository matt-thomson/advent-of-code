use std::env;

use day08::Problem;

fn main() {
    let path = env::args().nth(1).unwrap();
    let problem = Problem::new(path);

    println!("Part 1: {}", problem.part_one());
    println!("Part 2: {}", problem.part_two());
}
