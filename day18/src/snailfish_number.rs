use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct SnailfishNumber {
    entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    depth: usize,
    value: u32,
}

impl FromStr for SnailfishNumber {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut entries = vec![];
        let mut depth = 0;
        let mut value = 0;

        for c in input.chars() {
            match c {
                '[' => depth += 1,
                ']' => {
                    if value != 0 {
                        entries.push(Entry { depth, value });
                        value = 0;
                    }

                    depth -= 1;
                }
                ',' => {
                    if value != 0 {
                        entries.push(Entry { depth, value });
                        value = 0;
                    }
                }
                '0'..='9' => {
                    value = value * 10 + c.to_digit(10).unwrap();
                }
                _ => panic!("unexpected character {}", c),
            }
        }

        Ok(Self { entries })
    }
}

impl SnailfishNumber {
    fn step_reduce(&self) -> Option<Self> {
        unimplemented!();
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

    #[rstest]
    #[case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
    fn should_explode(#[case] input: SnailfishNumber, #[case] expected: SnailfishNumber) {
        let exploded: SnailfishNumber = input.step_reduce().unwrap();
        assert_eq!(exploded, expected);
    }
}
