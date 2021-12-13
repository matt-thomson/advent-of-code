use std::collections::BTreeSet;

use crate::fold::Fold;

#[derive(Debug)]
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
