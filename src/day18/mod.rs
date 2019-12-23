mod maze;
mod route;

use std::fs;
use std::path::PathBuf;

use fixedbitset::FixedBitSet;
use pathfinding::prelude::dijkstra;
use structopt::StructOpt;

use crate::problem::Problem;

use maze::Maze;
use route::Routes;

#[derive(Debug, StructOpt)]
pub struct Day18 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day18 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let input = fs::read_to_string(&self.input).unwrap();
        let maze = Maze::parse(&input);
        let routes = route::all(&maze, &maze.entrances()[0]);

        let (_, shortest) = dijkstra(
            &(0, FixedBitSet::with_capacity(maze.keys().len() + 1)),
            |state| neighbours(&state, &routes),
            |(_, keys)| keys.count_ones(..) == routes.len() - 1,
        )
        .unwrap();

        shortest
    }

    fn part_two(&self) -> usize {
        let mut input = fs::read_to_string(&self.input).unwrap();

        let start = input.find('@').unwrap();
        let line_length = input.find('\n').unwrap();

        input.replace_range((start - line_length - 2)..=start - line_length, "@#@");
        input.replace_range((start - 1)..(start + 2), "###");
        input.replace_range((start + line_length)..(start + line_length + 3), "@#@");

        let maze = Maze::parse(&input);
        let routes: Vec<Routes> = maze
            .entrances()
            .iter()
            .map(|entrance| route::all(&maze, entrance))
            .collect();

        let (_, shortest) = dijkstra(
            &(
                [0, 0, 0, 0],
                FixedBitSet::with_capacity(maze.keys().len() + 1),
            ),
            |state| neighbours_quadrants(&state, &routes),
            |(_, keys)| keys.count_ones(..) == maze.keys().len(),
        )
        .unwrap();

        shortest
    }
}

type State = (usize, FixedBitSet);

fn neighbours(state: &State, routes: &Routes) -> Vec<(State, usize)> {
    let (position, keys) = state;

    let possible_routes = routes
        .get(&position)
        .unwrap()
        .iter()
        .filter(|(key, _)| !keys.contains(**key))
        .filter(|(_, route)| route.reachable(&keys));

    let mut result = vec![];

    for (&destination, route) in possible_routes {
        let mut new_keys = keys.clone();
        new_keys.insert(destination);

        let new_state = (destination, new_keys);
        result.push((new_state, route.length()));
    }

    result
}

type StateQuadrants = ([usize; 4], FixedBitSet);

fn neighbours_quadrants(
    state: &StateQuadrants,
    all_routes: &[Routes],
) -> Vec<(StateQuadrants, usize)> {
    let (positions, keys) = state;
    let mut result = vec![];

    for i in 0..4 {
        let position = positions[i];
        let routes = &all_routes[i];

        for ((neighbour, new_keys), distance) in neighbours(&(position, keys.clone()), routes) {
            let mut new_positions = *positions;
            new_positions[i] = neighbour;

            let new_state = (new_positions, new_keys);
            result.push((new_state, distance));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day18a.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_one(), 8);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day18b.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_one(), 86);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day18c.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_one(), 132);
    }

    #[test]
    fn test_part_one_d() {
        let input = PathBuf::from("fixtures/day18d.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_one(), 136);
    }

    #[test]
    fn test_part_one_e() {
        let input = PathBuf::from("fixtures/day18e.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_one(), 81);
    }

    #[test]
    fn test_part_two_f() {
        let input = PathBuf::from("fixtures/day18f.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_two(), 8);
    }

    #[test]
    fn test_part_two_g() {
        let input = PathBuf::from("fixtures/day18g.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_two(), 24);
    }

    #[test]
    fn test_part_two_h() {
        let input = PathBuf::from("fixtures/day18h.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_two(), 32);
    }

    #[test]
    fn test_part_two_i() {
        let input = PathBuf::from("fixtures/day18i.txt");
        let problem = Day18 { input };

        assert_eq!(problem.part_two(), 72);
    }
}
