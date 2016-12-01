use super::{Direction, Relationship};

use nom::{alpha, digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Relationship {
    let result = relationship(line.as_bytes());

    match result {
        IResult::Done(_, relationship) => relationship,
        _                              => panic!("Invalid result {:?}", result)
    }
}

named!(person<String>, map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));
named!(happiness<u32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(gain<Direction>, map!(tag!("gain"), |_| Direction::Gain));
named!(lose<Direction>, map!(tag!("lose"), |_| Direction::Lose));
named!(direction<Direction>, alt!(gain | lose));

named!(relationship<Relationship>, chain!(
    from:      person                          ~
    space                                      ~
    tag!("would")                              ~
    space                                      ~
    direction: direction                       ~
    space                                      ~
    happiness: happiness                       ~
    space                                      ~
    tag!("happiness units by sitting next to") ~
    space                                      ~
    to:        person                          ~
    tag!("."), || Relationship::new(&from, &to, direction, happiness)
));

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse_gain_relationship() {
        let relationship = parse("Alice would gain 54 happiness units by sitting next to Bob.");

        assert_eq!(relationship.from(), "Alice");
        assert_eq!(relationship.to(), "Bob");
        assert_eq!(relationship.happiness(), 54);
    }

    #[test]
    fn test_parse_lose_relationship() {
        let relationship = parse("Alice would lose 54 happiness units by sitting next to Bob.");

        assert_eq!(relationship.from(), "Alice");
        assert_eq!(relationship.to(), "Bob");
        assert_eq!(relationship.happiness(), -54);
    }
}
