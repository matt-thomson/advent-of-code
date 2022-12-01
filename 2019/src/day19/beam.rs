use std::collections::HashMap;
use std::path::Path;

use crate::intcode::Program;

use super::Position;

pub struct Beam {
    program: Program,
    cache: HashMap<Position, bool>,
}

impl Beam {
    pub fn new(path: &Path) -> Self {
        let program = Program::read(path);
        Self {
            program,
            cache: HashMap::new(),
        }
    }

    pub fn contains(&mut self, position: &Position) -> bool {
        if let Some(result) = self.cache.get(position) {
            return *result;
        }

        let (x, y) = *position;

        let output = self.program.launch().run(&[x as i64, y as i64]);
        assert_eq!(output.len(), 1);

        let result = output[0] == 1;
        self.cache.insert(*position, result);

        result
    }
}
