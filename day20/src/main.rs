use std::env;

fn main() {
    let input = env::args().nth(1).expect("Must supply an input").parse().unwrap();

    println!("{}", solve_part_one(input));
}

fn solve_part_one(input: u32) -> u32 {
    let mut candidate = 1;

    loop {
        let factor_sum = (1..(candidate + 1))
            .filter(|x| candidate % x == 0)
            .fold(0, |acc, x| acc + x);

        if factor_sum * 10 >= input {
            return candidate;
        }

        candidate += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::solve_part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(140), 8);
    }
}
