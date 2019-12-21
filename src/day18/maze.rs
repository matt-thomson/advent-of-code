use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Maze {
    entrance: Position,
    keys: HashMap<Position, usize>,
    doors: HashMap<Position, usize>,
    walls: HashSet<Position>,
}

impl Maze {
    pub fn read(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut entrance = None;
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        let mut walls = HashSet::new();

        for (y, row) in reader.lines().enumerate() {
            for (x, position) in row.unwrap().chars().enumerate() {
                match position {
                    '@' => {
                        entrance = Some((x, y));
                    }
                    '.' => (),
                    '#' => {
                        walls.insert((x, y));
                    }
                    i if i.is_ascii_uppercase() => {
                        doors.insert((x, y), i as usize - 64);
                    }
                    i if i.is_ascii_lowercase() => {
                        keys.insert((x, y), i as usize - 96);
                    }
                    _ => panic!("unknown pixel {}", position),
                }
            }
        }

        Self {
            entrance: entrance.unwrap(),
            keys,
            doors,
            walls,
        }
    }

    pub fn entrance(&self) -> &Position {
        &self.entrance
    }

    pub fn keys(&self) -> Vec<&Position> {
        self.keys.keys().collect()
    }

    pub fn key(&self, position: &Position) -> Option<&usize> {
        self.keys.get(position)
    }

    pub fn door(&self, position: &Position) -> Option<&usize> {
        self.doors.get(position)
    }

    pub fn is_wall(&self, position: &Position) -> bool {
        self.walls.contains(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_read() {
        let path = PathBuf::from("fixtures/day18a.txt");
        let maze = Maze::read(&path);

        assert_eq!(*maze.entrance(), (5, 1));

        assert_eq!(*maze.key(&(7, 1)).unwrap(), 1);
        assert_eq!(*maze.key(&(1, 1)).unwrap(), 2);

        assert_eq!(*maze.door(&(3, 1)).unwrap(), 1);

        assert!(maze.is_wall(&(0, 1)));
        assert!(!maze.is_wall(&(2, 1)));
    }
}
