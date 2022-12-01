mod screen;
mod tile;

use std::cmp::Ordering;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

use screen::Screen;

#[derive(Debug, StructOpt)]
pub struct Day13 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day13 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let mut computer = Program::read(&self.input).launch();

        let output = computer.run(&[]);
        let screen = Screen::new(&output);

        screen.num_blocks()
    }

    fn part_two(&self) -> usize {
        let mut computer = Program::read(&self.input).launch();
        computer.poke(0, 2);

        let output = computer.run(&[]);
        let mut screen = Screen::new(&output);

        while !computer.is_halted() {
            let (paddle, _) = screen.paddle();
            let (ball, _) = screen.ball();

            let direction = match paddle.cmp(&ball) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            screen.update(&computer.run(&[direction]));
        }

        assert!(screen.num_blocks() == 0);

        screen.score()
    }
}
