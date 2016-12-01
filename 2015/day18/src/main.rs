mod grid;

use grid::Grid;

use std::env;

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    let num_steps = env::args().nth(2).expect("Must supply a number of steps").parse().unwrap();

    println!("{}", solve_part_one(&filename, num_steps));
    println!("{}", solve_part_two(&filename, num_steps));
}

fn solve_part_one(filename: &str, num_steps: usize) -> usize {
    let mut grid = Grid::parse(filename);

    for _ in 0..num_steps {
        grid = grid.step();
    }

    grid.live_cells()
}

fn solve_part_two(filename: &str, num_steps: usize) -> usize {
    let mut grid = Grid::parse(filename);

    for _ in 0..num_steps {
        grid = grid.turn_corners_on().step();
    }

    grid.turn_corners_on().live_cells()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt", 4), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt", 5), 17);
    }
}
