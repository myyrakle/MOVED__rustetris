#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn start_point(column_count: u8) -> Self {
        let y = 0;
        let x = column_count as u32 / 2 - 2;

        Self { x, y }
    }

    pub fn add_x(mut self, x: u32) -> Self {
        self.x = self.x + x;
        self
    }

    pub fn add_y(mut self, y: u32) -> Self {
        self.y = self.y + y;
        self
    }

    pub fn move_xy(mut self, x: u32, y: u32) -> Self {
        self.x = self.x + x;
        self.y = self.y + y;
        self
    }
}
