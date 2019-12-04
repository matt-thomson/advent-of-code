use structopt::StructOpt;

use crate::command;

#[derive(Debug, StructOpt)]
pub struct Day04 {
    start: u32,
    end: u32,
}

impl command::Command for Day04 {
    fn part_one(&self) -> u32 {
        (self.start..=self.end)
            .filter(|x| valid_password(*x))
            .count() as u32
    }

    fn part_two(&self) -> u32 {
        unimplemented!()
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

fn valid_password(password: u32) -> bool {
    let digits = digits(password);
    has_double(&digits) && never_decrease(&digits)
}

fn has_double(digits: &[u32]) -> bool {
    digits.windows(2).any(|pair| pair[0] == pair[1])
}

fn never_decrease(digits: &[u32]) -> bool {
    digits.windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_digits() {
        assert_eq!(digits(12345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_valid_password() {
        assert!(valid_password(111_111));
        assert!(!valid_password(223_450));
        assert!(!valid_password(123_789));
    }

    #[test]
    fn test_part_one() {
        let command = Day04 {
            start: 300,
            end: 400,
        };

        assert_eq!(command.part_one(), 13);
    }
}
