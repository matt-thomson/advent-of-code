use std::collections::HashSet;

use crate::enhancer::Enhancer;

#[derive(Debug)]
pub struct Image {
    pixels: HashSet<(isize, isize)>,
    outside: bool,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Image {
    pub fn parse(lines: &mut dyn Iterator<Item = String>) -> Self {
        let mut pixels = HashSet::new();

        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    pixels.insert((x as isize, y as isize));
                }
            }
        }

        let x_max = pixels.iter().map(|(x, _)| x).max().unwrap() + 1;
        let y_max = pixels.iter().map(|(_, y)| y).max().unwrap() + 1;

        Self {
            pixels,
            outside: false,
            x_min: 0,
            x_max,
            y_min: 0,
            y_max,
        }
    }

    pub fn enhance(&self, enhancer: &Enhancer) -> Self {
        let mut pixels = HashSet::new();

        let x_min = self.x_min - 1;
        let x_max = self.x_max + 1;
        let y_min = self.y_min - 1;
        let y_max = self.y_max + 1;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let neighbours: Vec<bool> = ((y - 1)..=(y + 1))
                    .flat_map(|y| ((x - 1)..=x + 1).map(move |x| self.lit((x, y))))
                    .collect();

                if enhancer.lit(&neighbours) {
                    pixels.insert((x, y));
                }
            }
        }

        let outside = enhancer.lit(&[self.outside; 9]);

        Self {
            pixels,
            outside,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn lit_pixels(&self) -> usize {
        self.pixels.len()
    }

    fn lit(&self, (x, y): (isize, isize)) -> bool {
        if (self.x_min..=self.x_max).contains(&x) && (self.y_min..=self.y_max).contains(&y) {
            self.pixels.contains(&(x, y))
        } else {
            self.outside
        }
    }
}
