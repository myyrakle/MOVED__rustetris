use super::tetris_cell::TetrisCell;
use itertools::Itertools;

pub struct TetrisBoard {
    pub column_count: u8, //테트리스 열 개수(가로 길이)
    pub row_count: u8,    //테트리스 행 개수(세로 길이)
    pub cells: Vec<Vec<TetrisCell>>,
}

impl TetrisBoard {
    pub fn unfold(&self) -> Vec<i32> {
        self.cells
            .clone()
            .into_iter()
            .flatten()
            .map(|e| e as i32)
            .collect::<Vec<i32>>()
    }

    pub fn from_unfold(unfolded: Vec<i32>, column_count: u8, row_count: u8) -> Self {
        Self {
            column_count,
            row_count,
            cells: unfolded
                .into_iter()
                .map(|e| TetrisCell::try_from(e).unwrap())
                .chunks(column_count as usize)
                .into_iter()
                .map(|chunk| chunk.collect::<Vec<TetrisCell>>())
                .collect(),
        }
    }
}
