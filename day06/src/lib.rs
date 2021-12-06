use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    ages: Vec<u32>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let ages = fs::read_to_string(&path)
            .unwrap()
            .trim()
            .split(',')
            .map(|age| age.parse().unwrap())
            .collect();

        Self { ages }
    }

    pub fn part_one(&self) -> usize {
        let ages = (0..80).fold(self.ages.clone(), |current, _| next_day(&current));
        ages.len()
    }
}

fn next_day(ages: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(ages.len());
    let mut new = 0;

    for age in ages {
        if *age > 0 {
            result.push(age - 1);
        } else {
            result.push(6);
            new += 1;
        }
    }

    (0..new).for_each(|_| result.push(8));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 5934);
    }
}
