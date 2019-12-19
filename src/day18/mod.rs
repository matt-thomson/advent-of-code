mod maze;
mod route;

use std::collections::HashSet;
use std::path::PathBuf;

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

        let mut best = std::usize::MAX;
        solve(&routes, '@', 0, &mut HashSet::new(), &mut best);

        best
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

fn solve(
    routes: &Routes,
    position: char,
    length: usize,
    keys: &mut HashSet<char>,
    best: &mut usize,
) {
    if length >= *best {
        return;
    }

    if keys.len() == routes.len() - 1 {
        *best = length;
        return;
    }

    let mut possible_routes: Vec<_> = routes
        .get(&position)
        .unwrap()
        .iter()
        .filter(|(_, route)| route.reachable(&keys))
        .filter(|(key, _)| !keys.contains(&key))
        .collect();

    possible_routes.sort_by_key(|(_, route)| route.length());

    for (&key, route) in possible_routes {
        keys.insert(key);
        solve(&routes, key, length + route.length(), keys, best);
        keys.remove(&key);
    }
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
