use std::fs;
use std::path::Path;

pub fn parse(path: &Path) -> Vec<u32> {
    fs::read_to_string(&path)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn run(program: &mut [u32]) {
    for counter in (0..).step_by(4) {
        match program[counter] {
            1 => {
                let first = program[counter + 1] as usize;
                let second = program[counter + 2] as usize;
                let location = program[counter + 3] as usize;
                program[location] = program[first] + program[second];
            }
            2 => {
                let first = program[counter + 1] as usize;
                let second = program[counter + 2] as usize;
                let location = program[counter + 3] as usize;
                program[location] = program[first] * program[second];
            }
            99 => break,
            _ => unreachable!(),
        }
    }
}

mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_parse() {
        let path = PathBuf::from("fixtures/day02.txt");
        let program = parse(&path);

        assert_eq!(program, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50, 60]);
    }

    #[test]
    fn test_run_example_one() {
        let mut program = vec![1, 0, 0, 0, 99];
        run(&mut program);

        assert_eq!(program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_example_two() {
        let mut program = vec![2, 3, 0, 3, 99];
        run(&mut program);

        assert_eq!(program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_example_three() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        run(&mut program);

        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_example_four() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut program);

        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_run_example_five() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut program);

        assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
