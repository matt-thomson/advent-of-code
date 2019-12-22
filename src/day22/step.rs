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

    pub fn apply(&self, deck: &[usize]) -> Vec<usize> {
        match self {
            Self::DealIntoNewStack => {
                let mut result = deck.to_vec();
                result.reverse();
                result
            }
            Self::CutCards(number) => {
                let offset = if *number > 0 {
                    *number as usize
                } else {
                    deck.len() - (number.abs() as usize)
                };

                let mut result = vec![0; deck.len()];

                result[..(deck.len() - offset)].copy_from_slice(&deck[offset..]);
                result[(deck.len() - offset)..].copy_from_slice(&deck[..offset]);

                result
            }
            Self::DealWithIncrement(increment) => {
                let mut result = vec![0; deck.len()];

                for i in 0..deck.len() {
                    let position = (i * *increment) % deck.len();
                    result[position] = deck[i];
                }

                result
            }
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

    #[test]
    fn test_apply_deal_into_new_stack() {
        let step = Step::DealIntoNewStack;
        let deck = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let result = step.apply(&deck);

        assert_eq!(result, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_apply_cut_positive() {
        let step = Step::CutCards(3);
        let deck = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let result = step.apply(&deck);

        assert_eq!(result, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_apply_cut_negative() {
        let step = Step::CutCards(-4);
        let deck = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let result = step.apply(&deck);

        assert_eq!(result, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_apply_deal_with_increment() {
        let step = Step::DealWithIncrement(3);
        let deck = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let result = step.apply(&deck);

        assert_eq!(result, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
