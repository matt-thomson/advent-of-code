use std::collections::{HashMap, HashSet};

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Maze {
    entrance: Position,
    keys: HashMap<Position, usize>,
    doors: HashMap<Position, usize>,
    walls: HashSet<Position>,
}

impl Maze {
    pub fn parse(input: &str) -> Self {
        let mut entrance = None;
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        let mut walls = HashSet::new();

        for (y, row) in input.lines().enumerate() {
            for (x, position) in row.chars().enumerate() {
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

    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_parse() {
        let path = PathBuf::from("fixtures/day18a.txt");
        let input = fs::read_to_string(&path).unwrap();
        let maze = Maze::parse(&input);

        assert_eq!(*maze.entrance(), (5, 1));

        assert_eq!(*maze.key(&(7, 1)).unwrap(), 1);
        assert_eq!(*maze.key(&(1, 1)).unwrap(), 2);

        assert_eq!(*maze.door(&(3, 1)).unwrap(), 1);

        assert!(maze.is_wall(&(0, 1)));
        assert!(!maze.is_wall(&(2, 1)));
    }
}
