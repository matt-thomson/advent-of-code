use super::chemical::Chemical;

#[derive(Debug)]
pub struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    pub fn parse(input: &str) -> Self {
        let equals = input.find('=').unwrap();

        let inputs = input[0..(equals - 1)]
            .split(", ")
            .map(|x| Chemical::parse(x))
            .collect();

        let output = Chemical::parse(&input[(equals + 3)..]);

        Reaction { inputs, output }
    }

    pub fn inputs(&self) -> &[Chemical] {
        &self.inputs
    }

    pub fn output(&self) -> &Chemical {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let reaction = Reaction::parse("2 AB, 3 BC, 4 CA => 1 FUEL");

        assert_eq!(reaction.inputs.len(), 3);

        assert_eq!(reaction.inputs()[0].amount(), 2);
        assert_eq!(reaction.inputs()[0].name(), "AB");

        assert_eq!(reaction.inputs()[1].amount(), 3);
        assert_eq!(reaction.inputs()[1].name(), "BC");

        assert_eq!(reaction.inputs()[2].amount(), 4);
        assert_eq!(reaction.inputs()[2].name(), "CA");

        assert_eq!(reaction.output().amount(), 1);
        assert_eq!(reaction.output().name(), "FUEL");
    }
}
