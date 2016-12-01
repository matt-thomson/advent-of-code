extern crate regex;

use regex::Regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> usize {
    solve(filename, &count_difference)
}

fn solve_part_two(filename: &str) -> usize {
    solve(filename, &count_extra)
}

fn solve(filename: &str, op: &Fn(&str) -> usize) -> usize {
    let file = BufReader::new(File::open(filename).unwrap());
    file.lines()
        .map(|line| op(&line.unwrap()))
        .fold(0, |acc, i| acc + i)
}

fn count_difference(input: &str) -> usize {
    count_regex(input, "\"") + count_regex(input, "\\\\\\\\") + 3 * count_regex(input, "\\\\x[0-9a-f]{2}")
}

fn count_extra(input: &str) -> usize {
    count_regex(input, "\"") + count_regex(input, "\\\\") + 2
}

fn count_regex(input: &str, regex: &str) -> usize {
    Regex::new(regex).unwrap().find_iter(input).count()
}

#[cfg(test)]
mod tests {
    use super::{count_difference, count_extra, solve_part_one, solve_part_two};

    #[test]
    fn test_count_difference_with_quotes() {
        assert_eq!(count_difference("\"a\""), 2);
    }

    #[test]
    fn test_count_difference_with_slashes() {
        assert_eq!(count_difference("a\\\\a\\\\a"), 2);
    }

    #[test]
    fn test_count_difference_with_hex_character() {
        assert_eq!(count_difference("a\\xaaa\\xaaa"), 6);
    }

    #[test]
    fn test_count_extra_with_quotes() {
        println!("{}", "\"\"");
        assert_eq!(count_extra("\"\""), 4);
    }

    #[test]
    fn test_count_extra_with_text() {
        assert_eq!(count_extra("\"abc\""), 4);
    }

    #[test]
    fn test_count_extra_with_slashes() {
        assert_eq!(count_extra("\"aaa\\\"aaa\""), 6);
    }

    #[test]
    fn test_count_extra_with_hex_character() {
        assert_eq!(count_extra("\"\\x27\""), 5);
    }

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one("data/example.txt");
        assert_eq!(result, 12);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two("data/example.txt");
        assert_eq!(result, 19);
    }
}
