use crate::bracket::BracketKind;

#[derive(Debug)]
pub enum SyntaxError<'a> {
    Corrupted(&'a BracketKind),
    Incomplete(Vec<&'a BracketKind>),
}

impl<'a> SyntaxError<'a> {
    pub fn is_corrupted(&self) -> bool {
        match self {
            SyntaxError::Corrupted(_) => true,
            SyntaxError::Incomplete(_) => false,
        }
    }

    pub fn score(&self) -> u64 {
        match self {
            SyntaxError::Corrupted(kind) => kind.corrupted_score(),
            SyntaxError::Incomplete(kinds) => kinds
                .iter()
                .fold(0, |acc, kind| acc * 5 + kind.incomplete_score()),
        }
    }
}
