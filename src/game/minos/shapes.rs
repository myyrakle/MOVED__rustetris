use crate::game::tetris_cell::TetrisCell;

use super::colors::{
    I_DEFAULT_COLOR, J_DEFAULT_COLOR, L_DEFAULT_COLOR, O_DEFAULT_COLOR, S_DEFAULT_COLOR,
    T_DEFAULT_COLOR, Z_DEFAULT_COLOR,
};

#[derive(Debug, Clone, Copy)]
pub enum Mino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
    ETC,
}

#[derive(Debug, Clone, Copy)]
pub struct MinoShape {
    pub mino: Mino,
    pub cells: MinoShapeCells,
}

pub type MinoShapeCells = [[TetrisCell; 4]; 4];

impl MinoShape {
    // ■■■■
    // □□□□
    // □□□□
    // □□□□
    pub const I: Self = Self {
        mino: Mino::I,
        cells: [
            [
                I_DEFAULT_COLOR,
                I_DEFAULT_COLOR,
                I_DEFAULT_COLOR,
                I_DEFAULT_COLOR,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };

    // ■■■□
    // ■□□□
    // □□□□
    // □□□□
    pub const L: Self = Self {
        mino: Mino::L,
        cells: [
            [
                L_DEFAULT_COLOR,
                L_DEFAULT_COLOR,
                L_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                L_DEFAULT_COLOR,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };

    // ■■■□
    // □□■□
    // □□□□
    // □□□□
    pub const J: MinoShape = Self {
        mino: Mino::J,
        cells: [
            [
                J_DEFAULT_COLOR,
                J_DEFAULT_COLOR,
                J_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                J_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };

    // □■■□
    // □■■□
    // □□□□
    // □□□□
    pub const O: Self = Self {
        mino: Mino::O,
        cells: [
            [
                TetrisCell::Empty,
                O_DEFAULT_COLOR,
                O_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                O_DEFAULT_COLOR,
                O_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };

    // □■■□
    // ■■□□
    // □□□□
    // □□□□
    pub const S: Self = Self {
        mino: Mino::S,
        cells: [
            [
                TetrisCell::Empty,
                S_DEFAULT_COLOR,
                S_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                S_DEFAULT_COLOR,
                S_DEFAULT_COLOR,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };

    // ■■□□
    // □■■□
    // □□□□
    // □□□□
    pub const Z: Self = Self {
        mino: Mino::Z,
        cells: [
            [
                Z_DEFAULT_COLOR,
                Z_DEFAULT_COLOR,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                Z_DEFAULT_COLOR,
                Z_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };

    pub const T: Self = Self {
        mino: Mino::T,
        cells: [
            [
                TetrisCell::Empty,
                T_DEFAULT_COLOR,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                T_DEFAULT_COLOR,
                T_DEFAULT_COLOR,
                T_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };
}
