#![allow(clippy::explicit_counter_loop)]

use crate::minos::shapes::MinoShape;

use super::{point::Point, tetris_cell::TetrisCell};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct TetrisBoard {
    pub column_count: u8, //테트리스 열 개수(가로 길이)
    pub row_count: u8,    //테트리스 행 개수(세로 길이)
    pub board_width: u32,
    pub board_height: u32,
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

    pub fn from_unfold(
        unfolded: Vec<i32>,
        board_width: u32,
        board_height: u32,
        column_count: u8,
        row_count: u8,
    ) -> Self {
        Self {
            column_count,
            row_count,
            board_width,
            board_height,
            cells: unfolded
                .into_iter()
                .map(|e| TetrisCell::try_from(e).unwrap())
                .chunks(column_count as usize)
                .into_iter()
                .map(|chunk| chunk.collect::<Vec<TetrisCell>>())
                .collect(),
        }
    }

    pub fn write_current_mino(&mut self, mino: MinoShape, position: Point) {
        let x = position.x as usize;
        let y = position.y as usize;

        let mut mino_x = 0;

        let mino_row_count = mino.len();
        let mino_column_count = mino[0].len();

        for x in x..(x + mino_column_count) {
            let mut mino_y = 0;

            for y in y..(y + mino_row_count) {
                let y = y as usize;
                let x = x as usize;

                if let TetrisCell::Empty = self.cells[y][x] {
                    // No Conflict
                    self.cells[y][x] = mino[mino_y][mino_x];
                } else if let TetrisCell::Empty = mino[mino_y][mino_x] {
                    // No Conflict
                } else {
                    // Conflict
                    panic!("block conflict");
                }

                mino_y += 1;
            }

            mino_x += 1;
        }
    }
}
