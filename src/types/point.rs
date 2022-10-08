#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn start_point(column_count: u8) -> Self {
        let y = 0;
        let x = column_count as i64 / 2 - 1;

        Self { x, y }
    }

    pub fn set_x(mut self, x: i64) -> Self {
        self.x = x;
        self
    }

    pub fn set_y(mut self, y: i64) -> Self {
        self.y = y;
        self
    }
}
