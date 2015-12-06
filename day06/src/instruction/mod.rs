mod parse;

#[derive(Debug)]
pub struct Instruction {
    action: Action,
    top: usize,
    left: usize,
    bottom: usize,
    right: usize
}

#[derive(Debug, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle
}

impl Instruction {
    fn new(action: Action, top: usize, left: usize, bottom: usize, right: usize) -> Instruction {
        Instruction {
            action: action,
            top: top,
            left: left,
            bottom: bottom,
            right: right
        }
    }

    pub fn parse(line: &str) -> Instruction {
        parse::parse(line)
    }

    pub fn english(&self, grid: &mut [[bool; 1000]; 1000]) {
        for x in self.left..(self.right + 1) {
            for y in self.top..(self.bottom + 1) {
                grid[x][y] = match self.action {
                    Action::TurnOn  => true,
                    Action::TurnOff => false,
                    Action::Toggle  => !grid[x][y]
                }
            }
        }
    }

    pub fn elvish(&self, grid: &mut [[u16; 1000]; 1000]) {
        for x in self.left..(self.right + 1) {
            for y in self.top..(self.bottom + 1) {
                grid[x][y] = match self.action {
                    Action::TurnOn  => grid[x][y] + 1,
                    Action::TurnOff => if grid[x][y] > 0 { grid[x][y] - 1 } else { 0 },
                    Action::Toggle  => grid[x][y] + 2
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, Instruction};

    #[test]
    fn should_parse_instruction() {
        let instruction = Instruction::parse("turn on 1,2 through 3,4");
        assert_eq!(instruction.action, Action::TurnOn);
        assert_eq!(instruction.top, 2);
        assert_eq!(instruction.left, 1);
        assert_eq!(instruction.bottom, 4);
        assert_eq!(instruction.right, 3);
    }

    #[test]
    fn should_apply_turn_on_in_english() {
        let mut grid = [[false; 1000]; 1000];
        let instruction = Instruction::new(Action::TurnOn, 1, 2, 3, 4);

        instruction.english(&mut grid);

        assert!(grid[2][1]);
        assert!(grid[3][1]);
        assert!(grid[4][1]);
        assert!(grid[2][2]);
        assert!(grid[3][2]);
        assert!(grid[4][2]);
        assert!(grid[2][3]);
        assert!(grid[3][3]);
        assert!(grid[4][3]);
    }

    #[test]
    fn should_apply_turn_off_in_english() {
        let mut grid = [[true; 1000]; 1000];
        let instruction = Instruction::new(Action::TurnOff, 1, 2, 3, 4);

        instruction.english(&mut grid);

        assert!(!grid[2][1]);
        assert!(!grid[3][1]);
        assert!(!grid[4][1]);
        assert!(!grid[2][2]);
        assert!(!grid[3][2]);
        assert!(!grid[4][2]);
        assert!(!grid[2][3]);
        assert!(!grid[3][3]);
        assert!(!grid[4][3]);
    }

    #[test]
    fn should_apply_toggle_in_english() {
        let mut grid = [[false; 1000]; 1000];
        grid[3][2] = true;

        let instruction = Instruction::new(Action::Toggle, 1, 2, 3, 4);

        instruction.english(&mut grid);

        assert!(grid[2][1]);
        assert!(grid[3][1]);
        assert!(grid[4][1]);
        assert!(grid[2][2]);
        assert!(!grid[3][2]);
        assert!(grid[4][2]);
        assert!(grid[2][3]);
        assert!(grid[3][3]);
        assert!(grid[4][3]);
    }

    #[test]
    fn should_apply_turn_on_in_elvish() {
        let mut grid = [[0; 1000]; 1000];
        let instruction = Instruction::new(Action::TurnOn, 1, 2, 3, 4);

        instruction.elvish(&mut grid);

        assert_eq!(grid[2][1], 1);
        assert_eq!(grid[3][1], 1);
        assert_eq!(grid[4][1], 1);
        assert_eq!(grid[2][2], 1);
        assert_eq!(grid[3][2], 1);
        assert_eq!(grid[4][2], 1);
        assert_eq!(grid[2][3], 1);
        assert_eq!(grid[3][3], 1);
        assert_eq!(grid[4][3], 1);
    }

    #[test]
    fn should_apply_turn_off_in_elvish() {
        let mut grid = [[1; 1000]; 1000];
        grid[3][2] = 0;

        let instruction = Instruction::new(Action::TurnOff, 1, 2, 3, 4);

        instruction.elvish(&mut grid);

        assert_eq!(grid[2][1], 0);
        assert_eq!(grid[3][1], 0);
        assert_eq!(grid[4][1], 0);
        assert_eq!(grid[2][2], 0);
        assert_eq!(grid[3][2], 0);
        assert_eq!(grid[4][2], 0);
        assert_eq!(grid[2][3], 0);
        assert_eq!(grid[3][3], 0);
        assert_eq!(grid[4][3], 0);
    }

    #[test]
    fn should_apply_toggle_in_elvish() {
        let mut grid = [[1; 1000]; 1000];
        grid[3][2] = 0;

        let instruction = Instruction::new(Action::Toggle, 1, 2, 3, 4);

        instruction.elvish(&mut grid);

        assert_eq!(grid[2][1], 3);
        assert_eq!(grid[3][1], 3);
        assert_eq!(grid[4][1], 3);
        assert_eq!(grid[2][2], 3);
        assert_eq!(grid[3][2], 2);
        assert_eq!(grid[4][2], 3);
        assert_eq!(grid[2][3], 3);
        assert_eq!(grid[3][3], 3);
        assert_eq!(grid[4][3], 3);
    }
}
