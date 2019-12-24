use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const SIZE: usize = 5;

type Position = (usize, usize);

pub struct Board {
    bugs: HashSet<Position>,
}

impl Board {
    pub fn read(path: &Path) -> Self {
        let mut bugs = HashSet::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for (y, row) in reader.lines().enumerate() {
            for (x, position) in row.unwrap().chars().enumerate() {
                if position == '#' {
                    bugs.insert((x, y));
                }
            }
        }

        Self { bugs }
    }

    pub fn step(&self) -> Self {
        let mut bugs = HashSet::new();

        for x in 0..SIZE {
            for y in 0..SIZE {
                let position = (x, y);
                let neighbours = self.neighbours(&position);
                let alive = self.bugs.contains(&position);

                if neighbours == 1 || (neighbours == 2 && !alive) {
                    bugs.insert(position);
                }
            }
        }

        Self { bugs }
    }

    pub fn biodiversity(&self) -> u32 {
        self.bugs.iter().map(|(x, y)| 1 << (y * SIZE + x)).sum()
    }

    fn neighbours(&self, position: &Position) -> usize {
        let mut count = 0;
        let (x, y) = *position;

        if x != 0 && self.bugs.contains(&(x - 1, y)) {
            count += 1;
        }

        if y != 0 && self.bugs.contains(&(x, y - 1)) {
            count += 1;
        }

        if x != SIZE - 1 && self.bugs.contains(&(x + 1, y)) {
            count += 1;
        }

        if y != SIZE - 1 && self.bugs.contains(&(x, y + 1)) {
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

        let mut expected = HashSet::new();
        expected.insert((4, 0));
        expected.insert((0, 1));
        expected.insert((3, 1));
        expected.insert((0, 2));
        expected.insert((3, 2));
        expected.insert((4, 2));
        expected.insert((2, 3));
        expected.insert((0, 4));

        assert_eq!(board.bugs, expected);
    }

    #[test]
    fn test_step() {
        let path = PathBuf::from("fixtures/day24.txt");
        let board = Board::read(&path).step();

        let mut expected = HashSet::new();
        expected.insert((0, 0));
        expected.insert((3, 0));
        expected.insert((0, 1));
        expected.insert((1, 1));
        expected.insert((2, 1));
        expected.insert((3, 1));
        expected.insert((0, 2));
        expected.insert((1, 2));
        expected.insert((2, 2));
        expected.insert((4, 2));
        expected.insert((0, 3));
        expected.insert((1, 3));
        expected.insert((3, 3));
        expected.insert((4, 3));
        expected.insert((1, 4));
        expected.insert((2, 4));

        assert_eq!(board.bugs, expected);
    }
    #[test]
    fn test_biodiversity() {
        let path = PathBuf::from("fixtures/day24.txt");
        let board = Board::read(&path);

        let mut expected = HashSet::new();
        expected.insert((4, 0));
        expected.insert((0, 1));
        expected.insert((3, 1));
        expected.insert((0, 2));
        expected.insert((3, 2));
        expected.insert((4, 2));
        expected.insert((2, 3));
        expected.insert((0, 4));

        assert_eq!(board.biodiversity(), 1_205_552);
    }
}
