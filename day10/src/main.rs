use std::env;

fn main() {
    let input = env::args().nth(1).expect("Must supply an input");
    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));
}

fn solve_part_one(input: &str) -> usize {
    solve(input, 40)
}

fn solve_part_two(input: &str) -> usize {
    solve(input, 50)
}

fn solve(input: &str, steps: usize) -> usize {
    let mut value = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();

    for _ in 0..steps {
        value = look_and_say(&value);
    }

    value.len()
}

fn look_and_say(input: &[u32]) -> Vec<u32> {
    let mut current = input[0];
    let mut count = 1;
    let mut output = vec![];

    for value in input[1..].iter() {
        if *value == current {
            count += 1;
        } else {
            output.push(count);
            output.push(current);

            current = *value;
            count = 1;
        }
    }

    output.push(count);
    output.push(current);

    output
}

#[cfg(test)]
mod tests {
    use super::{look_and_say, solve};

    #[test]
    fn test_example_1() {
        assert_eq!(look_and_say(&vec![1]), vec![1, 1]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(look_and_say(&vec![1, 1]), vec![2, 1]);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(look_and_say(&vec![2, 1]), vec![1, 2, 1, 1]);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(look_and_say(&vec![1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(look_and_say(&vec![1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("1", 5), 6);
    }
}
