use crate::direction::Direction;

pub struct Rope<const N: usize> {
    head: (i64, i64),
    tail: [(i64, i64); N],
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        Self {
            head: (0, 0),
            tail: [(0, 0); N],
        }
    }

    pub fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }

        let dx = self.head.0.abs_diff(self.tail[0].0);
        let dy = self.head.1.abs_diff(self.tail[0].1);

        if dx > 1 || dy > 1 {
            if self.head.0 > self.tail[0].0 {
                self.tail[0].0 += 1;
            } else if self.head.0 < self.tail[0].0 {
                self.tail[0].0 -= 1;
            }

            if self.head.1 > self.tail[0].1 {
                self.tail[0].1 += 1;
            } else if self.head.1 < self.tail[0].1 {
                self.tail[0].1 -= 1;
            }
        }
    }

    pub fn tails(&self) -> [(i64, i64); N] {
        self.tail.clone()
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
        let mut rope = Rope { head, tail: [tail] };
        rope.step(&direction);

        assert_eq!(rope.head, expected_head);
        assert_eq!(rope.tail, [expected_tail]);
    }
}
