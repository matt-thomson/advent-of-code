#[derive(Debug, PartialEq)]
pub enum Step {
    DealIntoNewStack,
    CutCards(isize),
    DealWithIncrement(usize),
}

impl Step {
    pub fn parse(input: &str) -> Self {
        match input {
            "deal into new stack" => Self::DealIntoNewStack,
            x if x.starts_with("cut ") => Self::CutCards(x[4..].parse().unwrap()),
            x if x.starts_with("deal with increment ") => {
                Self::DealWithIncrement(x[20..].parse().unwrap())
            }
            _ => panic!("unknown step {}", input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Step::parse("deal into new stack"), Step::DealIntoNewStack);

        assert_eq!(Step::parse("cut 123"), Step::CutCards(123));
        assert_eq!(Step::parse("cut -123"), Step::CutCards(-123));

        assert_eq!(
            Step::parse("deal with increment 123"),
            Step::DealWithIncrement(123)
        );
    }
}
