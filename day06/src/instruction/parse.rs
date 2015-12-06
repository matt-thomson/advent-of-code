use super::{Action, Instruction};

use nom::{digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Instruction {
    let result = instruction(line.as_bytes());

    match result {
        IResult::Done(_, instruction) => instruction,
        _                             => panic!("Invalid result {:?}", result)
    }
}

named!(toggle<Action>, map!(tag!("toggle"), |_| Action::Toggle));
named!(turn_on<Action>, map!(tag!("turn on"), |_| Action::TurnOn));
named!(turn_off<Action>, map!(tag!("turn off"), |_| Action::TurnOff));
named!(action<Action>, alt!(toggle | turn_on | turn_off));

named!(coordinate<usize>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
named!(position<(usize, usize)>, chain!(x: coordinate ~ tag!(",") ~ y: coordinate, || (x, y)));

named!(instruction<Instruction>, chain!(
    action:       action   ~
    space                  ~
    top_left:     position ~
    space                  ~
    tag!("through")        ~
    space                  ~
    bottom_right: position, || {
        let (left, top) = top_left;
        let (right, bottom) = bottom_right;
        Instruction::new(action, top, left, bottom, right)
    }
));

#[cfg(test)]
mod tests {
    use super::{action, position};

    use instruction::Action;
    use nom::IResult::Done;

    #[test]
    fn test_parse_turn_on() {
        let result = action(b"turn on");

        match result {
            Done(_, action) => assert_eq!(action, Action::TurnOn),
            _               => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_turn_off() {
        let result = action(b"turn off");

        match result {
            Done(_, action) => assert_eq!(action, Action::TurnOff),
            _               => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_toggle() {
        let result = action(b"toggle");

        match result {
            Done(_, action) => assert_eq!(action, Action::Toggle),
            _               => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_position() {
        let result = position(b"12,34");

        match result {
            Done(_, position) => assert_eq!(position, (12, 34)),
            _               => panic!("Incorrect result {:?}", result)
        };
    }
}
