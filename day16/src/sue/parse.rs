use super::Sue;

use nom::{alpha, digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Sue {
    let result = sue(line.as_bytes());

    match result {
        IResult::Done(_, sue) => sue,
        _                     => panic!("Invalid result {:?}", result)
    }
}

named!(name<String>, map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));
named!(number<u32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(property<(String, u32)>, chain!(
    property: name   ~
    tag!(":")        ~
    space            ~
    value: number    ~
    many0!(tag!(", ")), || (property, value)
));

named!(sue<Sue>, chain!(
    tag!("Sue")    ~
    space          ~
    number: number ~
    tag!(":")      ~
    space          ~
    properties: many0!(property), || {
        let mut sue = Sue::new(number);
        for (property, value) in properties {
            sue.add(&property, value);
        }
        sue
    }
));
