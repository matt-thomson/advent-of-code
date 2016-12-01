use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> usize {
    solve(filename, &|string| is_nice(string))
}

fn solve_part_two(filename: &str) -> usize {
    solve(filename, &|string| is_new_nice(string))
}

fn solve(filename: &str, predicate: &Fn(&str) -> bool) -> usize {
    BufReader::new(File::open(filename).unwrap()).lines()
        .map(|line| line.unwrap())
        .filter(|string| predicate(&string))
        .count()
}

fn is_nice(string: &str) -> bool {
    let char_pairs = char_pairs(string);
    has_three_vowels(string) && has_double_letter(&char_pairs) && has_no_bad_pairs(&char_pairs)
}

fn is_new_nice(string: &str) -> bool {
    let char_pairs = char_pairs(string);
    let char_triples = char_triples(string);
    has_repeating_pair(&char_pairs) && has_repeat_with_one_between(&char_triples)
}

fn char_pairs(string: &str) -> Vec<(char, char)> {
    let first = string.chars();
    let second = string.chars().skip(1);

    first.zip(second).collect()
}

fn char_triples(string: &str) -> Vec<(char, char, char)> {
    let first = string.chars();
    let second = string.chars().skip(1);
    let third = string.chars().skip(2);

    first.zip(second).zip(third).map(|((first, second), third)| (first, second, third)).collect()
}

fn has_three_vowels(string: &str) -> bool {
    string.chars().filter(|c| is_vowel(c)).count() >= 3
}

fn has_double_letter(char_pairs: &[(char, char)]) -> bool {
    char_pairs.iter().any(|&(first, second)| first == second)
}

fn has_no_bad_pairs(char_pairs: &[(char, char)]) -> bool {
    char_pairs.iter().all(|&(first, second)| !is_bad_pair(&first, &second))
}

fn has_repeating_pair(char_pairs: &[(char, char)]) -> bool {
    let mut seen = HashSet::new();
    let mut repeat = None;

    for pair in char_pairs {
        let (first, second) = *pair;
        if first == second {
            if repeat == Some(first) {
                repeat = None;
                continue;
            }

            repeat = Some(first);
        } else {
            repeat = None;
        }

        if seen.contains(&pair) {
            return true;
        }

        seen.insert(pair);
    }

    false
}

fn has_repeat_with_one_between(char_triples: &[(char, char, char)]) -> bool {
    char_triples.iter().any(|&(first, _, third)| first == third)
}

fn is_vowel(letter: &char) -> bool {
    match *letter {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _   => false
    }
}

fn is_bad_pair(first: &char, second: &char) -> bool {
    match (*first, *second) {
        ('a', 'b') => true,
        ('c', 'd') => true,
        ('p', 'q') => true,
        ('x', 'y') => true,
        _          => false
    }
}

#[cfg(test)]
mod tests {
    use super::{is_nice, is_new_nice, solve_part_one, solve_part_two};

    #[test]
    fn test_nice_example_1() {
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_nice_example_2() {
        assert!(is_nice("aaa"));
    }

    #[test]
    fn test_nice_example_3() {
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_nice_example_4() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_nice_example_5() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_new_nice_example_1() {
        assert!(is_new_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_new_nice_example_2() {
        assert!(is_new_nice("xxyxx"));
    }

    #[test]
    fn test_new_nice_example_3() {
        assert!(!is_new_nice("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_new_nice_example_4() {
        assert!(!is_new_nice("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example1.txt"), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example2.txt"), 2);
    }
}
