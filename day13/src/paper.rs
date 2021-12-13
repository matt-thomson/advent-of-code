use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use crate::fold::Fold;

#[derive(Clone, Debug)]
pub struct Paper {
    dots: BTreeSet<(usize, usize)>,
}

impl Paper {
    pub fn parse(lines: &mut dyn Iterator<Item = String>) -> Self {
        let dots = lines.map(|line| parse_dot(&line)).collect();

        Self { dots }
    }

    pub fn apply_fold(&self, fold: &Fold) -> Self {
        let dots = self.dots.iter().map(|dot| fold.image(dot)).collect();

        Self { dots }
    }

    pub fn num_dots(&self) -> usize {
        self.dots.len()
    }
}

fn parse_dot(coords: &str) -> (usize, usize) {
    let (x, y) = coords.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

impl Display for Paper {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let x_max = *self.dots.iter().map(|(x, _)| x).max().unwrap();
        let y_max = *self.dots.iter().map(|(_, y)| y).max().unwrap();

        for y in 0..=y_max {
            for x in 0..=x_max {
                if self.dots.contains(&(x, y)) {
                    write!(formatter, "█")?;
                } else {
                    write!(formatter, " ")?;
                }
            }

            writeln!(formatter)?;
        }

        Ok(())
    }
}
