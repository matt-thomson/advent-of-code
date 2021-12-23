use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnailfishNumber {
    entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
        let mut in_number = false;

        for c in input.chars() {
            match c {
                '[' => depth += 1,
                ']' | ',' => {
                    if in_number {
                        entries.push(Entry { depth, value });
                        value = 0;
                        in_number = false;
                    }

                    if c == ']' {
                        depth -= 1;
                    }
                }
                '0'..='9' => {
                    in_number = true;
                    value = value * 10 + c.to_digit(10).unwrap();
                }
                _ => panic!("unexpected character {}", c),
            }
        }

        Ok(Self { entries })
    }
}

impl SnailfishNumber {
    pub fn add(&self, other: &Self) -> Self {
        let mut entries = self.entries.clone();
        entries.extend(other.entries.clone());

        entries.iter_mut().for_each(|entry| entry.depth += 1);

        let mut result = Self { entries };
        result.reduce();

        result
    }

    pub fn magnitude(&self) -> u32 {
        let mut entries = self.entries.clone();

        while entries.len() > 1 {
            for i in 0..(entries.len() - 1) {
                if entries[i].depth == entries[i + 1].depth {
                    entries[i].value = entries[i].value * 3 + entries[i + 1].value * 2;
                    entries[i].depth -= 1;

                    entries.remove(i + 1);

                    break;
                }
            }
        }

        entries[0].value
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        for i in 0..(self.entries.len() - 1) {
            if self.entries[i].depth == 5 {
                if i > 0 {
                    self.entries[i - 1].value += self.entries[i].value;
                }

                if i < self.entries.len() - 2 {
                    self.entries[i + 2].value += self.entries[i + 1].value;
                }

                self.entries[i].value = 0;
                self.entries[i].depth = 4;

                self.entries.remove(i + 1);

                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for i in 0..(self.entries.len()) {
            if self.entries[i].value >= 10 {
                let left = self.entries[i].value / 2;
                let right = left + if self.entries[i].value % 2 == 0 { 0 } else { 1 };

                self.entries[i].value = left;
                self.entries[i].depth += 1;

                self.entries.insert(
                    i + 1,
                    Entry {
                        value: right,
                        depth: self.entries[i].depth,
                    },
                );

                return true;
            }
        }

        false
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
    #[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    fn should_explode(#[case] mut input: SnailfishNumber, #[case] expected: SnailfishNumber) {
        assert!(input.explode());
        assert_eq!(input, expected);
    }

    #[rstest]
    #[case(
        "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
    )]
    #[case(
        "[[[[0,7],4],[14,[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,7],[0,13]]],[1,1]]"
    )]
    #[case(
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
    )]
    fn should_split(#[case] mut input: SnailfishNumber, #[case] expected: SnailfishNumber) {
        assert!(input.split());
        assert_eq!(input, expected);
    }

    #[rstest]
    #[case(
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )]
    fn should_reduce(#[case] mut input: SnailfishNumber, #[case] expected: SnailfishNumber) {
        input.reduce();
        assert_eq!(input, expected);
    }

    #[rstest]
    #[case(
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )]
    fn should_add(
        #[case] first: SnailfishNumber,
        #[case] second: SnailfishNumber,
        #[case] expected: SnailfishNumber,
    ) {
        let result = first.add(&second);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    fn should_calculate_magnitude(#[case] input: SnailfishNumber, #[case] expected: u32) {
        assert_eq!(input.magnitude(), expected);
    }
}
