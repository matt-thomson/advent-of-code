use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Maze {
    walls: HashSet<Position>,
}

impl Maze {
    pub fn read(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut walls = HashSet::new();
        let mut letters = HashMap::new();

        for (y, row) in reader.lines().enumerate() {
            for (x, position) in row.unwrap().chars().enumerate() {
                match position {
                    '#' => {
                        walls.insert((x, y));
                    }
                    i if i.is_ascii_uppercase() => {
                        letters.insert((x, y), i);
                    }
                    '.' => (),
                    ' ' => (),
                    _ => panic!("unknown position {}", position),
                }
            }
        }

        Self { walls }
    }
}
