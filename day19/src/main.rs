#[macro_use]
extern crate nom;

mod machine;

use machine::Machine;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");

    println!("{}", solve_part_one(&filename));
}

fn solve_part_one(filename: &str) -> usize {
    let mut file = BufReader::new(File::open(filename).unwrap());
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let machine = Machine::parse(&input);
    let medicine = machine.medicine();

    let mut seen = HashSet::new();

    for i in 0..medicine.len() {
        let before = &medicine[0..i];
        let after = &medicine[(i+1)..];
        let current = &medicine[i];

        match machine.substitutions(current) {
            Some(subs) => {
                for sub in subs {
                    let mut result = vec![];
                    result.extend(before);
                    result.extend(sub);
                    result.extend(after);

                    seen.insert(result);
                }
            }
            None => ()
        }
    }

    seen.len()
}

#[cfg(test)]
mod tests {
    use super::solve_part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 4);
    }
}
