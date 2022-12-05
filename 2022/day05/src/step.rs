use std::str::FromStr;

use eyre::ErrReport;

#[derive(Debug)]
pub struct Step {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Step {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = input.split_whitespace().collect();

        let count = parts[1].parse()?;
        let from = parts[3].parse()?;
        let to = parts[5].parse()?;

        Ok(Self { count, from, to })
    }
}
