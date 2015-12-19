use std::cmp::{min, max};
use std::fmt::{Debug, Error, Formatter, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Grid {
    cells: Vec<Vec<bool>>
}

impl Grid {
    pub fn parse(filename: &str) -> Grid {
        let file = BufReader::new(File::open(filename).unwrap());
        let cells = file.lines().map(|line| parse_line(&line.unwrap())).collect();

        Grid { cells: cells }
    }

    pub fn step(&self) -> Grid {
        let mut new_cells = vec![];

        for y in 0..self.cells.len() {
            let mut row = vec![];

            for x in 0..self.cells[0].len() {
                row.push(self.step_cell(x, y));
            }

            new_cells.push(row);
        }

        Grid { cells: new_cells }
    }

    pub fn turn_corners_on(&self) -> Grid {
        let mut new_cells = self.cells.clone();

        new_cells[0][0] = true;
        new_cells[0][self.cells[0].len() - 1] = true;
        new_cells[self.cells.len() - 1][0] = true;
        new_cells[self.cells.len() - 1][self.cells[0].len() - 1] = true;

        Grid { cells: new_cells }
    }

    pub fn live_cells(&self) -> usize {
        self.cells.iter().map(|row| row.iter().filter(|x| **x).count()).fold(0, |acc, i| acc + i)
    }

    fn step_cell(&self, x: usize, y: usize) -> bool {
        let live_neighbours = self.live_neighbours(x, y);

        live_neighbours == 3 || (live_neighbours == 2 && self.cells[y][x])
    }

    fn live_neighbours(&self, x: usize, y: usize) -> usize {
        let x_min = max(x, 1) - 1;
        let y_min = max(y, 1) - 1;

        let x_max = min(x, self.cells[0].len() - 2) + 1;
        let y_max = min(y, self.cells.len() - 2) + 1;

        let mut count = 0;

        for x in x_min..(x_max + 1) {
            for y in y_min..(y_max + 1) {
                if self.cells[y][x] {
                    count += 1;
                }
            }
        }

        if self.cells[y][x] {
            count -= 1;
        }

        count
    }
}

impl Debug for Grid {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        let mut result = Ok(());

        for row in self.cells.iter() {
            for cell in row {
                let char = if *cell { '#' } else { '.' };
                result = result.and(formatter.write_char(char));
            }

            result = result.and(formatter.write_char('\n'));
        }

        result
    }
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_parse_grid() {
        let grid = Grid::parse("data/example.txt");
        assert!(!grid.cells[0][0]);
        assert!(grid.cells[1][3]);
    }

    #[test]
    fn test_live_neighbours() {
        let grid = Grid::parse("data/example.txt");
        assert_eq!(grid.live_neighbours(0, 0), 1);
        assert_eq!(grid.live_neighbours(3, 1), 2);
        assert_eq!(grid.live_neighbours(5, 5), 1);
    }

    #[test]
    fn test_step() {
        let grid = Grid::parse("data/example.txt").step();
        assert!(!grid.cells[0][1]);
        assert!(grid.cells[0][2]);
    }

    #[test]
    fn test_live_cells() {
        let grid = Grid::parse("data/example.txt");
        assert_eq!(grid.live_cells(), 15);
    }
}
