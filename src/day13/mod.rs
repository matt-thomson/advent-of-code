mod tile;

use std::collections::HashMap;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

use tile::Tile;

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

        let mut screen: HashMap<(i64, i64), Tile> = HashMap::new();

        for chunk in output.chunks(3) {
            screen.insert((chunk[0], chunk[1]), Tile::from(chunk[2]));
        }

        screen
            .iter()
            .filter(|(_, tile)| **tile == Tile::Block)
            .count()
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}
