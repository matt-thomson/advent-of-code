use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct CompressedPath {
    main: Vec<FunctionCall>,
    a: Vec<Instruction>,
    b: Vec<Instruction>,
    c: Vec<Instruction>,
}

impl Display for CompressedPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", &self.main.iter().format(","))?;

        for func in &[&self.a, &self.b, &self.c] {
            writeln!(f, "{}", &func.iter().format(","))?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum FunctionCall {
    A,
    B,
    C,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
        }
    }
}

pub fn compress(path: &[Instruction]) -> CompressedPath {
    for a_len in 1..=10.min(path.len()) {
        let a = &path[0..a_len];
        let rest = remove_prefixes(path, &[a]);

        for b_len in 1..=10.min(rest.len()) {
            let b = &rest[0..b_len];
            let rest = remove_prefixes(path, &[a, b]);

            for c_len in 1..=10.min(rest.len()) {
                let c = &rest[0..c_len];

                if remove_prefixes(rest, &[a, b, c]).is_empty() {
                    let main = find_calls(path, a, b, c);
                    if main.len() <= 10 {
                        return CompressedPath {
                            main,
                            a: a.to_vec(),
                            b: b.to_vec(),
                            c: c.to_vec(),
                        };
                    }
                }
            }
        }
    }

    unreachable!();
}

fn remove_prefixes<'a>(
    path: &'a [Instruction],
    prefixes: &[&'a [Instruction]],
) -> &'a [Instruction] {
    let mut path = path;

    loop {
        let mut updated = false;
        for prefix in prefixes {
            while path.starts_with(prefix) {
                path = &path[prefix.len()..];
                updated = true;
            }
        }

        if !updated {
            break;
        }
    }

    path
}

fn find_calls(
    path: &[Instruction],
    a: &[Instruction],
    b: &[Instruction],
    c: &[Instruction],
) -> Vec<FunctionCall> {
    let mut result = vec![];
    let mut path = path;

    while !path.is_empty() {
        if path.starts_with(a) {
            result.push(FunctionCall::A);
            path = &path[a.len()..];
        } else if path.starts_with(b) {
            result.push(FunctionCall::B);
            path = &path[b.len()..];
        } else if path.starts_with(c) {
            result.push(FunctionCall::C);
            path = &path[c.len()..];
        } else {
            unreachable!();
        }
    }

    result
}
