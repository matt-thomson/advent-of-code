use std::convert::Infallible;
use std::str::FromStr;

const SEGMENTS: [[bool; 7]; 10] = [
    [true, true, true, false, true, true, true],
    [false, false, true, false, false, true, false],
    [true, false, true, true, true, false, true],
    [true, false, true, true, false, true, true],
    [false, true, true, true, false, true, false],
    [true, true, false, true, false, true, true],
    [true, true, false, true, true, true, true],
    [true, false, true, false, false, true, false],
    [true, true, true, true, true, true, true],
    [true, true, true, true, false, true, true],
];

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

    pub fn apply_mapping(&self, mapping: [usize; 7]) -> Digit {
        let mut segments: [bool; 7] = Default::default();

        for idx in 0..7 {
            segments[mapping[idx]] = self.segments[idx];
        }

        Self { segments }
    }

    pub fn value(&self) -> usize {
        SEGMENTS
            .iter()
            .position(|&segments| segments == self.segments)
            .unwrap()
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
