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

    Ok((normalized as u8) - b'a')
}

fn find_coordinates(input: &str, target: char) -> Result<(usize, usize)> {
    input
        .lines()
        .map(|line| line.find(target))
        .enumerate()
        .find_map(|(y, x)| x.map(|x| (x, y)))
        .ok_or_else(|| eyre!("couldn't find {target}"))
}

impl Heightmap {
    pub fn start(&self) -> &(usize, usize) {
        &self.start
    }

    pub fn end(&self) -> &(usize, usize) {
        &self.end
    }

    pub fn neighbours(&self, from @ (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        [
            x.checked_sub(1).map(|x| (x, y)),
            Some((x + 1, y)),
            y.checked_sub(1).map(|y| (x, y)),
            Some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|(x, _)| *x < self.heights[0].len())
        .filter(|(_, y)| *y < self.heights.len())
        .filter(|to| self.height(*to) + 1 >= self.height(from))
        .collect()
    }

    pub fn height(&self, (x, y): (usize, usize)) -> u8 {
        self.heights[y][x]
    }
}
