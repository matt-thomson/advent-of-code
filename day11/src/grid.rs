use std::collections::VecDeque;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Grid {
    energy_levels: Vec<Vec<u32>>,
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let energy_levels = input
            .lines()
            .map(|line| line.bytes().map(|c| (c - b'0') as u32).collect())
            .collect();

        Ok(Self { energy_levels })
    }
}

impl Grid {
    pub fn step(&mut self) -> usize {
        let mut flashes = 0;
        let mut queue = VecDeque::new();

        for (y, row) in self.energy_levels.iter_mut().enumerate() {
            for (x, level) in row.iter_mut().enumerate() {
                *level += 1;

                if *level > 9 {
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            if self.energy_levels[y][x] == 0 {
                continue;
            }

            for neighbour in self.neighbours((x, y)) {
                if self.apply_flash(neighbour) {
                    queue.push_back(neighbour);
                }
            }

            self.energy_levels[y][x] = 0;

            flashes += 1;
        }

        flashes
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let x_min = if x > 0 { x - 1 } else { 0 };
        let x_max = if x < self.energy_levels[0].len() - 1 {
            x + 1
        } else {
            x
        };

        let y_min = if y > 0 { y - 1 } else { 0 };
        let y_max = if y < self.energy_levels.len() - 1 {
            y + 1
        } else {
            y
        };

        (x_min..=x_max)
            .flat_map(|x| (y_min..=y_max).map(move |y| (x, y)))
            .filter(|position| *position != (x, y))
            .collect()
    }

    fn apply_flash(&mut self, (x, y): (usize, usize)) -> bool {
        if self.energy_levels[y][x] > 0 {
            self.energy_levels[y][x] += 1;
        }

        self.energy_levels[y][x] > 9
    }
}
