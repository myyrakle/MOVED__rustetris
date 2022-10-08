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
        let x = position.x;
        let y = position.y;

        let mut mino_x = 0;

        let mino_row_count = mino.len();
        let mino_column_count = mino[0].len();

        log::error!("{} {}", x, mino_column_count);
        for x in x..(x + mino_column_count as i64) {
            let mut mino_y = 0;

            for y in y..(y + mino_row_count as i64) {
                let y = y as usize;
                let x = x as usize;

                let cell = self.cells.get(y).map(|e| e.get(x)).flatten();

                match cell {
                    Some(cell) => {
                        if let TetrisCell::Empty = cell {
                            // No Conflict
                            self.cells[y][x] = mino[mino_y][mino_x];
                        } else if let TetrisCell::Empty = mino[mino_y][mino_x] {
                            // No Conflict
                        } else {
                            // Conflict
                            panic!("block conflict");
                        }
                    }
                    None => {}
                }

                mino_y += 1;
            }

            mino_x += 1;
        }
    }
}
