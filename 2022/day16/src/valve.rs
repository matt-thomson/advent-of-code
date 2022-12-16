use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub struct Valve {
    name: u64,
    flow_rate: u64,
    tunnels: Vec<u64>,
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

fn parse_name(input: &str) -> IResult<&str, u64> {
    map(pair(anychar, anychar), |(first, second)| {
        (first as u64 - 'A' as u64) * 26 + (second as u64 - 'A' as u64)
    })(input)
}

impl Valve {
    pub fn name(&self) -> u64 {
        self.name
    }

    pub fn flow_rate(&self) -> u64 {
        self.flow_rate
    }

    pub fn tunnels(&self) -> &[u64] {
        &self.tunnels
    }
}

#[cfg(test)]
mod tests {
    use super::Valve;

    #[test]
    fn test_parse() {
        let input = "Valve BB has flow rate=13; tunnels lead to valves CC, AA";
        let valve: Valve = input.parse().unwrap();

        assert_eq!(valve.name, 27);
        assert_eq!(valve.flow_rate, 13);
        assert_eq!(valve.tunnels, [54, 0]);
    }
}
