#[derive(Debug)]
pub struct Board {
    contents: Vec<Vec<u32>>,
}

impl Board {
    pub fn new(lines: &[String]) -> Self {
        let contents = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect()
            })
            .collect();

        Self { contents }
    }

    pub fn position(&self, number: u32) -> Option<(usize, usize)> {
        (0..5)
            .flat_map(|x| (0..5).map(move |y| (x, y)))
            .find(|&(x, y)| self.contents[y][x] == number)
    }

    pub fn number(&self, x: usize, y: usize) -> u32 {
        self.contents[y][x]
    }
}
