use std::collections::HashSet;
use std::env;

const ASCII_A: u8 = 97;
const ASCII_Z: u8 = 122;

fn main() {
    let input = env::args().nth(1).expect("Must supply an input");
    println!("{}", solve_part_one(&input));
}

fn solve_part_one(input: &str) -> String {
    let mut password = next_string(input);

    loop {
        if is_password(&password) {
            return password;
        } else {
            password = next_string(&password);
        }
    }
}

fn is_password(input: &str) -> bool {
    contains_straight(input) && no_bad_chars(input) && contains_two_pairs(input)
}

fn next_string(input: &str) -> String {
    let mut bytes = input.bytes().collect::<Vec<_>>();

    for i in (0..bytes.len()).rev() {
        let byte = bytes[i];
        if byte == ASCII_Z {
            bytes[i] = ASCII_A;
        } else {
            bytes[i] = byte + 1;
            break;
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn contains_straight(input: &str) -> bool {
    byte_triples(input).iter().any(|&(c1, c2, c3)| c1 + 1 == c2 && c1 + 2 == c3)
}

fn no_bad_chars(input: &str) -> bool {
    input.chars().all(|c| c != 'i' && c != 'o' && c != 'l')
}

fn contains_two_pairs(input: &str) -> bool {
    let mut seen = HashSet::new();

    for (c1, c2) in char_pairs(input) {
        if c1 == c2 {
            seen.insert(c1);
        }
    }

    seen.len() >= 2
}

fn char_pairs(string: &str) -> Vec<(char, char)> {
    let first = string.chars();
    let second = string.chars().skip(1);

    first.zip(second).collect()
}

fn byte_triples(input: &str) -> Vec<(u8, u8, u8)> {
    let first = input.bytes();
    let second = input.bytes().skip(1);
    let third = input.bytes().skip(2);

    first.zip(second).zip(third).map(|((first, second), third)| (first, second, third)).collect()
}

#[cfg(test)]
mod tests {
    use super::{is_password, next_string, solve_part_one};

    #[test]
    fn test_password_example_1() {
        assert!(is_password("fghmmnn"));
    }

    #[test]
    fn test_password_example_2() {
        assert!(!is_password("hijklmmn"));
    }

    #[test]
    fn test_password_example_3() {
        assert!(!is_password("abbceffg"));
    }

    #[test]
    fn test_password_example_4() {
        assert!(!is_password("abbcegjk"));
    }

    #[test]
    fn test_password_example_5() {
        assert!(!is_password("abbbcegjk"));
    }

    #[test]
    fn test_next_string_example_1() {
        assert_eq!(next_string("abc"), "abd");
    }

    #[test]
    fn test_next_string_example_2() {
        assert_eq!(next_string("abz"), "aca");
    }

    #[test]
    fn test_next_string_example_3() {
        assert_eq!(next_string("azz"), "baa");
    }

    #[test]
    fn test_part_one_example_1() {
        assert_eq!(solve_part_one("abcdefgh"), "abcdffaa");
    }

    #[test]
    fn test_part_one_example_2() {
        assert_eq!(solve_part_one("ghijklmn"), "ghjaabcc");
    }
}
