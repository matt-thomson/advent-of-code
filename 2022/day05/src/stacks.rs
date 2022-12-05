use std::str::FromStr;

use eyre::{eyre, ErrReport, Result};

use crate::step::Step;

#[derive(Clone, Debug)]
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

impl Stacks {
    pub fn apply_9000(&mut self, step: &Step) -> Result<()> {
        for _ in 0..step.count {
            let item = self.0[step.from - 1]
                .pop()
                .ok_or_else(|| eyre!("reached empty stack"))?;

            self.0[step.to - 1].push(item);
        }

        Ok(())
    }

    pub fn apply_9001(&mut self, step: &Step) -> Result<()> {
        let mut temp = vec![];

        for _ in 0..step.count {
            let item = self.0[step.from - 1]
                .pop()
                .ok_or_else(|| eyre!("reached empty stack"))?;

            temp.push(item);
        }

        temp.reverse();

        for item in temp {
            self.0[step.to - 1].push(item);
        }

        Ok(())
    }

    pub fn tops(&self) -> Result<String> {
        self.0
            .iter()
            .map(|stack| stack.last().ok_or_else(|| eyre!("reached empty stack")))
            .collect()
    }
}
