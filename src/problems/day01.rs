use clap::Parser;

use crate::Problem;

#[derive(Debug, Parser)]
pub struct Day01 {
    maximum: u32,
}

impl Problem for Day01 {
    type Output = u32;

    fn part_one(&self) -> u32 {
        (1..=self.maximum).sum()
    }

    fn part_two(&self) -> u32 {
        (1..=self.maximum).map(|x| x * x).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Day01 { maximum: 100 };

        assert_eq!(problem.part_one(), 5050);
    }

    #[test]
    fn test_part_two() {
        let problem = Day01 { maximum: 100 };

        assert_eq!(problem.part_two(), 338350);
    }
}
