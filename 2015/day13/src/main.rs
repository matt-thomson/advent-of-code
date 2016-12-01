#[macro_use]
extern crate nom;

mod family;
mod relationship;

use relationship::Relationship;
use family::Family;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> i32 {
    let family = read_family(filename);
    find_arrangement(&family, &family.people()[0], &vec![], 0)
}

fn solve_part_two(filename: &str) -> i32 {
    let mut family = read_family(filename);
    family.add_yourself();

    find_arrangement(&family, &family.people()[0], &vec![], 0)
}

fn read_family(filename: &str) -> Family {
    let file = BufReader::new(File::open(filename).unwrap());
    let relationships = file.lines().map(|line| Relationship::parse(&line.unwrap()));
    Family::new(relationships)
}

fn find_arrangement(family: &Family,
                    current: &str,
                    seated: &Vec<String>,
                    happiness: i32) -> i32 {
    let mut new_seated = seated.clone();
    new_seated.push(current.to_string());

    if new_seated.len() == family.people().len() {
        return happiness + family.happiness(current, &seated[0]);
    }

    let mut best = None;

    for person in family.people() {
        if new_seated.contains(person) {
            continue;
        }

        let step = happiness + family.happiness(current, person);
        let happiness = find_arrangement(family, &person, &new_seated, step);

        best = match best {
            Some(prev_best) => if happiness > prev_best { Some(happiness) } else { Some(prev_best) },
            None => Some(happiness)
        };
    }

    best.unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve_part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 330);
    }
}
