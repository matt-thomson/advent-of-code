use crate::board::Board;

#[derive(Debug)]
pub struct Game<'a> {
    board: &'a Board,
    marked: [[bool; 5]; 5],
}

pub struct Result {
    pub steps: usize,
    pub score: u32,
}

pub fn play(board: &Board, numbers: &[u32]) -> Result {
    let mut game = Game {
        board,
        marked: Default::default(),
    };

    for (index, number) in numbers.iter().enumerate() {
        game.mark(*number);

        if game.is_winner() {
            return Result {
                steps: index,
                score: game.score(),
            };
        }
    }

    panic!("Board did not win");
}

impl<'a> Game<'a> {
    fn mark(&mut self, number: u32) {
        if let Some((x, y)) = self.board.position(number) {
            self.marked[y][x] = true;
        }
    }

    fn is_winner(&self) -> bool {
        self.row_marked() || self.column_marked()
    }

    fn score(&self) -> u32 {
        (0..5)
            .flat_map(|x| (0..5).map(move |y| (x, y)))
            .filter(|(x, y)| !self.marked[*y][*x])
            .map(|(x, y)| self.board.number(x, y))
            .sum()
    }

    fn row_marked(&self) -> bool {
        (0..5).any(|y| (0..5).all(|x| self.marked[y][x]))
    }

    fn column_marked(&self) -> bool {
        (0..5).any(|x| (0..5).all(|y| self.marked[y][x]))
    }
}
