use std::env;

fn main() {
    let input = env::args().nth(1).expect("Must supply an input");
    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));
}

fn solve_part_one(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| acc + char_value(&c))
}

fn solve_part_two(input: &str) -> usize {
    let mut acc = 0;

    for (index, c) in input.chars().enumerate() {
        acc += char_value(&c);

        if acc == -1 {
            return index + 1;
        }
    }

    panic!("Could not solve");
}

fn char_value(c: &char) -> i32 {
    match *c {
        '(' => 1,
        ')' => -1,
        _   => panic!("Unknown character {}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one_example_1() {
        assert_eq!(solve_part_one("(())"), 0);
    }

    #[test]
    fn test_part_one_example_2() {
        assert_eq!(solve_part_one("()()"), 0);
    }

    #[test]
    fn test_part_one_example_3() {
        assert_eq!(solve_part_one("((("), 3);
    }

    #[test]
    fn test_part_one_example_4() {
        assert_eq!(solve_part_one("(()(()("), 3);
    }

    #[test]
    fn test_part_one_example_5() {
        assert_eq!(solve_part_one("))((((("), 3);
    }

    #[test]
    fn test_part_one_example_6() {
        assert_eq!(solve_part_one("())"), -1);
    }

    #[test]
    fn test_part_one_example_7() {
        assert_eq!(solve_part_one("))("), -1);
    }

    #[test]
    fn test_part_one_example_8() {
        assert_eq!(solve_part_one(")))"), -3);
    }

    #[test]
    fn test_part_one_example_9() {
        assert_eq!(solve_part_one(")())())"), -3);
    }

    #[test]
    fn test_part_two_example_1() {
        assert_eq!(solve_part_two(")"), 1);
    }

    #[test]
    fn test_part_two_example_2() {
        assert_eq!(solve_part_two("()())"), 5);
    }
}
