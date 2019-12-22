use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Position = (usize, usize);
pub type PositionWithLevel = (Position, usize);

#[derive(Debug)]
pub struct Maze {
    start: Position,
    end: Position,
    spaces: HashSet<Position>,
    inner_portals: HashMap<Position, Position>,
    outer_portals: HashMap<Position, Position>,
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

        let mut raw_inner = HashMap::new();
        let mut raw_outer = HashMap::new();

        let x_max = letters.keys().map(|(x, _)| *x).max().unwrap();
        let y_max = letters.keys().map(|(_, y)| *y).max().unwrap();

        for (&(x, y), i) in &letters {
            if let Some(&other) = letters.get(&(x, y + 1)) {
                let pair = (*i, other);

                if y == 0 {
                    raw_outer.insert(pair, (x, 2));
                } else if y == y_max - 1 {
                    raw_outer.insert(pair, (x, y_max - 2));
                } else if spaces.contains(&(x, y + 2)) {
                    raw_inner.insert(pair, (x, y + 2));
                } else if spaces.contains(&(x, y - 1)) {
                    raw_inner.insert(pair, (x, y - 1));
                }
            } else if let Some(&other) = letters.get(&(x + 1, y)) {
                let pair = (*i, other);

                if x == 0 {
                    raw_outer.insert(pair, (2, y));
                } else if x == x_max - 1 {
                    raw_outer.insert(pair, (x_max - 2, y));
                } else if spaces.contains(&(x + 2, y)) {
                    raw_inner.insert(pair, (x + 2, y));
                } else if spaces.contains(&(x - 1, y)) {
                    raw_inner.insert(pair, (x - 1, y));
                }
            }
        }

        let start = raw_outer.remove(&('A', 'A')).unwrap();
        let end = raw_outer.remove(&('Z', 'Z')).unwrap();

        let mut inner_portals = HashMap::new();
        let mut outer_portals = HashMap::new();

        for (pair, &inner) in raw_inner.iter() {
            let outer = *raw_outer.get(pair).unwrap();

            inner_portals.insert(inner, outer);
            outer_portals.insert(outer, inner);
        }

        Self {
            start,
            end,
            spaces,
            inner_portals,
            outer_portals,
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

        if let Some(out) = self.inner_portals.get(&position) {
            result.push(*out);
        }

        if let Some(out) = self.outer_portals.get(&position) {
            result.push(*out);
        }

        result
    }

    pub fn neighbours_with_level(
        &self,
        position_with_level: &PositionWithLevel,
    ) -> Vec<PositionWithLevel> {
        let (position, level) = *position_with_level;
        let (x, y) = position;

        let candidates = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        let mut result = vec![];

        for candidate in &candidates {
            if self.spaces.contains(&candidate) {
                result.push((*candidate, level));
            }
        }

        if let Some(out) = self.inner_portals.get(&position) {
            result.push((*out, level + 1));
        }

        if level > 0 {
            if let Some(out) = self.outer_portals.get(&position) {
                result.push((*out, level - 1));
            }
        }

        result
    }
}
