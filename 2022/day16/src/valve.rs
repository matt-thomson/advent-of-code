use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, valve) = map(
            tuple((
                preceded(tag("Valve "), parse_name),
                preceded(tag(" has flow rate="), nom::character::complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), parse_name),
                ),
            )),
            |(name, flow_rate, tunnels)| Valve {
                name,
                flow_rate,
                tunnels,
            },
        )(input)
        .finish()
        .map_err(|s| eyre!(s.to_string()))?;

        Ok(valve)
    }
}

fn parse_name(input: &str) -> IResult<&str, String> {
    map(take(2usize), |name: &str| name.to_string())(input)
}

#[cfg(test)]
mod tests {
    use super::Valve;

    #[test]
    fn test_parse() {
        let input = "Valve BB has flow rate=13; tunnels lead to valves CC, AA";
        let valve: Valve = input.parse().unwrap();

        assert_eq!(valve.name, "BB");
        assert_eq!(valve.flow_rate, 13);
        assert_eq!(valve.tunnels, ["CC", "AA"]);
    }
}
