use std::cmp::Ordering;
use std::collections::HashSet;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use num::Integer;
use structopt::StructOpt;

use crate::problem::Problem;

type Point = (usize, usize);

#[derive(Debug, StructOpt)]
pub struct Day10 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "200")]
    winner: usize,
}

impl Problem for Day10 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let asteroids = read(&self.input);

        asteroids
            .iter()
            .map(|asteroid| visible(&asteroids, *asteroid).len())
            .max()
            .unwrap()
    }

    fn part_two(&self) -> usize {
        let asteroids = read(&self.input);
        let base = asteroids
            .iter()
            .max_by_key(|asteroid| visible(&asteroids, **asteroid).len())
            .unwrap();

        let (x, y) = destroyed(&asteroids, *base, 200);
        x * 100 + y
    }
}

fn read(path: &Path) -> HashSet<Point> {
    let mut result = HashSet::new();
    let file = File::open(path).unwrap();
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

fn visible(asteroids: &HashSet<Point>, point: Point) -> Vec<&Point> {
    asteroids
        .iter()
        .filter(|asteroid| **asteroid != point)
        .filter(|asteroid| {
            !points_between(point, **asteroid)
                .iter()
                .any(|other| asteroids.contains(other))
        })
        .collect()
}

fn destroyed(asteroids: &HashSet<Point>, base: Point, count: usize) -> Point {
    let mut visible = visible(asteroids, base);

    if count > visible.len() {
        let mut not_visible = asteroids.clone();
        for asteroid in &visible {
            not_visible.remove(asteroid);
        }

        destroyed(&not_visible, base, count - visible.len())
    } else {
        visible.sort_by(|asteroid1, asteroid2| {
            angle(base, **asteroid1)
                .partial_cmp(&angle(base, **asteroid2))
                .unwrap_or(Ordering::Equal)
        });

        *visible[count - 1]
    }
}

fn angle(first: Point, second: Point) -> f32 {
    let (first_x, first_y) = first;
    let (second_x, second_y) = second;
    let diff_x = (first_x as f32) - (second_x as f32);
    let diff_y = (first_y as f32) - (second_y as f32);

    (2_f32 - (diff_x.atan2(diff_y) / PI)) % 2_f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let input = PathBuf::from("fixtures/day10a.txt");

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

        assert_eq!(read(&input), expected.into_iter().collect());
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

        assert_eq!(visible(&asteroids, (3, 4)).len(), 8);
        assert_eq!(visible(&asteroids, (4, 2)).len(), 5);
    }

    #[test]
    fn test_destroyed() {
        let input = PathBuf::from("fixtures/day10e.txt");
        let asteroids = read(&input);

        let base = (11, 13);

        assert_eq!(destroyed(&asteroids, base, 1), (11, 12));
        assert_eq!(destroyed(&asteroids, base, 2), (12, 1));
        assert_eq!(destroyed(&asteroids, base, 3), (12, 2));
        assert_eq!(destroyed(&asteroids, base, 10), (12, 8));
        assert_eq!(destroyed(&asteroids, base, 20), (16, 0));
        assert_eq!(destroyed(&asteroids, base, 50), (16, 9));
        assert_eq!(destroyed(&asteroids, base, 100), (10, 16));
        assert_eq!(destroyed(&asteroids, base, 199), (9, 6));
        assert_eq!(destroyed(&asteroids, base, 200), (8, 2));
        assert_eq!(destroyed(&asteroids, base, 201), (10, 9));
        assert_eq!(destroyed(&asteroids, base, 299), (11, 1));
    }

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day10a.txt");
        let problem = Day10 { input, winner: 200 };

        assert_eq!(problem.part_one(), 8);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day10b.txt");
        let problem = Day10 { input, winner: 200 };

        assert_eq!(problem.part_one(), 33);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day10c.txt");
        let problem = Day10 { input, winner: 200 };

        assert_eq!(problem.part_one(), 35);
    }

    #[test]
    fn test_part_one_d() {
        let input = PathBuf::from("fixtures/day10d.txt");
        let problem = Day10 { input, winner: 200 };

        assert_eq!(problem.part_one(), 41);
    }

    #[test]
    fn test_part_one_e() {
        let input = PathBuf::from("fixtures/day10e.txt");
        let problem = Day10 { input, winner: 200 };

        assert_eq!(problem.part_one(), 210);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day10e.txt");
        let problem = Day10 { input, winner: 200 };

        assert_eq!(problem.part_two(), 802);
    }
}
