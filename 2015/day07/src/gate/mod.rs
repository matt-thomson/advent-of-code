mod parse;

#[derive(Debug)]
pub struct Gate {
    pub action: Action,
    pub output: String
}

impl Gate {
    fn new(action: Action, output: String) -> Gate {
        Gate { action: action, output: output }
    }

    pub fn parse(line: &str) -> Gate {
        parse::parse(line)
    }
}

#[derive(Debug, PartialEq)]
pub enum Input {
    Number(u16),
    Wire(String)
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Literal(Input),
    And(Input, Input),
    Or(Input, Input),
    Not(Input),
    LShift(Input, u16),
    RShift(Input, u16)
}

#[cfg(test)]
mod tests {
    use super::{Action, Gate, Input};

    #[test]
    fn test_should_parse_number_gate() {
        let gate = Gate::parse("123 -> x");
        assert_eq!(gate.action, Action::Literal(Input::Number(123)));
        assert_eq!(gate.output, "x");
    }

    #[test]
    fn test_should_parse_wire_gate() {
        let gate = Gate::parse("y -> x");
        assert_eq!(gate.action, Action::Literal(Input::Wire("y".to_string())));
        assert_eq!(gate.output, "x");
    }

    #[test]
    fn test_should_parse_and_gate() {
        let gate = Gate::parse("a AND 1 -> c");
        assert_eq!(gate.action, Action::And(Input::Wire("a".to_string()), Input::Number(1)));
        assert_eq!(gate.output, "c");
    }

    #[test]
    fn test_should_parse_or_gate() {
        let gate = Gate::parse("a OR 1 -> c");
        assert_eq!(gate.action, Action::Or(Input::Wire("a".to_string()), Input::Number(1)));
        assert_eq!(gate.output, "c");
    }

    #[test]
    fn test_should_parse_not_gate() {
        let gate = Gate::parse("NOT a -> b");
        assert_eq!(gate.action, Action::Not(Input::Wire("a".to_string())));
        assert_eq!(gate.output, "b");
    }

    #[test]
    fn test_should_parse_lshift_gate() {
        let gate = Gate::parse("a LSHIFT 1 -> b");
        assert_eq!(gate.action, Action::LShift(Input::Wire("a".to_string()), 1));
        assert_eq!(gate.output, "b");
    }

    #[test]
    fn test_should_parse_rshift_gate() {
        let gate = Gate::parse("a RSHIFT 1 -> b");
        assert_eq!(gate.action, Action::RShift(Input::Wire("a".to_string()), 1));
        assert_eq!(gate.output, "b");
    }
}
