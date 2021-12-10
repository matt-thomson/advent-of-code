use std::{convert::Infallible, str::FromStr};

use crate::bracket::{Bracket, BracketRole};
use crate::syntax_error::SyntaxError;

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
    pub fn syntax_error(&self) -> SyntaxError {
        let mut chunks = vec![];

        for bracket in &self.brackets {
            match bracket.role {
                BracketRole::Open => {
                    chunks.push(&bracket.kind);
                }
                BracketRole::Close => {
                    let expected = chunks.pop().unwrap();
                    if expected != &bracket.kind {
                        return SyntaxError::Corrupted(&bracket.kind);
                    }
                }
            }
        }

        chunks.reverse();
        SyntaxError::Incomplete(chunks)
    }
}
