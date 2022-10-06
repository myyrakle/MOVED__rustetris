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
}
