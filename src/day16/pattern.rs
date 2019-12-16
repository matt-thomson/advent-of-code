const BASE_PATTERN_LENGTH: usize = 4;
const BASE_PATTERN: [i32; BASE_PATTERN_LENGTH] = [0, 1, 0, -1];

#[derive(Debug)]
pub struct Pattern {
    index: usize,
    emitted: usize,
    position: usize,
}

impl Pattern {
    pub fn new(position: usize) -> Self {
        Pattern {
            index: 0,
            emitted: 0,
            position: position,
        }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.emitted += 1;

        if self.emitted > self.position {
            self.index += 1;
            self.index %= BASE_PATTERN_LENGTH;
            self.emitted = 0;
        }

        Some(BASE_PATTERN[self.index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_0() {
        let pattern = Pattern::new(0);
        let output: Vec<_> = pattern.take(20).collect();

        let expected = vec![
            1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0,
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_pattern_2() {
        let pattern = Pattern::new(2);
        let output: Vec<_> = pattern.take(20).collect();

        let expected = vec![
            0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
        ];
        assert_eq!(output, expected);
    }
}
