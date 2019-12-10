use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use num::Integer;
use structopt::StructOpt;

use crate::problem::Problem;

type Point = (usize, usize);

#[derive(Debug, StructOpt)]
pub struct Day10 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day10 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let asteroids = self.read();

        asteroids
            .iter()
            .map(|asteroid| visible(&asteroids, *asteroid))
            .max()
            .unwrap()
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

impl Day10 {
    fn read(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        for (y, row) in reader.lines().enumerate() {
            for (x, position) in row.unwrap().chars().enumerate() {
                if position == '#' {
                    result.insert((x, y));
                }
            }
        }

        result
    }
}

fn points_between(first: Point, second: Point) -> Vec<Point> {
    let (first_x, first_y) = first;
    let (second_x, second_y) = second;
    let diff_x = (first_x as isize) - (second_x as isize);
    let diff_y = (first_y as isize) - (second_y as isize);

    let gcd = diff_x.gcd(&diff_y);
    let step_x = diff_x / gcd;
    let step_y = diff_y / gcd;

    (1..gcd.abs())
        .map(|i| (first_x as isize - step_x * i, first_y as isize - step_y * i))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn visible(asteroids: &HashSet<Point>, point: Point) -> usize {
    asteroids
        .iter()
        .filter(|asteroid| **asteroid != point)
        .filter(|asteroid| {
            !points_between(point, **asteroid)
                .iter()
                .any(|other| asteroids.contains(other))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let input = PathBuf::from("fixtures/day10a.txt");
        let problem = Day10 { input };

        let expected = vec![
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ];

        assert_eq!(problem.read(), expected.into_iter().collect());
    }

    #[test]
    fn test_points_between() {
        assert_eq!(points_between((1, 0), (4, 6)), vec![(2, 2), (3, 4)]);
        assert_eq!(points_between((4, 6), (1, 0)), vec![(3, 4), (2, 2)]);

        assert_eq!(points_between((1, 0), (4, 0)), vec![(2, 0), (3, 0)]);
        assert_eq!(points_between((0, 1), (0, 4)), vec![(0, 2), (0, 3)]);
    }

    #[test]
    fn test_visible() {
        let asteroids = vec![
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ]
        .into_iter()
        .collect();

        assert_eq!(visible(&asteroids, (3, 4)), 8);
        assert_eq!(visible(&asteroids, (4, 2)), 5);
    }

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day10a.txt");
        let problem = Day10 { input };

        assert_eq!(problem.part_one(), 8);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day10b.txt");
        let problem = Day10 { input };

        assert_eq!(problem.part_one(), 33);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day10c.txt");
        let problem = Day10 { input };

        assert_eq!(problem.part_one(), 35);
    }

    #[test]
    fn test_part_one_d() {
        let input = PathBuf::from("fixtures/day10d.txt");
        let problem = Day10 { input };

        assert_eq!(problem.part_one(), 41);
    }

    #[test]
    fn test_part_one_e() {
        let input = PathBuf::from("fixtures/day10e.txt");
        let problem = Day10 { input };

        assert_eq!(problem.part_one(), 210);
    }
}
