mod colour;
mod direction;
mod rotation;
mod state;

use std::path::{Path, PathBuf};

use structopt::StructOpt;

use crate::intcode::{Computer, Program};
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
        identifier(&self.input, Colour::Black).painted().len()
    }

    fn part_two(&self) -> usize {
        let identifier = identifier(&self.input, Colour::White);

        let min_x = *identifier.painted().keys().map(|(x, _)| x).min().unwrap();
        let max_x = *identifier.painted().keys().map(|(x, _)| x).max().unwrap();

        let min_y = *identifier.painted().keys().map(|(_, y)| y).min().unwrap();
        let max_y = *identifier.painted().keys().map(|(_, y)| y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", identifier.colour(&(x, y)));
            }

            println!();
        }

        0
    }
}

fn identifier(path: &Path, initial: Colour) -> State {
    let mut computer = Program::read(path).launch();
    let mut state = State::new(initial);

    while !computer.is_halted() {
        let (colour, rotation) = run(&mut computer, state.current_colour());
        state.step(colour, rotation);
    }

    state
}

fn run(computer: &mut Computer, colour: &Colour) -> (Colour, Rotation) {
    let output = computer.run(&[colour.to_i64()]);

    let colour = Colour::from(output[0]);
    let rotation = Rotation::from(output[1]);

    (colour, rotation)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::intcode::Computer;

    #[test]
    fn test_run() {
        let program = vec![3, 100, 4, 100, 104, 1, 99];
        let mut computer = Computer::new(program);

        let (colour, rotation) = run(&mut computer, &Colour::White);

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
