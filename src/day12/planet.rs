use std::cmp::Ordering;

const DIMENSIONS: usize = 3;

type Coordinates = [isize; DIMENSIONS];

#[derive(Clone, Copy, Debug)]
pub struct Planet {
    position: Coordinates,
    velocity: Coordinates,
}

impl Planet {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split(' ');

        let x = Self::parse_part(parts.next().unwrap());
        let y = Self::parse_part(parts.next().unwrap());
        let z = Self::parse_part(parts.next().unwrap());

        Planet {
            position: [x, y, z],
            velocity: [0, 0, 0],
        }
    }

    pub fn apply_gravity(&mut self, other: &Planet) {
        let deltas: Vec<_> = self
            .position
            .iter()
            .zip(other.position.iter())
            .map(|(&first, &second)| Self::velocity_delta(first, second))
            .collect();

        deltas
            .iter()
            .enumerate()
            .for_each(|(i, delta)| self.velocity[i] += delta);
    }

    pub fn step(&mut self) {
        for i in 0..DIMENSIONS {
            self.position[i] += self.velocity[i];
        }
    }

    pub fn energy(&self) -> usize {
        let potential: usize = self.position.iter().map(|pos| pos.abs() as usize).sum();
        let kinetic: usize = self.velocity.iter().map(|pos| pos.abs() as usize).sum();

        potential * kinetic
    }

    fn parse_part(input: &str) -> isize {
        let equals = input.find('=').unwrap();
        let suffix = input.len() - 1;

        input[(equals + 1)..suffix].parse().unwrap()
    }

    fn velocity_delta(this: isize, other: isize) -> isize {
        match this.cmp(&other) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let planet = Planet::parse("<x=2, y=-10, z=-7>");

        assert_eq!(planet.position, [2, -10, -7]);
        assert_eq!(planet.velocity, [0, 0, 0]);
    }

    #[test]
    fn test_apply_gravity() {
        let mut first = Planet::parse("<x=-1, y=0, z=2>");
        let second = Planet::parse("<x=2, y=-10, z=-7>");

        first.apply_gravity(&second);

        assert_eq!(first.position, [-1, 0, 2]);
        assert_eq!(first.velocity, [1, -1, -1]);
    }

    #[test]
    fn test_step() {
        let mut planet = Planet {
            position: [-1, 0, 2],
            velocity: [3, -1, -1],
        };

        planet.step();

        assert_eq!(planet.position, [2, -1, 1]);
        assert_eq!(planet.velocity, [3, -1, -1]);
    }

    #[test]
    fn test_energy() {
        let planet = Planet {
            position: [-1, 0, 2],
            velocity: [3, -1, -1],
        };

        assert_eq!(planet.energy(), 15);
    }
}
