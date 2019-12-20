use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Maze {
    spaces: HashSet<Position>,
}

impl Maze {
    pub fn read(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut spaces = HashSet::new();
        let mut letters = HashMap::new();

        for (y, row) in reader.lines().enumerate() {
            for (x, position) in row.unwrap().chars().enumerate() {
                match position {
                    '#' => (),
                    i if i.is_ascii_uppercase() => {
                        letters.insert((x, y), i);
                    }
                    '.' => {
                        spaces.insert((x, y));
                    }
                    ' ' => (),
                    _ => panic!("unknown position {}", position),
                }
            }
        }

        let mut portals: Vec<(Position, (char, char))> = vec![];

        for (&(x, y), i) in &letters {
            if let Some(&other) = letters.get(&(x, y + 1)) {
                if spaces.contains(&(x, y + 2)) {
                    portals.push(((x, y + 2), (*i, other)));
                } else if spaces.contains(&(x, y - 1)) {
                    portals.push(((x, y - 1), (*i, other)));
                }
            } else if let Some(&other) = letters.get(&(x + 1, y)) {
                if spaces.contains(&(x + 2, y)) {
                    portals.push(((x + 2, y), (*i, other)));
                } else if spaces.contains(&(x - 1, y)) {
                    portals.push(((x - 1, y), (*i, other)));
                }
            }
        }

        println!("{:?}", portals);

        Self { spaces }
    }
}
