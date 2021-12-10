#[derive(Debug, PartialEq, Eq)]
pub enum BracketKind {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketKind {
    pub fn corrupted_score(&self) -> u64 {
        match self {
            BracketKind::Round => 3,
            BracketKind::Square => 57,
            BracketKind::Curly => 1197,
            BracketKind::Angle => 25137,
        }
    }

    pub fn incomplete_score(&self) -> u64 {
        match self {
            BracketKind::Round => 1,
            BracketKind::Square => 2,
            BracketKind::Curly => 3,
            BracketKind::Angle => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BracketRole {
    Open,
    Close,
}

#[derive(Debug)]
pub struct Bracket {
    pub kind: BracketKind,
    pub role: BracketRole,
}

impl Bracket {
    pub fn from_char(c: char) -> Self {
        match c {
            '(' => Self {
                kind: BracketKind::Round,
                role: BracketRole::Open,
            },
            ')' => Self {
                kind: BracketKind::Round,
                role: BracketRole::Close,
            },
            '[' => Self {
                kind: BracketKind::Square,
                role: BracketRole::Open,
            },
            ']' => Self {
                kind: BracketKind::Square,
                role: BracketRole::Close,
            },
            '{' => Self {
                kind: BracketKind::Curly,
                role: BracketRole::Open,
            },
            '}' => Self {
                kind: BracketKind::Curly,
                role: BracketRole::Close,
            },
            '<' => Self {
                kind: BracketKind::Angle,
                role: BracketRole::Open,
            },
            '>' => Self {
                kind: BracketKind::Angle,
                role: BracketRole::Close,
            },
            _ => panic!("Unknown bracket {}", c),
        }
    }
}
