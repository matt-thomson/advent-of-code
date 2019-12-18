use itertools::Itertools;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct CompressedPath {
    main: Vec<FunctionCall>,
    a: Vec<Instruction>,
    b: Vec<Instruction>,
    c: Vec<Instruction>,
}

impl CompressedPath {
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.main.iter().map(|f| f.to_str()).join(","));
        result.push('\n');

        for f in &[&self.a, &self.b, &self.c] {
            result.push_str(&f.iter().map(|i| i.to_string()).join(","));
            result.push('\n');
        }

        result
    }
}

#[derive(Debug, PartialEq)]
enum FunctionCall {
    A,
    B,
    C,
}

impl FunctionCall {
    fn to_str(&self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

pub fn compress(path: &[Instruction]) -> CompressedPath {
    for a_len in 1..=10.min(path.len()) {
        let a = &path[0..a_len];
        let rest = remove_prefixes(&path, &[&a]);

        for b_len in 1..=10.min(rest.len()) {
            let b = &rest[0..b_len];
            let rest = remove_prefixes(&path, &[&a, &b]);

            for c_len in 1..=10.min(rest.len()) {
                let c = &rest[0..c_len];

                if remove_prefixes(&rest, &[&a, &b, &c]).is_empty() {
                    let main = find_calls(&path, &a, &b, &c);
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
