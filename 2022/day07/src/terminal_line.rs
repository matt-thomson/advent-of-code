use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, rest},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TerminalLine {
    List,
    ChangeRoot,
    ChangeOut,
    ChangeIn { name: String },
    File { name: String, size: u64 },
    Directory { name: String },
}

impl FromStr for TerminalLine {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let line = Self::parse_line(input);
        let (_, line) = line.finish().map_err(|s| eyre!(s.to_string()))?;

        Ok(line)
    }
}

impl TerminalLine {
    fn parse_line(input: &str) -> IResult<&str, Self> {
        all_consuming(alt((
            Self::parse_list,
            Self::parse_change_root,
            Self::parse_change_out,
            Self::parse_change_in,
            Self::parse_directory,
            Self::parse_file,
        )))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Self> {
        map(tag("$ ls"), |_| TerminalLine::List)(input)
    }

    fn parse_change_root(input: &str) -> IResult<&str, Self> {
        map(tag("$ cd /"), |_| TerminalLine::ChangeRoot)(input)
    }

    fn parse_change_out(input: &str) -> IResult<&str, Self> {
        map(tag("$ cd .."), |_| TerminalLine::ChangeOut)(input)
    }

    fn parse_change_in(input: &str) -> IResult<&str, Self> {
        map(preceded(tag("$ cd "), rest), |name: &str| {
            TerminalLine::ChangeIn {
                name: name.to_string(),
            }
        })(input)
    }

    fn parse_directory(input: &str) -> IResult<&str, Self> {
        map(preceded(tag("dir "), rest), |name: &str| {
            TerminalLine::Directory {
                name: name.to_string(),
            }
        })(input)
    }

    fn parse_file(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(map_res(digit1, str::parse), tag(" "), rest),
            |(size, name)| TerminalLine::File {
                name: name.to_string(),
                size,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::TerminalLine;

    #[rstest]
    #[case::list("$ ls", TerminalLine::List)]
    #[case::change_root("$ cd /", TerminalLine::ChangeRoot)]
    #[case::change_out("$ cd ..", TerminalLine::ChangeOut)]
    #[case::change_in("$ cd abc", TerminalLine::ChangeIn { name: "abc".to_string() })]
    #[case::directory("dir def", TerminalLine::Directory { name: "def".to_string() })]
    #[case::file("12345 ghi", TerminalLine::File { name: "ghi".to_string(), size: 12345 })]
    fn test_parse_terminal_line(#[case] input: &str, #[case] expected: TerminalLine) {
        let line: TerminalLine = input.parse().unwrap();
        assert_eq!(line, expected);
    }
}
