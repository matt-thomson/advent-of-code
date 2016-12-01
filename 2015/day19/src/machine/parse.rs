use super::Machine;

use nom::{multispace, space, IResult};

pub fn parse(input: &str) -> Machine {
    let result = machine(input.as_bytes());

    match result {
        IResult::Done(_, (substitutions, input)) => Machine::new(substitutions, input),
        _                                        => panic!("Invalid result {:?}", result)
    }
}

named!(upper<char>, one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
named!(lower<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));

named!(element<String>, chain!(
    upper: upper ~
    lower: complete!(lower)?, || {
        let mut result = String::new();
        result.push(upper);

        match lower {
            Some(l) => result.push(l),
            None    => ()
        };

        result
    }
));
named!(electron<String>, map!(tag!("e"), |_| "e".to_string()));

named!(molecule<Vec<String> >, many1!(element));

named!(substitution<(String, Vec<String>)>, chain!(
    input: alt!(element | electron) ~
    space                           ~
    tag!("=>")                      ~
    space                           ~
    output: molecule                ~
    multispace, || (input, output)
));

named!(machine<(Vec<(String, Vec<String>)>, Vec<String>)>, chain!(
    substitutions: many0!(substitution) ~
    input: molecule, || (substitutions, input)
));

#[cfg(test)]
mod tests {
    use super::{element, molecule, substitution};

    use nom::IResult::Done;

    #[test]
    fn test_element_one_char() {
        let result = element(b"H");

        match result {
            Done(_, element) => assert_eq!(element, "H"),
            _                => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_element_two_chars() {
        let result = element(b"He");

        match result {
            Done(_, element) => assert_eq!(element, "He"),
            _                => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_molecule() {
        let result = molecule(b"HHeONaS");

        match result {
            Done(_, molecule) => assert_eq!(molecule, vec!["H", "He", "O", "Na", "S"]),
             _                => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_substitution() {
        let result = substitution(b"H => HeO");
        match result {
            Done(_, (input, output)) => {
                assert_eq!(input, "H");
                assert_eq!(output, vec!["He", "O"]);
            },
             _                       => panic!("Incorrect result {:?}", result)
        }
    }

    #[test]
    fn test_electron_substitution() {
        let result = substitution(b"e => HeO");
        match result {
            Done(_, (input, output)) => {
                assert_eq!(input, "e");
                assert_eq!(output, vec!["He", "O"]);
            },
             _                       => panic!("Incorrect result {:?}", result)
        }
    }
}
