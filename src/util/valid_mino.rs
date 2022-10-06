use crate::{
    minos::shapes::MinoShape,
    types::{point::Point, tetris_board::TetrisBoard, tetris_cell::TetrisCell},
};

// 미노 충돌여부 검증
pub fn valid_mino(board: &TetrisBoard, mino: &MinoShape, point: Point) -> bool {
    for (mino_x, x) in (point.x..(board.column_count as i64)).enumerate() {
        for (mino_y, y) in (point.y..(board.row_count as i64)).enumerate() {
            let y = y as usize;
            let x = x as usize;

            if let TetrisCell::Empty = board.cells[y][x] {
                // No Conflict
            } else if let TetrisCell::Empty = mino[mino_y][mino_x] {
                // No Conflict
            } else {
                // Conflict
                return false;
            }
        }
    }

    true
}
