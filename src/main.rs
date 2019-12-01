mod day01;

use std::env;
use std::path::Path;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let path = Path::new(&filename);

    println!("{}", day01::part_one(&path));
    println!("{}", day01::part_two(&path));
}
