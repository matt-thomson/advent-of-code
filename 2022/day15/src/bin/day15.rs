use std::env;
use std::error::Error;

use day15::Problem;
use eyre::eyre;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("must supply path"))?;

    let problem = Problem::new(path)?;

    println!("Part one: {}", problem.part_one(2_000_000));
    println!("Part one: {}", problem.part_two(4_000_000)?);

    Ok(())
}
