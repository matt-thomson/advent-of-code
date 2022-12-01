#[derive(Debug, Hash, PartialEq, Eq)]
pub struct State {
    positions: [u32; 2],
    scores: [u32; 2],
    next_player: usize,
}

impl State {
    pub fn new(positions: &[u32; 2]) -> Self {
        Self {
            positions: *positions,
            scores: [0, 0],
            next_player: 0,
        }
    }

    pub fn next(&self, roll: u32) -> Self {
        let mut positions = self.positions;
        let mut scores = self.scores;

        positions[self.next_player] = (positions[self.next_player] + roll - 1) % 10 + 1;
        scores[self.next_player] += positions[self.next_player];

        let next_player = 1 - self.next_player;

        Self {
            positions,
            scores,
            next_player,
        }
    }

    pub fn winner(&self, required: u32) -> Option<usize> {
        (0..=1).find(|&player| self.scores[player] >= required)
    }

    pub fn score(&self, player: usize) -> u32 {
        self.scores[player]
    }

    pub fn all(required: u32) -> Vec<Self> {
        (0..required)
            .flat_map(|score1| {
                (0..required).flat_map(move |score2| {
                    (1..=10).flat_map(move |position1| {
                        (1..=10).flat_map(move |position2| {
                            (0..=1).map(move |next_player| Self {
                                positions: [position1, position2],
                                scores: [score1, score2],
                                next_player,
                            })
                        })
                    })
                })
            })
            .collect()
    }
}
