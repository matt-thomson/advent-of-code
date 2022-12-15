mod point;
mod reading;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use eyre::Result;

use crate::reading::Reading;

#[derive(Debug)]
pub struct Problem {
    readings: Vec<Reading>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let readings: Result<Vec<Reading>> = input.lines().map(|line| line.parse()).collect();

        Ok(Self {
            readings: readings?,
        })
    }

    pub fn part_one(&self, y: i64) -> usize {
        let mut clear = HashSet::new();

        for reading in &self.readings {
            let beacon_distance = reading.sensor().distance(reading.beacon());
            let row_distance = reading.sensor().y().abs_diff(y);

            if let Some(range) = beacon_distance.checked_sub(row_distance) {
                for i in 0..=range {
                    clear.insert(reading.sensor().x() + i as i64);
                    clear.insert(reading.sensor().x() - i as i64);
                }
            }
        }

        for reading in &self.readings {
            if reading.beacon().y() == y {
                clear.remove(&reading.beacon().x());
            }
        }

        clear.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one(10);

        assert_eq!(result, 26);
    }
}
