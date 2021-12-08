use std::convert::Infallible;
use std::str::FromStr;

use crate::digit::Digit;

#[derive(Debug)]
pub struct Display {
    digits: Vec<Digit>,
    output: Vec<Digit>,
}

impl Display {
    pub fn output(&self) -> Vec<usize> {
        let mapping = self.build_mapping();
        self.output
            .iter()
            .map(|digit| digit.apply_mapping(mapping).value())
            .collect()
    }

    pub fn total(&self) -> usize {
        let mut result = 0;

        for digit in self.output() {
            result *= 10;
            result += digit;
        }

        result
    }

    fn build_mapping(&self) -> [usize; 7] {
        let mut counts: [usize; 7] = Default::default();
        self.digits.iter().for_each(|digit| {
            digit
                .segments()
                .iter()
                .for_each(|&segment| counts[segment] += 1)
        });

        let mut mapping: [usize; 7] = Default::default();

        for (idx, count) in counts.iter().enumerate() {
            match count {
                4 => mapping[idx] = 4,
                6 => mapping[idx] = 1,
                9 => mapping[idx] = 5,
                _ => (),
            }
        }

        let c = *self
            .digit_with_segment_count(2)
            .segments()
            .iter()
            .find(|segment| mapping[**segment] == 0)
            .unwrap();

        mapping[c] = 2;

        let d = *self
            .digit_with_segment_count(4)
            .segments()
            .iter()
            .find(|segment| mapping[**segment] == 0)
            .unwrap();

        mapping[d] = 3;

        let (g, _) = counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count == 7)
            .find(|(idx, _)| mapping[*idx] == 0)
            .unwrap();

        mapping[g] = 6;

        mapping
    }

    fn digit_with_segment_count(&self, count: usize) -> &Digit {
        self.digits
            .iter()
            .find(|digit| digit.segments().len() == count)
            .unwrap()
    }
}

impl FromStr for Display {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (digits, output) = s.split_once('|').unwrap();
        Ok(Self {
            digits: parse_digits(digits),
            output: parse_digits(output),
        })
    }
}

fn parse_digits(input: &str) -> Vec<Digit> {
    input
        .trim()
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect()
}
