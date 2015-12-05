extern crate md5;

use std::env;

fn main() {
    let input = env::args().nth(1).expect("Must supply an input");
    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));
}

fn solve_part_one(input: &str) -> u32 {
    (1..).find(|&x| is_advent_coin(&digest(input, x))).unwrap()
}

fn solve_part_two(input: &str) -> u32 {
    (1..).find(|&x| is_super_advent_coin(&digest(input, x))).unwrap()
}

fn is_advent_coin(digest: &[u8]) -> bool {
    digest[0] == 0 && digest[1] == 0 && digest[2] < 16
}

fn is_super_advent_coin(digest: &[u8]) -> bool {
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

fn digest(input: &str, number: u32) -> [u8; 16] {
    let data = format!("{}{}", input, number).bytes().collect::<Vec<_>>();
    md5::compute(&data)
}

#[cfg(test)]
mod tests {
    use super::solve_part_one;

    #[test]
    fn test_part_one_example_1() {
        assert_eq!(solve_part_one("abcdef"), 609043);
    }

    #[test]
    fn test_part_one_example_2() {
        assert_eq!(solve_part_one("pqrstuv"), 1048970);
    }
}
