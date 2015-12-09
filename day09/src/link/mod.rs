mod parse;

#[derive(Debug)]
pub struct Link {
    from: String,
    to: String,
    distance: u32
}

impl Link {
    fn new(from: &str, to: &str, distance: u32) -> Link {
        Link { from: from.to_string(), to: to.to_string(), distance: distance }
    }

    pub fn parse(input: &str) -> Link {
        parse::parse(input)
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn to(&self) -> String {
        self.to.clone()
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }
}

#[cfg(test)]
mod tests {
    use super::Link;

    #[test]
    fn test_parse_link() {
        let link = Link::parse("London to Dublin = 464");

        assert_eq!(link.from(), "London");
        assert_eq!(link.to(), "Dublin");
        assert_eq!(link.distance(), 464);
    }
}
