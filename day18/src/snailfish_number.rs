use std::convert::Infallible;
use std::str::FromStr;

use crate::lexer::Lexer;

#[derive(Debug)]
pub enum SnailfishNumber {
    Number(u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl FromStr for SnailfishNumber {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer::new(input);
        Ok(read_number(&mut lexer))
    }
}

fn read_number(lexer: &mut Lexer) -> SnailfishNumber {
    match lexer.peek() {
        '[' => {
            lexer.expect('[');
            let left = read_number(lexer);

            lexer.expect(',');
            let right = read_number(lexer);

            lexer.expect(']');

            SnailfishNumber::Pair(Box::new(left), Box::new(right))
        }
        '0'..='9' => {
            let mut result = 0;
            while lexer.peek().is_numeric() {
                result = result * 10 + (lexer.next().to_digit(10).unwrap());
            }

            SnailfishNumber::Number(result)
        }
        _ => panic!("expected [ or digit, got {}", lexer.peek()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("[1,2]")]
    #[case("[[1,2],3]")]
    #[case("[9,[8,7]]")]
    #[case("[[1,9],[8,5]]")]
    #[case("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]")]
    #[case("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")]
    #[case("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")]
    fn should_parse_snailfish_number(#[case] input: &str) {
        let snailfish_number: Result<SnailfishNumber, _> = input.parse();
        assert!(snailfish_number.is_ok());
    }
}
