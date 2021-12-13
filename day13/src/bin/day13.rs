use std::env;

use day13::Problem;

fn main() {
    let path = env::args().nth(1).unwrap();
    let problem = Problem::new(path);

    println!("Part 1: {}", problem.part_one());
    println!();
    println!("{}", problem.part_two());
}
