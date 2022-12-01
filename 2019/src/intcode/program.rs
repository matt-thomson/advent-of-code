use std::fs;
use std::path::Path;

use super::Computer;

pub struct Program {
    code: Vec<i64>,
}

impl Program {
    pub fn read(path: &Path) -> Program {
        let code: Vec<i64> = fs::read_to_string(&path)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        Program { code }
    }

    pub fn launch(&self) -> Computer {
        Computer::new(self.code.to_vec())
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
