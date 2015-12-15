use super::Reindeer;

use nom::{alpha, digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Reindeer {
    let result = reindeer(line.as_bytes());

    match result {
        IResult::Done(_, reindeer) => reindeer,
        _                          => panic!("Invalid result {:?}", result)
    }
}

named!(name<String>, map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));
named!(number<u32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(reindeer<Reindeer>, chain!(
    name:      name                            ~
    space                                      ~
    tag!("can fly")                            ~
    space                                      ~
    speed:     number                          ~
    space                                      ~
    tag!("km/s for")                           ~
    space                                      ~
    fly_time:  number                          ~
    space                                      ~
    tag!("seconds, but then must rest for")    ~
    space                                      ~
    rest_time: number                          ~
    space                                      ~
    tag!("seconds."), || Reindeer::new(&name, speed, fly_time, rest_time)
));
