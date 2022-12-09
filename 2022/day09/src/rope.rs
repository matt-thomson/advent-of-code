use crate::direction::Direction;

pub struct Rope<const N: usize> {
    knots: [(i64, i64); N],
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self { knots: [(0, 0); N] }
    }
}

impl<const N: usize> Rope<N> {
    pub fn step(&mut self, direction: &Direction) {
        let mut head = self.knots.get_mut(0).unwrap();

        match direction {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
        }

        for i in 0..(self.knots.len() - 1) {
            let first = self.knots[i];
            let mut second = self.knots.get_mut(i + 1).unwrap();

            let dx = first.0.abs_diff(second.0);
            let dy = first.1.abs_diff(second.1);

            if dx > 1 || dy > 1 {
                if first.0 > second.0 {
                    second.0 += 1;
                } else if first.0 < second.0 {
                    second.0 -= 1;
                }

                if first.1 > second.1 {
                    second.1 += 1;
                } else if first.1 < second.1 {
                    second.1 -= 1;
                }
            }
        }
    }

    pub fn tail(&self) -> (i64, i64) {
        self.knots.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::direction::Direction;

    use super::Rope;

    #[rstest]
    #[case((0, 0), (0, 0), "R", (1, 0), (0, 0))]
    #[case((1, 0), (0, 0), "R", (2, 0), (1, 0))]
    #[case((2, 0), (1, 0), "R", (3, 0), (2, 0))]
    #[case((3, 0), (2, 0), "R", (4, 0), (3, 0))]
    #[case((4, 0), (3, 0), "U", (4, 1), (3, 0))]
    #[case((4, 1), (3, 0), "U", (4, 2), (4, 1))]
    #[case((4, 2), (4, 1), "U", (4, 3), (4, 2))]
    #[case((4, 3), (4, 2), "U", (4, 4), (4, 3))]
    #[case((4, 4), (4, 3), "L", (3, 4), (4, 3))]
    #[case((3, 4), (4, 3), "L", (2, 4), (3, 4))]
    #[case((2, 4), (3, 4), "L", (1, 4), (2, 4))]
    #[case((1, 4), (2, 4), "D", (1, 3), (2, 4))]
    #[case((1, 3), (2, 4), "R", (2, 3), (2, 4))]
    #[case((2, 3), (2, 4), "R", (3, 3), (2, 4))]
    #[case((3, 3), (2, 4), "R", (4, 3), (3, 3))]
    #[case((4, 3), (3, 3), "R", (5, 3), (4, 3))]
    #[case((5, 3), (4, 3), "D", (5, 2), (4, 3))]
    #[case((5, 2), (4, 3), "L", (4, 2), (4, 3))]
    #[case((4, 2), (4, 3), "L", (3, 2), (4, 3))]
    #[case((3, 2), (4, 3), "L", (2, 2), (3, 2))]
    #[case((2, 2), (3, 3), "L", (1, 2), (2, 2))]
    #[case((1, 2), (2, 3), "L", (0, 2), (1, 2))]
    #[case((0, 2), (1, 2), "R", (1, 2), (1, 2))]
    #[case((1, 2), (1, 2), "R", (2, 2), (1, 2))]
    fn test_step(
        #[case] head: (i64, i64),
        #[case] tail: (i64, i64),
        #[case] direction: Direction,
        #[case] expected_head: (i64, i64),
        #[case] expected_tail: (i64, i64),
    ) {
        let mut rope = Rope {
            knots: [head, tail],
        };

        rope.step(&direction);

        assert_eq!(rope.knots, [expected_head, expected_tail]);
    }
}
