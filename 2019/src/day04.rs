use itertools::Itertools;
use structopt::StructOpt;

use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day04 {
    start: u32,
    end: u32,
}

impl Problem for Day04 {
    type Output = usize;

    fn part_one(&self) -> usize {
        self.solve(valid_password_part_one)
    }

    fn part_two(&self) -> usize {
        self.solve(valid_password_part_two)
    }
}

impl Day04 {
    fn solve(&self, predicate: fn(u32) -> bool) -> usize {
        (self.start..=self.end).filter(|x| predicate(*x)).count()
    }
}

fn digits(input: u32) -> Vec<u32> {
    let mut input = input;
    let mut result = vec![];

    while input > 0 {
        result.push(input % 10);
        input /= 10;
    }

    result.reverse();
    result
}

fn valid_password_part_one(password: u32) -> bool {
    let digits = digits(password);
    has_double(&digits) && never_decrease(&digits)
}

fn valid_password_part_two(password: u32) -> bool {
    let digits = digits(password);
    has_exact_double(&digits) && never_decrease(&digits)
}

fn has_double(digits: &[u32]) -> bool {
    digits.windows(2).any(|pair| pair[0] == pair[1])
}

fn has_exact_double(digits: &[u32]) -> bool {
    digits
        .iter()
        .group_by(|x| *x)
        .into_iter()
        .any(|(_, group)| group.count() == 2)
}

fn never_decrease(digits: &[u32]) -> bool {
    digits.windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::problem::Problem;

    #[test]
    fn test_digits() {
        assert_eq!(digits(12345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_valid_password_part_one() {
        assert!(valid_password_part_one(111_111));
        assert!(!valid_password_part_one(223_450));
        assert!(!valid_password_part_one(123_789));
    }

    #[test]
    fn test_valid_password_part_two() {
        assert!(valid_password_part_two(112_233));
        assert!(!valid_password_part_two(123_444));
        assert!(valid_password_part_two(111_122));
    }

    #[test]
    fn test_part_one() {
        let problem = Day04 {
            start: 300,
            end: 400,
        };

        assert_eq!(problem.part_one(), 13);
    }

    #[test]
    fn test_part_two() {
        let problem = Day04 {
            start: 300,
            end: 400,
        };

        assert_eq!(problem.part_two(), 12);
    }
}
