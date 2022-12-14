use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Segment {
    from: (usize, usize),
    to: (usize, usize),
}

impl Segment {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self { from, to }
    }

    pub fn points(&self) -> Result<Vec<(usize, usize)>> {
        let (from_x, from_y) = self.from;
        let (to_x, to_y) = self.to;

        if from_x == to_x {
            Ok((from_y.min(to_y)..=from_y.max(to_y))
                .map(move |y| (from_x, y))
                .collect())
        } else if from_y == to_y {
            Ok((from_x.min(to_x)..=from_x.max(to_x))
                .map(move |x| (x, from_y))
                .collect())
        } else {
            Err(eyre!("line segment must be horizontal or vertical [({from_x},{from_y}) -> ({to_x},{to_y})]"))
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Segment;

    #[rstest]
    #[case((8, 4), (8, 6), vec![(8, 4), (8, 5), (8, 6)])]
    #[case((8, 6), (8, 4), vec![(8, 4), (8, 5), (8, 6)])]
    #[case((6, 6), (8, 6), vec![(6, 6), (7, 6), (8, 6)])]
    #[case((8, 6), (6, 6), vec![(6, 6), (7, 6), (8, 6)])]
    fn test_points(
        #[case] from: (usize, usize),
        #[case] to: (usize, usize),
        #[case] expected: Vec<(usize, usize)>,
    ) {
        let segment = Segment { from, to };
        assert_eq!(segment.points().unwrap(), expected);
    }
}
