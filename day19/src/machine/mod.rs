mod parse;

use rand;
use rand::Rng;

use std::collections::BTreeMap;

pub struct Machine {
    substitutions: BTreeMap<String, Vec<Vec<String>>>,
    medicine: Vec<String>
}

impl Machine {
    pub fn parse(input: &str) -> Machine {
        parse::parse(input)
    }

    pub fn medicine(&self) -> &Vec<String> {
        &self.medicine
    }

    pub fn substitutions(&self, element: &str) -> Option<&Vec<Vec<String>>> {
        self.substitutions.get(element)
    }

    pub fn shuffled_substitutions(&self) -> Vec<(&String, &Vec<String>)> {
        let mut substitutions: Vec<(&String, &Vec<String>)> = self.substitutions.iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to)))
            .collect();

        rand::thread_rng().shuffle(&mut substitutions[..]);
        substitutions
    }

    fn new(substitutions: Vec<(String, Vec<String>)>, medicine: Vec<String>) -> Machine {
        let mut sub_map = BTreeMap::new();

        for (input, output) in substitutions {
            sub_map.entry(input).or_insert(vec![]).push(output);
        }

        Machine { substitutions: sub_map, medicine: medicine }
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    #[test]
    fn test_parse_machine() {
        let machine = Machine::parse("H => He\nHe => O\nH => NNa\nHHeO");
        assert_eq!(*machine.medicine(), vec!["H", "He", "O"]);

        let h_substitutions = machine.substitutions.get("H").unwrap();
        assert_eq!(h_substitutions.len(), 2);
        assert_eq!(h_substitutions[0], vec!["He"]);
        assert_eq!(h_substitutions[1], vec!["N", "Na"]);
    }
}
