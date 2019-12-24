use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
}
