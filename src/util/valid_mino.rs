use crate::{
    minos::shapes::MinoShape,
    types::{point::Point, tetris_board::TetrisBoard, tetris_cell::TetrisCell},
};

// 미노 충돌여부 검증
pub fn valid_mino(board: &TetrisBoard, mino: &MinoShape, point: Point) -> bool {
    let mut mino_x = 0;

    for x in point.x..(board.column_count as i64) {
        let mut mino_y = 0;

        for y in point.y..(board.row_count as i64) {
            let y = y as usize;
            let x = x as usize;

            if let TetrisCell::Empty = board.cells[y][x] {
                // No Conflict
            } else {
                if let TetrisCell::Empty = mino[mino_y][mino_x] {
                    // No Conflict
                } else {
                    // Conflict
                    return false;
                }
            }

            mino_y += 1;
        }

        mino_x += 1;
    }

    true
}
