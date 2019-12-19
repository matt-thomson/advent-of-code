mod maze;
mod route;

use std::collections::{BTreeSet, HashMap};
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

        solve(&routes)
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

fn solve(routes: &Routes) -> usize {
    let mut current: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();
    current.insert(('@', BTreeSet::new()), 0);

    loop {
        let (position, keys, distance) = find_shortest(&current);

        let possible_routes: Vec<_> = routes
            .get(&position)
            .unwrap()
            .iter()
            .filter(|(key, _)| !keys.contains(&key))
            .filter(|(_, route)| route.reachable(&keys))
            .collect();

        if possible_routes.is_empty() {
            return distance;
        }

        for (&destination, route) in possible_routes {
            let mut new_keys = keys.clone();
            new_keys.insert(destination);

            let total_distance = distance + route.length();
            let entry = current
                .entry((destination, new_keys))
                .or_insert(total_distance);

            if *entry > total_distance {
                *entry = total_distance;
            }
        }

        current.remove(&(position, keys));
    }
}

fn find_shortest(
    current: &HashMap<(char, BTreeSet<char>), usize>,
) -> (char, BTreeSet<char>, usize) {
    let ((position, keys), distance) = current
        .iter()
        .min_by_key(|((_, keys), distance)| *distance * 100 + keys.len())
        .unwrap();

    (*position, keys.clone(), *distance)
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
    #[ignore]
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
