use std::collections::{BTreeSet, HashMap, VecDeque};

use super::maze::{Maze, Position};

pub type Routes = HashMap<char, HashMap<char, Route>>;

#[derive(Debug)]
pub struct Route {
    length: usize,
    doors: BTreeSet<char>,
}

impl Route {
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn reachable(&self, keys: &BTreeSet<char>) -> bool {
        self.doors.difference(keys).count() == 0
    }

    fn new() -> Self {
        Self {
            length: 0,
            doors: BTreeSet::new(),
        }
    }

    fn step(&self, door: &Option<&char>) -> Self {
        let mut doors = self.doors.clone();

        if let Some(door) = door {
            doors.insert(**door);
        }

        Self {
            length: self.length + 1,
            doors,
        }
    }
}

pub fn all(maze: &Maze) -> Routes {
    let mut result = HashMap::new();

    result.insert('@', all_from(&maze, maze.entrance()));

    for position in maze.keys() {
        result.insert(*maze.key(&position).unwrap(), all_from(&maze, &position));
    }

    result
}

fn all_from(maze: &Maze, start: &Position) -> HashMap<char, Route> {
    let mut result = HashMap::new();
    let mut visited = BTreeSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((*start, Route::new()));

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
            let new_route = route.step(&door);

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

    use std::path::PathBuf;

    #[test]
    fn test_all_from() {
        let path = PathBuf::from("fixtures/day18a.txt");
        let maze = Maze::read(&path);

        let routes = all_from(&maze, &(5, 1));

        let a_route = routes.get(&'A').unwrap();
        assert_eq!(a_route.length, 2);
        assert_eq!(a_route.doors.len(), 0);

        let b_route = routes.get(&'B').unwrap();
        assert_eq!(b_route.length, 4);
        assert_eq!(b_route.doors.len(), 1);
        assert!(b_route.doors.contains(&'A'));
    }

    #[test]
    fn test_all() {
        let path = PathBuf::from("fixtures/day18a.txt");
        let maze = Maze::read(&path);

        let routes = all(&maze);

        let a_to_b_route = routes.get(&'A').unwrap().get(&'B').unwrap();
        assert_eq!(a_to_b_route.length, 6);
        assert_eq!(a_to_b_route.doors.len(), 1);
        assert!(a_to_b_route.doors.contains(&'A'));

        let entrance_to_a_route = routes.get(&'@').unwrap().get(&'A').unwrap();
        assert_eq!(entrance_to_a_route.length, 2);
        assert_eq!(entrance_to_a_route.doors.len(), 0);
    }

    #[test]
    fn test_reachable() {
        let mut doors = BTreeSet::new();
        doors.insert('A');
        doors.insert('B');

        let route = Route { length: 0, doors };

        let mut keys = BTreeSet::new();
        keys.insert('A');
        keys.insert('C');
        assert!(!route.reachable(&keys));

        keys.insert('B');
        assert!(route.reachable(&keys));
    }
}
