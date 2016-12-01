mod parse;

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Sue {
    number: u32,
    properties: BTreeMap<String, u32>
}

impl Sue {
    fn new(number: u32) -> Sue {
        Sue { number: number, properties: BTreeMap::new() }
    }

    fn add(&mut self, property: &str, value: u32) {
        self.properties.insert(property.to_string(), value);
    }

    pub fn parse(input: &str) -> Sue {
        parse::parse(input)
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn matches(&self, ticker: &BTreeMap<String, Box<Fn(u32) -> bool>>) -> bool {
        self.properties.iter().all(|(property, value)| ticker.get(property).unwrap()(*value))
    }
}

#[cfg(test)]
mod tests {
    use super::Sue;
    use std::collections::BTreeMap;

    #[test]
    fn test_parse_sue() {
        let sue = Sue::parse("Sue 1: goldfish: 6, trees: 9, akitas: 0");
        assert_eq!(sue.number(), 1);
        assert_eq!(*sue.properties.get("goldfish").unwrap(), 6);
        assert_eq!(*sue.properties.get("trees").unwrap(), 9);
        assert_eq!(*sue.properties.get("akitas").unwrap(), 0);
    }

    #[test]
    fn test_matches_true() {
        let mut ticker: BTreeMap<String, Box<Fn(u32) -> bool>> = BTreeMap::new();
        ticker.insert("goldfish".to_string(), Box::new(|x| x == 6));
        ticker.insert("trees".to_string(), Box::new(|x| x == 9));
        ticker.insert("cars".to_string(), Box::new(|x| x == 1));
        ticker.insert("akitas".to_string(), Box::new(|x| x == 0));

        let sue = Sue::parse("Sue 1: goldfish: 6, trees: 9, akitas: 0");;
        assert!(sue.matches(&ticker));
    }

    #[test]
    fn test_matches_false() {
        let mut ticker: BTreeMap<String, Box<Fn(u32) -> bool>> = BTreeMap::new();
        ticker.insert("goldfish".to_string(), Box::new(|x| x == 6));
        ticker.insert("trees".to_string(), Box::new(|x| x == 9));
        ticker.insert("cars".to_string(), Box::new(|x| x == 1));
        ticker.insert("akitas".to_string(), Box::new(|x| x == 1));

        let sue = Sue::parse("Sue 1: goldfish: 6, trees: 9, akitas: 0");
        assert!(!sue.matches(&ticker));
    }
}
