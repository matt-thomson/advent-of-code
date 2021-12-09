use std::collections::{HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid {
    heights: Vec<Vec<u32>>,
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let heights = input
            .lines()
            .map(|line| line.bytes().map(|c| (c - b'0') as u32).collect())
            .collect();

        Ok(Self { heights })
    }
}

impl Grid {
    pub fn low_points(&self) -> Vec<(usize, usize)> {
        (0..self.heights[0].len())
            .flat_map(|x| (0..self.heights.len()).map(move |y| (x, y)))
            .filter(|&position| self.is_low_point(position))
            .collect()
    }

    pub fn basin_size(&self, low_point: (usize, usize)) -> usize {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(low_point);

        while let Some(position) = queue.pop_front() {
            seen.insert(position);

            self.neighbours(position)
                .into_iter()
                .filter(|&neighbour| self.height(neighbour) < 9)
                .filter(|neighbour| !seen.contains(neighbour))
                .for_each(|neighbour| queue.push_back(neighbour));
        }

        seen.len()
    }

    pub fn height(&self, (x, y): (usize, usize)) -> u32 {
        self.heights[y][x]
    }

    fn is_low_point(&self, position: (usize, usize)) -> bool {
        self.neighbours(position)
            .iter()
            .all(|&neighbour| self.height(neighbour) > self.height(position))
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];

        if x > 0 {
            result.push((x - 1, y));
        }

        if x < self.heights[0].len() - 1 {
            result.push((x + 1, y))
        }

        if y > 0 {
            result.push((x, y - 1));
        }

        if y < self.heights.len() - 1 {
            result.push((x, y + 1));
        }

        result
    }
}
