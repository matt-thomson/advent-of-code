use std::collections::BTreeMap;
use std::env;

fn main() {
    let input = env::args().nth(1).expect("Must supply an input").parse().unwrap();

    println!("{}", solve_part_one(input));
    println!("{}", solve_part_two(input));
}

fn solve_part_one(input: u32) -> u32 {
    let mut candidate = 1;

    loop {
        let prime_factors = prime_factors(candidate).into_iter().collect::<Vec<_>>();
        let factor_sum = all_factors(&prime_factors).iter().fold(0, |acc, i| acc + i);

        if factor_sum * 10 >= input {
            return candidate;
        }

        candidate += 1;
    }
}

fn solve_part_two(input: u32) -> u32 {
    let mut candidate = 1;

    loop {
        let prime_factors = prime_factors(candidate).into_iter().collect::<Vec<_>>();
        let factor_sum = all_factors(&prime_factors).iter()
            .filter(|factor| *factor * 50 >= candidate)
            .fold(0, |acc, i| acc + i);

        if factor_sum * 11 >= input {
            return candidate;
        }

        candidate += 1;
    }
}

fn prime_factors(input: u32) -> BTreeMap<u32, u32> {
    let mut result = BTreeMap::new();
    let mut remaining = input;

    for candidate in 2.. {
        if candidate * candidate > input {
            break;
        }

        while remaining % candidate == 0 {
            *result.entry(candidate).or_insert(0) += 1;
            remaining /= candidate;
        }

        if remaining == 1 {
            break;
        }
    }

    if remaining > 1 {
        *result.entry(remaining).or_insert(0) += 1;
    }

    result
}

fn all_factors(prime_factors: &[(u32, u32)]) -> Vec<u32> {
    if prime_factors.len() == 0 {
        vec![1]
    } else {
        let mut result = vec![];
        let (factor, count) = prime_factors[0];
        let other_factors = all_factors(&prime_factors[1..]);

        for i in 0..(count + 1) {
            let multiplier = factor.pow(i);
            for other_factor in other_factors.iter() {
                result.push(multiplier * other_factor);
            }
        }

        result
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
