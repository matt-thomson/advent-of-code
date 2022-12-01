mod enhancer;
mod image;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use enhancer::Enhancer;
use image::Image;

#[derive(Debug)]
pub struct Problem {
    enhancer: Enhancer,
    image: Image,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());

        let enhancer = lines.next().unwrap().parse().unwrap();
        let image = Image::parse(&mut lines.skip(1));

        Self { enhancer, image }
    }

    pub fn part_one(&self) -> usize {
        self.solve(2)
    }

    pub fn part_two(&self) -> usize {
        self.solve(50)
    }

    fn solve(&self, iterations: usize) -> usize {
        (0..iterations)
            .fold(self.image.clone(), |image, _| image.enhance(&self.enhancer))
            .lit_pixels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 35);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 3351);
    }
}
