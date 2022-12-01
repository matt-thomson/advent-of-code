#[derive(Debug)]
pub struct Chemical {
    amount: u64,
    name: String,
}

impl Chemical {
    pub fn parse(input: &str) -> Self {
        let space = input.find(' ').unwrap();
        let amount = input[0..space].parse().unwrap();
        let name = input[(space + 1)..].to_string();

        Chemical { amount, name }
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let chemical = Chemical::parse("12 ABC");

        assert_eq!(chemical.amount(), 12);
        assert_eq!(chemical.name(), "ABC");
    }
}
