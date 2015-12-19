use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    let capacity = env::args().nth(2).expect("Must supply a capacity").parse().unwrap();

    println!("{}", solve_part_one(&filename, capacity));
    println!("{}", solve_part_two(&filename, capacity));
}

fn solve_part_one(filename: &str, capacity: u32) -> u32 {
    let file = BufReader::new(File::open(filename).unwrap());
    let containers: Vec<u32> = file.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    solve(&containers, capacity, 0).values().fold(0, |acc, i| acc + i)
}

fn solve_part_two(filename: &str, capacity: u32) -> u32 {
    let file = BufReader::new(File::open(filename).unwrap());
    let containers: Vec<u32> = file.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    let result = solve(&containers, capacity, 0);
    let min_containers = result.keys().min().unwrap();
    *result.get(min_containers).unwrap()
}

fn solve(containers: &[u32], capacity: u32, num_containers: u32) -> BTreeMap<u32, u32> {
    if capacity == 0 {
        let mut result = BTreeMap::new();
        result.insert(num_containers, 1);
        result
    } else if containers.len() == 0 {
        BTreeMap::new()
    } else {
        let next_container = containers[0];
        let other_containers = &containers[1..];

        let without_next_container = solve(other_containers, capacity, num_containers);

        if next_container <= capacity {
            let mut result = solve(other_containers, capacity - next_container, num_containers + 1);

            for (key, value) in without_next_container {
                *result.entry(key).or_insert(0) += value;
            }

            result
        } else {
            without_next_container
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt", 25), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt", 25), 3);
    }
}
