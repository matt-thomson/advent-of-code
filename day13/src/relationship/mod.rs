mod parse;

#[derive(Debug)]
pub struct Relationship {
    from: String,
    to: String,
    happiness: i32,
}

#[derive(Debug)]
enum Direction {
    Gain,
    Lose
}

impl Relationship {
    pub fn parse(input: &str) -> Relationship {
        parse::parse(input)
    }

    fn new(from: &str, to: &str, direction: Direction, happiness: u32) -> Relationship {
        let net_happiness = match direction {
            Direction::Gain => happiness as i32,
            Direction::Lose => -(happiness as i32)
        };

        Relationship {
            from: from.to_string(),
            to: to.to_string(),
            happiness: net_happiness
        }
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn to(&self) -> String {
        self.to.clone()
    }

    pub fn happiness(&self) -> i32 {
        self.happiness
    }
}

#[cfg(test)]
mod tests {
    use super::Relationship;

    #[test]
    fn test_parse_relationship() {
        let relationship = Relationship::parse("Alice would gain 54 happiness units by sitting next to Bob.");

        assert_eq!(relationship.from(), "Alice");
        assert_eq!(relationship.to(), "Bob");
        assert_eq!(relationship.happiness(), 54);
    }
}
