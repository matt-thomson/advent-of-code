use std::str::FromStr;

use eyre::{eyre, ErrReport};

#[derive(Debug)]
pub struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = input.lines().rev().collect();

        let num_stacks = lines[0]
            .split_whitespace()
            .last()
            .ok_or_else(|| eyre!("couldn't find number of stacks"))?
            .parse()?;

        let mut stacks = vec![vec![]; num_stacks];

        for line in &lines[1..] {
            for (index, stack) in stacks.iter_mut().enumerate() {
                if let Some(c) = line
                    .chars()
                    .nth(4 * index + 1)
                    .filter(|c| !c.is_whitespace())
                {
                    stack.push(c);
                }
            }
        }

        Ok(Self(stacks))
    }
}
