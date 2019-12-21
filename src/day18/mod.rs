mod maze;
mod route;

use std::collections::BTreeSet;
use std::path::PathBuf;

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
        let maze = Maze::read(&self.input);
        let routes = route::all(&maze);

        solve(&routes)
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

type State = (usize, BTreeSet<usize>);

fn solve(routes: &Routes) -> usize {
    let (x, shortest) = dijkstra(
        &(0, BTreeSet::new()),
        |state| neighbours(&state, &routes),
        |(_, keys)| keys.len() == routes.len() - 1,
    )
    .unwrap();

    dbg!(x, shortest);

    shortest
}

fn neighbours(state: &State, routes: &Routes) -> Vec<(State, usize)> {
    let (position, keys) = state;

    let possible_routes = routes
        .get(&position)
        .unwrap()
        .iter()
        .filter(|(key, _)| !keys.contains(&key))
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
}
