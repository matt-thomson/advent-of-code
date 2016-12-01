#[macro_use]
extern crate nom;
extern crate rand;

mod machine;

use machine::Machine;

use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");

    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
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

fn solve_part_two(filename: &str) -> u32 {
    let mut file = BufReader::new(File::open(filename).unwrap());
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let machine = Machine::parse(&input);

    let mut best_so_far = u32::max_value();

    for _ in 0..100 {
        match try_substitution(&machine) {
            Some(value) => best_so_far = min(best_so_far, value),
            None        => ()
        }
    };

    best_so_far
}

fn try_substitution(machine: &Machine) -> Option<u32> {
    let mut medicine = machine.medicine().clone();
    let mut count = 0;

    loop {
        if medicine == ["e"] {
            return Some(count);
        }

        let (position, length, replacement) = match find_substitution(&medicine, &machine) {
            Some(x) => x,
            None    => return None
        };

        for _ in 0..length {
            medicine.remove(position);
        }

        medicine.insert(position, replacement);

        count += 1;
    }
}

fn find_substitution(medicine: &Vec<String>, machine: &Machine) -> Option<(usize, usize, String)> {
    for (from, to) in machine.shuffled_substitutions() {
        if to.len() > medicine.len() {
            continue;
        }

        for i in 0..(medicine.len() + 1 - to.len()) {
            let slice = &medicine[i..(i + to.len())];

            if slice == &to[..] {
                return Some((i, to.len(), from.clone()));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 3);
    }
}
