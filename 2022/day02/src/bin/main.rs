use std::env;
use std::error::Error;

use anyhow::anyhow;
use day02::Problem;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("must supply path"))?;

    let problem = Problem::new(path)?;

    println!("Part one: {}", problem.part_one()?);

    Ok(())
}
