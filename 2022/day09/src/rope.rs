use crate::direction::Direction;

#[derive(Default)]
pub struct Rope {
    head: (u64, u64),
    tail: (u64, u64),
}

impl Rope {
    pub fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::direction::Direction;

    use super::Rope;

    #[rstest]
    #[case((0, 0), (0, 0), "R", (1, 0), (0, 0))]
    fn test_step(
        #[case] head: (u64, u64),
        #[case] tail: (u64, u64),
        #[case] direction: Direction,
        #[case] expected_head: (u64, u64),
        #[case] expected_tail: (u64, u64),
    ) {
        let mut rope = Rope { head, tail };
        rope.step(&direction);

        assert_eq!(rope.head, expected_head);
        assert_eq!(rope.tail, expected_tail);
    }
}
