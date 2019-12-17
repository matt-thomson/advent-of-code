use std::collections::HashSet;

use super::direction::Direction;
use super::pixel::Pixel;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Image {
    scaffolds: HashSet<Position>,
    robot: Position,
}

impl Image {
    pub fn new(input: &[i64]) -> Self {
        let mut scaffolds = HashSet::new();
        let mut robot = None;

        for (y, row) in input.split(|&c| c == 10).enumerate() {
            for (x, c) in row.iter().map(|&c| (c as u8) as char).enumerate() {
                match Pixel::from(c) {
                    Pixel::OpenSpace => (),
                    Pixel::Scaffold => {
                        scaffolds.insert((x, y));
                    }
                    Pixel::Robot => {
                        robot = Some((x, y));
                    }
                }
            }
        }

        Self {
            scaffolds,
            robot: robot.unwrap(),
        }
    }

    pub fn intersections(&self) -> Vec<&Position> {
        self.scaffolds
            .iter()
            .filter(|scaffold| self.is_intersection(&scaffold))
            .collect()
    }

    fn is_intersection(&self, position: &Position) -> bool {
        let (x, y) = *position;

        if x == 0 || y == 0 {
            return false;
        }

        Direction::all()
            .iter()
            .flat_map(|direction| direction.step(&position))
            .all(|neighbour| self.scaffolds.contains(&neighbour))
    }
}
