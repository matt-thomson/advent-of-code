use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Maze {
    start: Position,
    end: Position,
    spaces: HashSet<Position>,
    portals: HashMap<Position, Position>,
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

        let mut raw_portals: Vec<(Position, (char, char))> = vec![];

        for (&(x, y), i) in &letters {
            if let Some(&other) = letters.get(&(x, y + 1)) {
                if spaces.contains(&(x, y + 2)) {
                    raw_portals.push(((x, y + 2), (*i, other)));
                } else if spaces.contains(&(x, y - 1)) {
                    raw_portals.push(((x, y - 1), (*i, other)));
                }
            } else if let Some(&other) = letters.get(&(x + 1, y)) {
                if spaces.contains(&(x + 2, y)) {
                    raw_portals.push(((x + 2, y), (*i, other)));
                } else if spaces.contains(&(x - 1, y)) {
                    raw_portals.push(((x - 1, y), (*i, other)));
                }
            }
        }

        let mut portals = HashMap::new();
        let mut unpaired = HashMap::new();

        for (position, name) in raw_portals {
            match unpaired.get(&name) {
                Some(other) => {
                    portals.insert(position, *other);
                    portals.insert(*other, position);

                    unpaired.remove(&name);
                }
                None => {
                    unpaired.insert(name, position);
                }
            }
        }

        let start = *unpaired.get(&('A', 'A')).unwrap();
        let end = *unpaired.get(&('Z', 'Z')).unwrap();

        Self {
            start,
            end,
            spaces,
            portals,
        }
    }

    pub fn start(&self) -> &Position {
        &self.start
    }

    pub fn end(&self) -> &Position {
        &self.end
    }

    pub fn neighbours(&self, position: &Position) -> Vec<Position> {
        let (x, y) = *position;
        let candidates = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        let mut result = vec![];

        for candidate in &candidates {
            if self.spaces.contains(&candidate) {
                result.push(*candidate);
            }
        }

        if let Some(out) = self.portals.get(&position) {
            result.push(*out);
        }

        result
    }
}
