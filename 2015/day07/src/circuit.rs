use gate::{Action, Gate, Input};

use std::collections::HashMap;

pub struct Circuit {
    gates: HashMap<String, Action>
}

impl Circuit {
    pub fn parse(lines: &mut Iterator<Item = String>) -> Circuit {
        let mut gates = HashMap::new();

        for line in lines {
            let gate = Gate::parse(&line);
            gates.insert(gate.output, gate.action);
        }

        Circuit { gates: gates }
    }

    pub fn result(&self, wire: &str) -> u16 {
        let mut cache = HashMap::new();
        self.output(wire, &mut cache)
    }

    pub fn result_with_override(&self, value: u16, wire: &str) -> u16 {
        let mut cache = HashMap::new();
        cache.insert("b".to_string(), value);

        self.output(wire, &mut cache)
    }

    fn output(&self, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
        if cache.contains_key(wire) {
            return cache[wire];
        }

        let value = match self.gates[wire] {
            Action::Literal(ref input)          => self.input(&input, cache),
            Action::And(ref input1, ref input2) => self.input(&input1, cache) & self.input(&input2, cache),
            Action::Or(ref input1, ref input2)  => self.input(&input1, cache) | self.input(&input2, cache),
            Action::Not(ref input)              => !self.input(&input, cache),
            Action::LShift(ref input, amount)   => self.input(&input, cache) << amount,
            Action::RShift(ref input, amount)   => self.input(&input, cache) >> amount
        };

        cache.insert(wire.to_string(), value);
        value
    }

    fn input(&self, input: &Input, cache: &mut HashMap<String, u16>) -> u16 {
        match input {
            &Input::Number(x) => x,
            &Input::Wire(ref wire) => self.output(&wire, cache)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Circuit;
    use gate::{Action, Input};

    #[test]
    fn test_parses_circuit() {
        let input = vec!["1 -> x".to_string(), "2 -> y".to_string(), "x OR y -> z".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.gates.len(), 3);

        let ref action = circuit.gates["y"];
        assert_eq!(*action, Action::Literal(Input::Number(2)));
    }

    #[test]
    fn test_computes_literal() {
        let input = vec!["1 -> x".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("x"), 1);
    }

    #[test]
    fn test_computes_pass_through() {
        let input = vec!["1 -> x".to_string(), "x -> y".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("y"), 1);
    }

    #[test]
    fn test_computes_and() {
        let input = vec!["3 -> x".to_string(), "6 -> y".to_string(), "x AND y -> z".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("z"), 2);
    }

    #[test]
    fn test_computes_or() {
        let input = vec!["3 -> x".to_string(), "6 -> y".to_string(), "x OR y -> z".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("z"), 7);
    }

    #[test]
    fn test_computes_not() {
        let input = vec!["21845 -> x".to_string(), "NOT x -> y".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("y"), 43690);
    }

    #[test]
    fn test_computes_lshift() {
        let input = vec!["50 -> x".to_string(), "x LSHIFT 2 -> y".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("y"), 200);
    }

    #[test]
    fn test_computes_rshift() {
        let input = vec!["50 -> x".to_string(), "x RSHIFT 2 -> y".to_string()];
        let circuit = Circuit::parse(&mut input.into_iter());

        assert_eq!(circuit.result("y"), 12);
    }
}
