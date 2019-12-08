use std::fs;
use std::path::PathBuf;

use bytecount;
use structopt::StructOpt;

use crate::command;

#[derive(Debug, StructOpt)]
pub struct Day08 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "25")]
    width: usize,
    #[structopt(default_value = "6")]
    height: usize,
}

impl command::Command for Day08 {
    fn part_one(&self) -> u32 {
        let input = fs::read_to_string(&self.input).unwrap();
        let layer = input
            .trim()
            .as_bytes()
            .chunks(self.width * self.height)
            .min_by_key(|l| count(l, 0))
            .unwrap();

        (count(layer, 1) * count(layer, 2)) as u32
    }

    fn part_two(&self) -> u32 {
        unimplemented!();
    }
}

fn count(bytes: &[u8], digit: u8) -> usize {
    let target = digit + 48;
    bytecount::count(bytes, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_count() {
        let layer = "012304560789";
        assert_eq!(count(layer.as_bytes(), 0), 3);
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day08.txt");
        let command = Day08 {
            input,
            width: 3,
            height: 2,
        };

        assert_eq!(command.part_one(), 2);
    }
}
