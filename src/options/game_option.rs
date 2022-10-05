pub struct GameOption {
    pub board_width: Option<u32>,
    pub board_height: Option<u32>,
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
            board_width: None,
            board_height: None,
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

    pub fn board_width(mut self, board_width: u32) -> Self {
        self.board_width = Some(board_width);
        self
    }

    pub fn board_height(mut self, board_height: u32) -> Self {
        self.board_height = Some(board_height);
        self
    }

    pub fn bag_mode(mut self, bag_mode: bool) -> Self {
        self.bag_mode = Some(bag_mode);
        self
    }
}
