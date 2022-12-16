use std::env;
use std::error::Error;

use day10::Problem;
use eyre::eyre;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("must supply path"))?;

    let problem = Problem::new(path)?;

    println!("Part one: {}", problem.part_one());

    println!("Part two:");
    println!("{}", problem.part_two());

    Ok(())
}
