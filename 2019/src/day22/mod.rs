mod shuffle;
mod step;

use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use shuffle::Shuffle;

#[derive(Debug, StructOpt)]
pub struct Day22 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day22 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let num_cards = 10_007;
        let shuffle = Shuffle::read(&self.input, num_cards);

        (0..10_007)
            .find(|position| shuffle.card(*position) == 2019)
            .unwrap()
    }

    fn part_two(&self) -> usize {
        let num_cards = 119_315_717_514_047;
        let num_repeats = 101_741_582_076_661;

        let shuffle = Shuffle::read(&self.input, num_cards).repeat(num_repeats);

        shuffle.card(2020)
    }
}
