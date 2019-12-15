mod direction;
mod status;

use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::{Computer, Program};
use crate::problem::Problem;

use direction::Direction;
use status::Status;

#[derive(Debug, StructOpt)]
pub struct Day15 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day15 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let mut computer = Program::read(&self.input).launch();

        find_oxygen(&mut computer, None).unwrap()
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

fn step(computer: &mut Computer, direction: &Direction) -> Status {
    let output = computer.run(&[direction.as_i64()]);
    assert_eq!(output.len(), 1);

    Status::from(output[0])
}

fn find_oxygen(computer: &mut Computer, last: Option<&Direction>) -> Option<usize> {
    for direction in Direction::all() {
        if last == Some(direction.opposite()) {
            continue;
        }

        match step(computer, direction) {
            Status::HitWall => (),
            Status::Moved => {
                if let Some(result) = find_oxygen(computer, Some(direction)) {
                    return Some(result + 1);
                } else {
                    step(computer, direction.opposite());
                }
            }
            Status::FoundOxygen => return Some(1),
        }
    }

    None
}
