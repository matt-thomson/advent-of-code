use super::Link;

use nom::{alpha, digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Link {
    let result = link(line.as_bytes());

    match result {
        IResult::Done(_, link) => link,
        _                      => panic!("Invalid result {:?}", result)
    }
}

named!(place<String>, map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));
named!(distance<u32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(link<Link>, chain!(
    from:     place ~
    space           ~
    tag!("to")      ~
    space           ~
    to:       place ~
    space           ~
    tag!("=")       ~
    space           ~
    distance: distance, || Link::new(&from, &to, distance)
));

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse_link() {
        let link = parse("London to Dublin = 464");

        assert_eq!(link.from(), "London");
        assert_eq!(link.to(), "Dublin");
        assert_eq!(link.distance(), 464);
    }
}
