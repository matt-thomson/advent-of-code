#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    dx: i32,
    dy: i32,
}

impl Position {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { x: 0, y: 0, dx, dy }
    }

    pub fn step(&self) -> Self {
        Self {
            x: self.x + self.dx,
            y: self.y + self.dy,
            dx: self.dx - self.dx.signum(),
            dy: self.dy - 1,
        }
    }
}
