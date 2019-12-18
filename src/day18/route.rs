use std::collections::{HashMap, HashSet};

use super::maze::{Maze, Position};

#[derive(Debug)]
pub struct Route {
    length: usize,
    doors: HashSet<char>,
}

pub fn all(maze: &Maze) -> HashMap<char, HashMap<char, Route>> {
    let mut result = HashMap::new();

    result.insert('@', all_from(&maze, maze.entrance()));

    for position in maze.keys() {
        result.insert(*maze.key(&position).unwrap(), all_from(&maze, &position));
    }

    result
}

fn all_from(maze: &Maze, start: &Position) -> HashMap<char, Route> {
    let mut result = HashMap::new();
    dfs(
        &maze,
        start,
        0,
        &mut HashSet::new(),
        &mut vec![],
        &mut result,
    );

    result
}

fn dfs(
    maze: &Maze,
    position: &Position,
    length: usize,
    visited: &mut HashSet<Position>,
    doors: &mut Vec<char>,
    result: &mut HashMap<char, Route>,
) {
    let (x, y) = *position;
    let neighbours = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

    visited.insert(*position);

    if let Some(key) = maze.key(position) {
        if length > 0 {
            let route = Route {
                length,
                doors: doors.iter().cloned().collect(),
            };
            result.insert(*key, route);
        }
    }

    if let Some(door) = maze.door(position) {
        doors.push(*door);
    }

    for neighbour in &neighbours {
        if visited.contains(&neighbour) {
            continue;
        }

        if maze.is_wall(&neighbour) {
            continue;
        }

        dfs(&maze, neighbour, length + 1, visited, doors, result);
    }

    if maze.door(position).is_some() {
        doors.pop();
    }
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
}
