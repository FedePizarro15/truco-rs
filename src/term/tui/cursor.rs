#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
    pub fn new(x: u16, y: u16) -> Cursor {
        Cursor { x, y }
    }
}