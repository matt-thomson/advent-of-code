use super::{Action, Gate, Input};

use nom::{alpha, digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Gate {
    let result = gate(line.as_bytes());

    match result {
        IResult::Done(_, gate) => gate,
        _                      => panic!("Invalid line {} (result {:?})", line, result)
    }
}

named!(number<u16>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
named!(wire<String>, map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));

named!(number_input<Input>, map!(number, Input::Number));
named!(wire_input<Input>, map!(wire, Input::Wire));
named!(input<Input>, alt!(number_input | wire_input));

named!(literal<Action>, map!(input, |x| Action::Literal(x)));
named!(and<Action>,
       chain!(input1: input ~ space ~ tag!("AND") ~ space ~ input2: input, || Action::And(input1, input2)
));
named!(or<Action>,
       chain!(input1: input ~ space ~ tag!("OR") ~ space ~ input2: input, || Action::Or(input1, input2)
));
named!(not<Action>,
       chain!(tag!("NOT") ~ space ~ input: input, || Action::Not(input)
));
named!(lshift<Action>,
       chain!(input: input ~ space ~ tag!("LSHIFT") ~ space ~ number: number, || Action::LShift(input, number)
));
named!(rshift<Action>,
       chain!(input: input ~ space ~ tag!("RSHIFT") ~ space ~ number: number, || Action::RShift(input, number)
));
named!(action<Action>, alt!(and | or | not | complete!(lshift) | complete!(rshift) | literal));

named!(gate<Gate>, chain!(
    action:    action   ~
    space               ~
    tag!("->")          ~
    space               ~
    output:    wire, || Gate::new(action, output)
));
