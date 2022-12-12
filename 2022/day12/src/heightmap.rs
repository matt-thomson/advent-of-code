use std::str::FromStr;

use eyre::{eyre, ErrReport, Result};

#[derive(Debug)]
pub struct Heightmap {
    heights: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Heightmap {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let heights = input
            .lines()
            .map(|line| line.chars().map(char_to_height).collect())
            .collect::<Result<Vec<_>>>();

        let start = find_coordinates(input, 'S')?;
        let end = find_coordinates(input, 'E')?;

        Ok(Self {
            heights: heights?,
            start,
            end,
        })
    }
}

fn char_to_height(c: char) -> Result<u8> {
    let normalized = match c {
        'a'..='z' => c,
        'S' => 'a',
        'E' => 'z',
        _ => Err(eyre!("invalid character {c}"))?,
    };

    Ok((normalized as u8) - ('a' as u8))
}

fn find_coordinates(input: &str, target: char) -> Result<(usize, usize)> {
    input
        .lines()
        .map(|line| line.find(target))
        .enumerate()
        .flat_map(|(y, x)| x.map(|x| (x, y)))
        .next()
        .ok_or_else(|| eyre!("couldn't find {target}"))
}
