mod point;
mod reading;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

use crate::reading::Reading;

#[derive(Debug)]
pub struct Problem {
    readings: Vec<Reading>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let readings: Result<Vec<Reading>> = input.lines().map(str::parse).collect();

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

    pub fn part_two(&self, maximum: i64) -> Result<i64> {
        for y in 0..=maximum {
            let mut x = 0;

            'outer: while x < maximum {
                for reading in &self.readings {
                    let beacon_distance = reading.sensor().distance(reading.beacon());
                    let row_distance = reading.sensor().y().abs_diff(y);

                    if let Some(range) = beacon_distance.checked_sub(row_distance) {
                        if x.abs_diff(reading.sensor().x()) <= range {
                            x = reading.sensor().x() + (range as i64) + 1;
                            continue 'outer;
                        }
                    }
                }

                return Ok(x * 4_000_000 + y);
            }
        }

        Err(eyre!("couldn't find distress beacon"))
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

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two(20).unwrap();

        assert_eq!(result, 56_000_011);
    }
}
