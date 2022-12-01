use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
pub struct Enhancer {
    algorithm: [bool; 512],
}

impl FromStr for Enhancer {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let algorithm = input
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self { algorithm })
    }
}
impl Enhancer {
    pub fn lit(&self, neighbours: &[bool]) -> bool {
        let value = neighbours
            .iter()
            .map(|&x| if x { 1 } else { 0 })
            .fold(0, |acc, x| acc * 2 + x);

        self.algorithm[value]
    }
}
