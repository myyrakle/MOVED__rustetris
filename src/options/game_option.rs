pub struct GameOption {
    pub column_count: Option<u8>,
    pub row_count: Option<u8>,
    pub bag_mode: Option<bool>,
}

impl GameOption {
    pub fn build() -> Self {
        Self {
            column_count: None,
            row_count: None,
            bag_mode: None,
        }
    }

    pub fn column_count(mut self, column_count: u8) -> Self {
        self.column_count = Some(column_count);
        self
    }

    pub fn row_count(mut self, row_count: u8) -> Self {
        self.row_count = Some(row_count);
        self
    }

    pub fn bag_mode(mut self, bag_mode: bool) -> Self {
        self.bag_mode = Some(bag_mode);
        self
    }
}
