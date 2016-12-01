use std::env;

fn main() {
    let row = env::args().nth(1).expect("Must supply row").parse().unwrap();
    let column = env::args().nth(2).expect("Must supply column").parse().unwrap();

    println!("{}", solve_part_one(row, column));
}

fn solve_part_one(row: u32, column: u32) -> u32 {
    let diagonal = |sum: u32| (1..sum).map(move |x| (x, sum - x));
    let coordinates = (2..).flat_map(move |x| diagonal(x));

    let mut value: u64 = 20151125;

    for (x, y) in coordinates {
        if x == column && y == row {
            return value as u32;
        }

        value = (value * 252533) % 33554393;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::solve_part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(6, 6), 27995004);
    }
}
