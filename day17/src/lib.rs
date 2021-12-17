mod target;

use std::fs;
use std::path::Path;

use target::Target;

#[derive(Debug)]
pub struct Problem {
    target: Target,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let input = fs::read_to_string(path).unwrap();
        let target = input.trim().parse().unwrap();

        Self { target }
    }

    pub fn part_one(&self) -> usize {
        dbg!(self);

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 45);
    }
}
