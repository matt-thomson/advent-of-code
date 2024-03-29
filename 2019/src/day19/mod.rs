mod beam;
mod scan;
mod square;

use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use beam::Beam;
use scan::Scan;
use square::Square;

pub type Position = (usize, usize);

#[derive(Debug, StructOpt)]
pub struct Day19 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day19 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let mut beam = Beam::new(&self.input);

        (0..50)
            .flat_map(|x| (0..50).map(move |y| (x, y)))
            .filter(|position| beam.contains(position))
            .count()
    }

    fn part_two(&self) -> usize {
        let mut beam = Beam::new(&self.input);
        let scan = Scan::new();

        for start in scan {
            let mut square = Square::new(&start, 100);

            if square.all(|position| beam.contains(&position)) {
                let (x, y) = start;
                return x * 10000 + y;
            }
        }

        unreachable!();
    }
}
