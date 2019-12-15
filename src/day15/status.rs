#[derive(Debug)]
pub enum Status {
    HitWall,
    Moved,
    FoundOxygen,
}

impl Status {
    pub fn from(value: i64) -> Self {
        match value {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::FoundOxygen,
            x => panic!("unknown status {}", x),
        }
    }
}
