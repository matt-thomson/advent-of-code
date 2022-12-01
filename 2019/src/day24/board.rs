use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use fixedbitset::FixedBitSet;

const SIZE: usize = 5;

pub struct Board {
    bugs: FixedBitSet,
}

impl Board {
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

        Self { bugs }
    }

    pub fn step(&self) -> Self {
        let mut bugs = FixedBitSet::with_capacity(SIZE * SIZE);

        for position in 0..(SIZE * SIZE) {
            let neighbours = self.neighbours(position);
            let alive = self.bugs.contains(position);

            if neighbours == 1 || (neighbours == 2 && !alive) {
                bugs.insert(position);
            }
        }

        Self { bugs }
    }

    pub fn biodiversity(&self) -> u32 {
        self.bugs.as_slice()[0]
    }

    fn neighbours(&self, position: usize) -> usize {
        let mut count = 0;

        if position % SIZE != 0 && self.bugs.contains(position - 1) {
            count += 1;
        }

        if position >= SIZE && self.bugs.contains(position - SIZE) {
            count += 1;
        }

        if position % SIZE != SIZE - 1 && self.bugs.contains(position + 1) {
            count += 1;
        }

        if position < SIZE * SIZE - SIZE && self.bugs.contains(position + SIZE) {
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_read() {
        let path = PathBuf::from("fixtures/day24.txt");
        let board = Board::read(&path);

        let mut expected = FixedBitSet::with_capacity(SIZE * SIZE);
        expected.insert(4);
        expected.insert(5);
        expected.insert(8);
        expected.insert(10);
        expected.insert(13);
        expected.insert(14);
        expected.insert(17);
        expected.insert(20);

        assert_eq!(board.bugs, expected);
    }

    #[test]
    fn test_step() {
        let path = PathBuf::from("fixtures/day24.txt");
        let board = Board::read(&path).step();

        let mut expected = FixedBitSet::with_capacity(SIZE * SIZE);
        expected.insert(0);
        expected.insert(3);
        expected.insert(5);
        expected.insert(6);
        expected.insert(7);
        expected.insert(8);
        expected.insert(10);
        expected.insert(11);
        expected.insert(12);
        expected.insert(14);
        expected.insert(15);
        expected.insert(16);
        expected.insert(18);
        expected.insert(19);
        expected.insert(21);
        expected.insert(22);

        assert_eq!(board.bugs, expected);
    }
    #[test]
    fn test_biodiversity() {
        let path = PathBuf::from("fixtures/day24.txt");
        let board = Board::read(&path);

        assert_eq!(board.biodiversity(), 1_205_552);
    }
}
