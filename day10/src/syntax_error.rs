use crate::bracket::BracketKind;

#[derive(Debug)]
pub enum SyntaxError {
    Corrupted(BracketKind),
    Incomplete(Vec<BracketKind>),
}

impl SyntaxError {
    pub fn is_corrupted(&self) -> bool {
        match self {
            SyntaxError::Corrupted(_) => true,
            SyntaxError::Incomplete(_) => false,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            SyntaxError::Corrupted(kind) => kind.syntax_error_score(),
            SyntaxError::Incomplete(_) => todo!(),
        }
    }
}
