#[derive(Debug)]
pub struct Lexer {
    chars: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars = input.chars().collect();

        Self { chars, position: 0 }
    }

    pub fn peek(&self) -> char {
        self.chars[self.position]
    }

    pub fn expect(&mut self, expected: char) {
        if self.chars[self.position] != expected {
            panic!("expected {}, got {}", expected, self.chars[self.position]);
        }

        self.position += 1;
    }

    pub fn next(&mut self) -> char {
        let result = self.chars[self.position];
        self.position += 1;

        result
    }
}
