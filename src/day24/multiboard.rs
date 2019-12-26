use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use fixedbitset::FixedBitSet;

const SIZE: usize = 5;

pub struct MultiBoard {
    layers: HashMap<isize, FixedBitSet>,
}

impl MultiBoard {
    pub fn read(path: &Path) -> Self {
        let mut bugs = FixedBitSet::with_capacity(SIZE * SIZE);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for (y, row) in reader.lines().enumerate() {
            for (x, position) in row.unwrap().chars().enumerate() {
                if position == '#' {
                    bugs.insert(y * SIZE + x);
                }
            }
        }

        let mut layers = HashMap::new();
        layers.insert(0, bugs);

        Self { layers }
    }

    pub fn step(&self) -> Self {
        let mut layers = HashMap::new();

        let min_layer = self.layers.keys().min().unwrap() - 1;
        let max_layer = self.layers.keys().max().unwrap() + 1;

        for layer in min_layer..=max_layer {
            let mut bugs = FixedBitSet::with_capacity(SIZE * SIZE);

            for position in 0..(SIZE * SIZE) {
                if position == 12 {
                    continue;
                }

                let neighbours = self.neighbours(layer, position);
                let alive = self.is_bug(layer, position);

                if neighbours == 1 || (neighbours == 2 && !alive) {
                    bugs.insert(position);
                }
            }

            if bugs.count_ones(..) > 0 {
                layers.insert(layer, bugs);
            }
        }
        Self { layers }
    }

    pub fn num_bugs(&self) -> usize {
        self.layers.values().map(|layer| layer.count_ones(..)).sum()
    }

    fn neighbours(&self, layer: isize, position: usize) -> usize {
        candidates(layer, position)
            .iter()
            .filter(|(layer, position)| self.is_bug(*layer, *position))
            .count()
    }

    fn is_bug(&self, layer: isize, position: usize) -> bool {
        self.layers
            .get(&layer)
            .map(|bugs| bugs.contains(position))
            .unwrap_or(false)
    }
}

fn candidates(layer: isize, position: usize) -> Vec<(isize, usize)> {
    let mut candidates = vec![];

    if position == 17 {
        (0..SIZE).for_each(|other| candidates.push((layer - 1, other + SIZE * (SIZE - 1))));
    } else if position < SIZE {
        candidates.push((layer + 1, 7));
    } else {
        candidates.push((layer, position - SIZE));
    }

    if position == 13 {
        (0..SIZE).for_each(|other| candidates.push((layer - 1, (other + 1) * SIZE - 1)));
    } else if position % SIZE == 0 {
        candidates.push((layer + 1, 11));
    } else {
        candidates.push((layer, position - 1));
    }

    if position == 11 {
        (0..SIZE).for_each(|other| candidates.push((layer - 1, other * SIZE)));
    } else if position % SIZE == SIZE - 1 {
        candidates.push((layer + 1, 13));
    } else {
        candidates.push((layer, position + 1));
    }

    if position == 7 {
        (0..SIZE).for_each(|other| candidates.push((layer - 1, other)));
    } else if position >= SIZE * SIZE - SIZE {
        candidates.push((layer + 1, 17));
    } else {
        candidates.push((layer, position + SIZE));
    }

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidates_1() {
        let candidates = candidates(1, 18);
        let expected = vec![(1, 13), (1, 17), (1, 19), (1, 23)];

        assert_eq!(candidates, expected);
    }

    #[test]
    fn test_candidates_2() {
        let candidates = candidates(0, 6);
        let expected = vec![(0, 1), (0, 5), (0, 7), (0, 11)];

        assert_eq!(candidates, expected);
    }

    #[test]
    fn test_candidates_3() {
        let candidates = candidates(0, 3);
        let expected = vec![(1, 7), (0, 2), (0, 4), (0, 8)];

        assert_eq!(candidates, expected);
    }

    #[test]
    fn test_candidates_4() {
        let candidates = candidates(0, 4);
        let expected = vec![(1, 7), (0, 3), (1, 13), (0, 9)];

        assert_eq!(candidates, expected);
    }

    #[test]
    fn test_candidates_5() {
        let candidates = candidates(1, 13);
        let expected = vec![
            (1, 8),
            (0, 4),
            (0, 9),
            (0, 14),
            (0, 19),
            (0, 24),
            (1, 14),
            (1, 18),
        ];

        assert_eq!(candidates, expected);
    }

    #[test]
    fn test_candidates_6() {
        let candidates = candidates(0, 13);
        let expected = vec![
            (0, 8),
            (-1, 4),
            (-1, 9),
            (-1, 14),
            (-1, 19),
            (-1, 24),
            (0, 14),
            (0, 18),
        ];

        assert_eq!(candidates, expected);
    }
}
