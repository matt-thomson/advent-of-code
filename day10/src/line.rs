use std::{convert::Infallible, str::FromStr};

use crate::bracket::{Bracket, BracketKind, BracketRole};

#[derive(Debug)]
pub struct Line {
    brackets: Vec<Bracket>,
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let brackets = line.chars().map(Bracket::from_char).collect();

        Ok(Self { brackets })
    }
}

impl Line {
    pub fn illegal_bracket(&self) -> Option<&BracketKind> {
        let mut chunks = vec![];

        for bracket in &self.brackets {
            if bracket.role == BracketRole::Open {
                chunks.push(&bracket.kind);
            } else {
                let expected = chunks.pop().unwrap();
                if expected != &bracket.kind {
                    return Some(&bracket.kind);
                }
            }
        }

        None
    }
}
