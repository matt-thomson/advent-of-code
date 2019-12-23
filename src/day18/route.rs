use std::collections::{BTreeSet, HashMap, VecDeque};

use fixedbitset::FixedBitSet;

use super::maze::{Maze, Position};

pub type Routes = HashMap<usize, HashMap<usize, Route>>;

#[derive(Debug)]
pub struct Route {
    length: usize,
    doors: FixedBitSet,
}

impl Route {
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn reachable(&self, keys: &FixedBitSet) -> bool {
        self.doors.difference(keys).count() == 0
    }

    fn new(max_key: usize) -> Self {
        Self {
            length: 0,
            doors: FixedBitSet::with_capacity(max_key + 1),
        }
    }

    fn step(&self, door: Option<&usize>) -> Self {
        let mut doors = self.doors.clone();

        if let Some(door) = door {
            doors.insert(*door);
        }

        Self {
            length: self.length + 1,
            doors,
        }
    }
}

pub fn all(maze: &Maze, start: &Position) -> Routes {
    let mut result = HashMap::new();

    let all_from_start = all_from(&maze, start);

    for position in maze.keys() {
        let key = maze.key(&position).unwrap();

        if all_from_start.contains_key(key) {
            result.insert(*key, all_from(&maze, &position));
        }
    }

    result.insert(0, all_from_start);

    result
}

fn all_from(maze: &Maze, start: &Position) -> HashMap<usize, Route> {
    let mut result = HashMap::new();
    let mut visited = BTreeSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((*start, Route::new(maze.keys().len())));

    while let Some((position, route)) = queue.pop_front() {
        visited.insert(position);

        let (x, y) = position;
        let neighbours = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        for neighbour in neighbours.iter() {
            if visited.contains(&neighbour) {
                continue;
            }

            if maze.is_wall(&neighbour) {
                continue;
            }

            let door = maze.door(neighbour);
            let new_route = route.step(door);

            queue.push_back((*neighbour, new_route));
        }

        if let Some(key) = maze.key(&position) {
            if route.length() > 0 {
                result.insert(*key, route);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;

    use fixedbitset::FixedBitSet;

    #[test]
    fn test_all_from() {
        let path = PathBuf::from("fixtures/day18a.txt");
        let input = fs::read_to_string(&path).unwrap();
        let maze = Maze::parse(&input);

        let routes = all_from(&maze, &(5, 1));

        let a_route = routes.get(&1).unwrap();
        assert_eq!(a_route.length, 2);
        assert_eq!(a_route.doors.count_ones(..), 0);

        let b_route = routes.get(&2).unwrap();
        assert_eq!(b_route.length, 4);
        assert_eq!(b_route.doors.count_ones(..), 1);
        assert!(b_route.doors.contains(1));
    }

    #[test]
    fn test_all() {
        let path = PathBuf::from("fixtures/day18a.txt");
        let input = fs::read_to_string(&path).unwrap();
        let maze = Maze::parse(&input);

        let routes = all(&maze, &(5, 1));

        let a_to_b_route = routes.get(&1).unwrap().get(&2).unwrap();
        assert_eq!(a_to_b_route.length, 6);
        assert_eq!(a_to_b_route.doors.count_ones(..), 1);
        assert!(a_to_b_route.doors.contains(1));

        let entrance_to_a_route = routes.get(&0).unwrap().get(&1).unwrap();
        assert_eq!(entrance_to_a_route.length, 2);
        assert_eq!(entrance_to_a_route.doors.count_ones(..), 0);
    }

    #[test]
    fn test_reachable() {
        let mut doors = FixedBitSet::with_capacity(4);
        doors.insert(1);
        doors.insert(2);

        let route = Route { length: 0, doors };

        let mut keys = FixedBitSet::with_capacity(4);
        keys.insert(1);
        keys.insert(3);
        assert!(!route.reachable(&keys));

        keys.insert(2);
        assert!(route.reachable(&keys));
    }
}
