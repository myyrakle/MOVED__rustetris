use crate::{
    game::{point::Point, tetris_board::TetrisBoard, tetris_cell::TetrisCell},
    minos::shapes::MinoShape,
};

// 미노 충돌여부 검증
pub fn valid_mino(board: &TetrisBoard, mino: &MinoShape, point: Point) -> bool {
    let mino_row_count = mino.len();
    let mino_column_count = mino[0].len();

    for (mino_x, x) in (point.x..(point.x + mino_row_count as i64)).enumerate() {
        for (mino_y, y) in (point.y..(point.y + mino_column_count as i64)).enumerate() {
            let y = y as usize;
            let x = x as usize;

            let mino_is_empty = TetrisCell::Empty == mino[mino_y][mino_x];

            if mino_is_empty {
                continue;
            }

            let cell = board.cells.get(y).map(|e| e.get(x)).flatten();

            match cell {
                Some(cell) => {
                    // 비어있는 영역에 시도는 유효
                    if let TetrisCell::Empty = cell {
                        continue;
                    }
                    // 유효하지 않은 블럭 충돌
                    else {
                        return false;
                    }
                }
                None => {
                    // 미노가 존재함에도 존재하지 않는 영역에 침범 시도
                    return false;
                }
            }
        }
    }

    true
}
