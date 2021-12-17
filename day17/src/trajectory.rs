use crate::{position::Position, target::Target};

#[derive(Debug)]
pub struct Trajectory {
    position: Position,
    max_height: i32,
}

#[derive(Debug)]
pub enum Outcome {
    InFlight,
    HitTarget(i32),
    Overshot,
    Sank,
}

impl Trajectory {
    pub fn new(dx: i32, dy: i32) -> Self {
        let position = Position::new(dx, dy);
        Self {
            position,
            max_height: 0,
        }
    }

    pub fn step(&self) -> Self {
        let position = self.position.step();
        let max_height = self.max_height.max(position.y);

        Self {
            position,
            max_height,
        }
    }

    pub fn outcome(&self, target: &Target) -> Option<Outcome> {
        if target.contains(&self.position) {
            Some(Outcome::HitTarget(self.max_height))
        } else {
            None
        }
    }
}
