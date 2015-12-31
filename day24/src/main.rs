use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> u64 {
    solve(filename, 3)
}

fn solve_part_two(filename: &str) -> u64 {
    solve(filename, 4)
}

fn solve(filename: &str, num_splits: u32) -> u64 {
    let numbers = numbers(filename);
    let target = numbers.iter().fold(0, |acc, i| acc + i) / num_splits;

    for max_size in 1.. {
        let mut groups = find_groups_up_to(&numbers, target, max_size);

        if !groups.is_empty() {
            groups.sort_by(|a, b| qe(a).cmp(&qe(b)));
            return qe(&groups[0]);
        }
    }

    unreachable!();
}

fn find_groups_up_to(numbers: &[u32], target: u32, max_size: usize) -> Vec<Vec<u32>> {
    let mut groups = vec![];
    find_groups(&mut groups, numbers, vec![], 0, 0, target, max_size);

    groups
}

fn find_groups(groups: &mut Vec<Vec<u32>>,
               numbers: &[u32],
               current: Vec<u32>,
               current_sum: u32,
               current_size: usize,
               target: u32,
               max_size: usize) {
    if current_sum == target {
        groups.push(current);
    } else if !numbers.is_empty() && current_size < max_size {
        let next_number = numbers[0];
        let other_numbers = &numbers[1..];

        if current_sum + next_number <= target {
            let mut with_next = current.clone();
            with_next.push(next_number);

            find_groups(groups,
                        other_numbers,
                        with_next,
                        current_sum + next_number,
                        current_size + 1,
                        target,
                        max_size);
        }

        find_groups(groups,
                    other_numbers,
                    current,
                    current_sum,
                    current_size,
                    target,
                    max_size);
    }
}

fn numbers(filename: &str) -> Vec<u32> {
    let file = BufReader::new(File::open(filename).unwrap());
    file.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

fn qe(numbers: &[u32]) -> u64 {
    numbers.iter().fold(1, |acc, i| acc * (*i as u64))
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 99);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 44);
    }
}
