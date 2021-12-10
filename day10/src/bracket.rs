#[derive(Debug, PartialEq, Eq)]
pub enum BracketKind {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketKind {
    pub fn syntax_error_score(&self) -> u32 {
        match self {
            BracketKind::Round => 3,
            BracketKind::Square => 57,
            BracketKind::Curly => 1197,
            BracketKind::Angle => 25137,
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
