use std::env;
use std::error::Error;

use day02::Problem;
use eyre::eyre;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("must supply path"))?;

    let problem = Problem::new(path)?;

    println!("Part one: {}", problem.part_one()?);
    println!("Part one: {}", problem.part_two()?);

    Ok(())
}
