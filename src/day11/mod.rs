mod colour;
mod direction;
mod rotation;
mod state;

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Intcode;
use crate::problem::Problem;

use colour::Colour;
use rotation::Rotation;
use state::State;

#[derive(Debug, StructOpt)]
pub struct Day11 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day11 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let program: Vec<i64> = fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut intcode = Intcode::new(program);
        let mut state = State::new();

        while !intcode.is_halted() {
            let (colour, rotation) = run(&mut intcode, state.current_colour());
            state.step(colour, rotation);
        }

        state.num_painted()
    }

    fn part_two(&self) -> usize {
        unimplemented!()
    }
}

fn run(intcode: &mut Intcode, colour: &Colour) -> (Colour, Rotation) {
    let output = intcode.run(&[colour.to_i64()]);

    let colour = Colour::from(output[0]);
    let rotation = Rotation::from(output[1]);

    (colour, rotation)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::intcode::Intcode;

    #[test]
    fn test_run() {
        let program = vec![3, 100, 4, 100, 104, 1, 99];
        let mut intcode = Intcode::new(program);

        let (colour, rotation) = run(&mut intcode, &Colour::White);

        assert_eq!(colour, Colour::White);
        assert_eq!(rotation, Rotation::Clockwise);
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day11.txt");
        let problem = Day11 { input };

        assert_eq!(problem.part_one(), 6);
    }
}
