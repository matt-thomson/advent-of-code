use std::collections::{BTreeSet, HashMap};

use relationship::Relationship;

#[derive(Debug)]
pub struct Family {
    people: Vec<String>,
    happiness: HashMap<(String, String), i32>
}

impl Family {
    pub fn new<I: Iterator<Item = Relationship>>(relationships: I) -> Family {
        let mut people = BTreeSet::new();
        let mut happiness = HashMap::new();

        for relationship in relationships {
            people.insert(relationship.from());
            people.insert(relationship.to());

            happiness.insert((relationship.from(), relationship.to()), relationship.happiness());
        }

        Family { people: people.iter().map(|x| x.to_string()).collect(), happiness: happiness }
    }

    pub fn people(&self) -> &Vec<String> {
        &self.people
    }

    pub fn happiness(&self, from: &str, to: &str) -> i32 {
        if from == "yourself" || to == "yourself" {
            return 0;
        }

        let from_to = *self.happiness.get(&(from.to_string(), to.to_string())).unwrap();
        let to_from = *self.happiness.get(&(to.to_string(), from.to_string())).unwrap();

        from_to + to_from
    }

    pub fn add_yourself(&mut self) {
        self.people.push("yourself".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::Family;
    use relationship::Relationship;

    #[test]
    fn should_build_family() {
        let relationships = vec![
            Relationship::parse("Alice would gain 54 happiness units by sitting next to Bob."),
            Relationship::parse("Alice would lose 79 happiness units by sitting next to Carol."),
            Relationship::parse("Bob would gain 83 happiness units by sitting next to Alice."),
            Relationship::parse("Bob would lose 7 happiness units by sitting next to Carol."),
            Relationship::parse("Carol would lose 62 happiness units by sitting next to Alice."),
            Relationship::parse("Carol would gain 60 happiness units by sitting next to Bob.")
        ];

        let family = Family::new(relationships.into_iter());

        assert_eq!(*family.people(), vec!["Alice", "Bob", "Carol"]);
        assert_eq!(family.happiness("Bob", "Carol"), 53);
        assert_eq!(family.happiness("Carol", "Bob"), 53);
    }

    #[test]
    fn should_add_yourself() {
        let relationships = vec![
            Relationship::parse("Alice would gain 54 happiness units by sitting next to Bob."),
            Relationship::parse("Alice would lose 79 happiness units by sitting next to Carol."),
            Relationship::parse("Bob would gain 83 happiness units by sitting next to Alice."),
            Relationship::parse("Bob would lose 7 happiness units by sitting next to Carol."),
            Relationship::parse("Carol would lose 62 happiness units by sitting next to Alice."),
            Relationship::parse("Carol would gain 60 happiness units by sitting next to Bob.")
        ];

        let mut family = Family::new(relationships.into_iter());
        family.add_yourself();

        assert_eq!(*family.people(), vec!["Alice", "Bob", "Carol", "yourself"]);
        assert_eq!(family.happiness("Bob", "Carol"), 53);
        assert_eq!(family.happiness("Carol", "Bob"), 53);

        assert_eq!(family.happiness("yourself", "Carol"), 0);
        assert_eq!(family.happiness("Carol", "yourself"), 0);
    }
}
