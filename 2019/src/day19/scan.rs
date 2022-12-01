use super::Position;

pub struct Scan {
    distance: usize,
    x: usize,
}

impl Scan {
    pub fn new() -> Self {
        Self { distance: 0, x: 0 }
    }
}

impl Iterator for Scan {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let result = Some((self.x, self.distance - self.x));

        if self.x == self.distance {
            self.distance += 1;
            self.x = 0;
        } else {
            self.x += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let scan = Scan::new();
        let result: Vec<_> = scan.take(10).collect();

        let expected = vec![
            (0, 0),
            (0, 1),
            (1, 0),
            (0, 2),
            (1, 1),
            (2, 0),
            (0, 3),
            (1, 2),
            (2, 1),
            (3, 0),
        ];

        assert_eq!(result, expected);
    }
}
