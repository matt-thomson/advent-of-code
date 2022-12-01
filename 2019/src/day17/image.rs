use std::collections::HashSet;

use super::direction::{Direction, Rotation};
use super::instruction::Instruction;
use super::pixel::Pixel;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Image {
    scaffolds: HashSet<Position>,
    robot: Position,
    direction: Direction,
}

impl Image {
    pub fn new(input: &[i64]) -> Self {
        let mut scaffolds = HashSet::new();
        let mut robot = None;
        let mut direction: Option<Direction> = None;

        for (y, row) in input.split(|&c| c == 10).enumerate() {
            if row.is_empty() {
                break;
            };

            for (x, c) in row.iter().map(|&c| (c as u8) as char).enumerate() {
                match Pixel::from(c) {
                    Pixel::OpenSpace => (),
                    Pixel::Scaffold => {
                        scaffolds.insert((x, y));
                    }
                    Pixel::Robot(d) => {
                        robot = Some((x, y));
                        direction = Some(d);
                    }
                }
            }
        }

        Self {
            scaffolds,
            robot: robot.unwrap(),
            direction: direction.unwrap(),
        }
    }

    pub fn intersections(&self) -> Vec<&Position> {
        self.scaffolds
            .iter()
            .filter(|scaffold| self.is_intersection(scaffold))
            .collect()
    }

    pub fn path(&self) -> Vec<Instruction> {
        let mut result = vec![];
        let mut position = self.robot;
        let mut direction = self.direction;

        loop {
            let mut steps = 0;

            loop {
                if let Some(next) = direction.step(&position) {
                    if self.scaffolds.contains(&next) {
                        steps += 1;
                        position = next;
                        continue;
                    }
                }

                break;
            }

            if steps > 0 {
                result.push(Instruction::Move(steps));
            }

            if let Some(next) = direction.turn(Rotation::AntiClockwise).step(&position) {
                if self.scaffolds.contains(&next) {
                    direction = direction.turn(Rotation::AntiClockwise);
                    result.push(Instruction::TurnLeft);
                    continue;
                }
            }

            if let Some(next) = direction.turn(Rotation::Clockwise).step(&position) {
                if self.scaffolds.contains(&next) {
                    direction = direction.turn(Rotation::Clockwise);
                    result.push(Instruction::TurnRight);
                    continue;
                }
            }

            break;
        }

        result
    }

    fn is_intersection(&self, position: &Position) -> bool {
        let (x, y) = *position;

        if x == 0 || y == 0 {
            return false;
        }

        Direction::all()
            .iter()
            .flat_map(|direction| direction.step(position))
            .all(|neighbour| self.scaffolds.contains(&neighbour))
    }
}
