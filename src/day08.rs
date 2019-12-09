use std::fs;
use std::path::PathBuf;

use bytecount;
use itertools::Itertools;
use structopt::StructOpt;

use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day08 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "25")]
    width: usize,
    #[structopt(default_value = "6")]
    height: usize,
}

impl Problem for Day08 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let input = fs::read_to_string(&self.input).unwrap();
        let layer = input
            .trim()
            .as_bytes()
            .chunks(self.width * self.height)
            .min_by_key(|l| count(l, 0))
            .unwrap();

        count(layer, 1) * count(layer, 2)
    }

    fn part_two(&self) -> usize {
        for row in self.image() {
            println!("{}", format_row(&row));
        }

        0
    }
}

impl Day08 {
    fn image(&self) -> Vec<Vec<bool>> {
        let input = fs::read_to_string(&self.input).unwrap();
        let bytes = input.trim().as_bytes();

        let mut image = vec![];

        for i in 0..self.height {
            let mut row = vec![];

            for j in 0..self.width {
                let mut position = i * self.width + j;

                loop {
                    let pixel = bytes[position];

                    if pixel != 50 {
                        row.push(pixel == 48);
                        break;
                    }

                    position += self.width * self.height;
                }
            }

            image.push(row);
        }

        image
    }
}

fn count(bytes: &[u8], digit: u8) -> usize {
    let target = digit + 48;
    bytecount::count(bytes, target)
}

fn format_row(row: &[bool]) -> String {
    row.iter().map(|x| if *x { ' ' } else { '█' }).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::problem::Problem;

    #[test]
    fn test_count() {
        let layer = "012304560789";
        assert_eq!(count(layer.as_bytes(), 0), 3);
    }

    #[test]
    fn test_image() {
        let input = PathBuf::from("fixtures/day08b.txt");

        let problem = Day08 {
            input,
            width: 2,
            height: 2,
        };

        assert_eq!(problem.image(), vec![vec![true, false], vec![false, true]]);
    }

    #[test]
    fn test_format_row() {
        let row = vec![true, false, true, false, true];
        assert_eq!(format_row(&row), " █ █ ");
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day08a.txt");

        let problem = Day08 {
            input,
            width: 3,
            height: 2,
        };

        assert_eq!(problem.part_one(), 2);
    }
}
