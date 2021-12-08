use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Digit {
    segments: [bool; 7],
}

impl Digit {
    pub fn segments(&self) -> Vec<usize> {
        self.segments
            .iter()
            .enumerate()
            .filter(|(_, val)| **val)
            .map(|(idx, _)| idx)
            .collect()
    }
}

impl FromStr for Digit {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut segments: [bool; 7] = Default::default();

        input
            .bytes()
            .for_each(|c| segments[(c - b'a') as usize] = true);

        Ok(Self { segments })
    }
}
