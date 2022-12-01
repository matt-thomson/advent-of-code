use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    numbers: Vec<Vec<u32>>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let numbers: Vec<Vec<u32>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        Self { numbers }
    }

    pub fn part_one(&self) -> u32 {
        let binary: Vec<_> = (0..self.numbers[0].len())
            .map(|position| most_common(&self.numbers, position))
            .collect();

        let gamma = from_binary(&binary);
        let epsilon = 2_u32.pow(self.numbers[0].len() as u32) - 1 - gamma;

        gamma * epsilon
    }

    pub fn part_two(&self) -> u32 {
        let oxygen_generator_rating = self.calculate_rating(|bit| bit);
        let co2_scrubber_rating = self.calculate_rating(|bit| 1 - bit);

        oxygen_generator_rating * co2_scrubber_rating
    }

    fn calculate_rating<F>(&self, bit_criteria: F) -> u32
    where
        F: Fn(u32) -> u32,
    {
        let mut numbers = self.numbers.clone();

        for position in 0..numbers[0].len() {
            let most_common = most_common(&numbers, position);
            let selected_bit = bit_criteria(most_common);
            numbers.retain(|number| number[position] == selected_bit);

            if numbers.len() == 1 {
                break;
            }
        }

        from_binary(&numbers[0])
    }
}

fn most_common(numbers: &[Vec<u32>], position: usize) -> u32 {
    let count_ones: u32 = numbers.iter().map(|row| row[position]).sum();
    (count_ones * 2) / (numbers.len() as u32)
}

fn from_binary(numbers: &[u32]) -> u32 {
    numbers.iter().fold(0, |acc, number| acc * 2 + number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 198);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 230);
    }
}
