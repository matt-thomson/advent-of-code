use std::collections::HashMap;

use super::colour::Colour;
use super::direction::Direction;
use super::rotation::Rotation;

pub type Point = (isize, isize);

pub struct State {
    position: Point,
    direction: Direction,
    painted: HashMap<Point, Colour>,
}

impl State {
    pub fn new(initial: Colour) -> State {
        let position = (0, 0);

        let mut painted = HashMap::new();
        painted.insert(position, initial);

        State {
            position,
            direction: Direction::Up,
            painted,
        }
    }

    pub fn step(&mut self, colour: Colour, rotation: Rotation) {
        self.painted.insert(self.position, colour);
        self.direction = self.direction.turn(rotation);
        self.position = self.direction.step(&self.position);
    }

    pub fn current_colour(&self) -> &Colour {
        self.colour(&self.position)
    }

    pub fn painted(&self) -> &HashMap<Point, Colour> {
        &self.painted
    }

    pub fn colour(&self, point: &Point) -> &Colour {
        self.painted.get(point).unwrap_or(&Colour::Black)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut state = State::new(Colour::Black);
        state.step(Colour::White, Rotation::AntiClockwise);

        assert_eq!(state.position, (-1, 0));
        assert_eq!(state.direction, Direction::Left);
        assert_eq!(state.painted.get(&(0, 0)).unwrap(), &Colour::White);
    }
}
