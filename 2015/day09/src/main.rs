#[macro_use]
extern crate nom;

mod link;
mod map;

use link::Link;
use map::Map;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> u32 {
    let predicate = { |distance, shortest| distance < shortest };

    *solve(filename, &predicate).iter().min().unwrap()
}

fn solve_part_two(filename: &str) -> u32 {
    let predicate = { |distance, shortest| distance > shortest };

    *solve(filename, &predicate).iter().max().unwrap()
}

fn solve(filename: &str, predicate: &Fn(u32, u32) -> bool) -> Vec<u32> {
     let file = BufReader::new(File::open(filename).unwrap());
     let links = file.lines().map(|line| Link::parse(&line.unwrap()));
     let map = Map::new(links);

     map.places().iter().map(|place| find_route(&map, &predicate, place, &vec![], 0)).collect()
 }

fn find_route(map: &Map,
              predicate: &Fn(u32, u32) -> bool,
              position: &str,
              visited: &Vec<String>,
              current: u32) -> u32 {
    let mut new_visited = visited.clone();
    new_visited.push(position.to_string());

    if new_visited.len() == map.places().len() {
        return current;
    }

    let mut best = None;

    for place in map.places() {
        if new_visited.contains(place) {
            continue;
        }

        let step = current + map.distance(position, place);
        let distance = find_route(map, predicate, &place, &new_visited, step);

        best = match best {
            Some(prev_best) => if predicate(distance, prev_best) { Some(distance) } else { Some(prev_best) },
            None => Some(distance)
        };
    }

    best.unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 605);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 982);
    }
}
