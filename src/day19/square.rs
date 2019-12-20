use super::Position;

pub struct Square {
    x_start: usize,
    y_start: usize,
    x_offset: usize,
    y_offset: usize,
    size: usize,
}

impl Square {
    pub fn new(start: &Position, size: usize) -> Self {
        let (x_start, y_start) = *start;

        Self {
            x_start,
            y_start,
            x_offset: 0,
            y_offset: 0,
            size,
        }
    }
}

impl Iterator for Square {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.x_offset == self.size {
            self.x_offset = 0;
            self.y_offset += 1;
        }

        if self.y_offset == self.size {
            return None;
        }

        let result = Some((self.x_start + self.x_offset, self.y_start + self.y_offset));
        self.x_offset += 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        let square = Square::new(&(3, 4), 3);
        let result: Vec<_> = square.collect();

        let expected = vec![
            (3, 4),
            (4, 4),
            (5, 4),
            (3, 5),
            (4, 5),
            (5, 5),
            (3, 6),
            (4, 6),
            (5, 6),
        ];

        assert_eq!(result, expected);
    }
}
