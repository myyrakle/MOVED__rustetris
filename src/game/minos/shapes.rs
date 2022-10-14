use crate::game::tetris_cell::TetrisCell;

use super::colors::{
    I_DEFAULT_COLOR, J_DEFAULT_COLOR, L_DEFAULT_COLOR, O_DEFAULT_COLOR, S_DEFAULT_COLOR,
    T_DEFAULT_COLOR, Z_DEFAULT_COLOR,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Mino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
    #[default]
    ETC,
}

impl From<Mino> for i32 {
    fn from(value: Mino) -> Self {
        match value {
            Mino::I => 0,
            Mino::J => 1,
            Mino::L => 2,
            Mino::O => 3,
            Mino::S => 4,
            Mino::T => 5,
            Mino::Z => 6,
            Mino::ETC => 99,
        }
    }
}

impl From<i32> for Mino {
    fn from(value: i32) -> Self {
        match value {
            0 => Mino::I,
            1 => Mino::J,
            2 => Mino::L,
            3 => Mino::O,
            4 => Mino::S,
            5 => Mino::T,
            6 => Mino::Z,
            _ => Mino::ETC,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MinoShape {
    pub mino: Mino,
    pub cells: MinoShapeCells,
    pub rotation_count: usize,
}

pub type MinoShapeCells = [[TetrisCell; 4]; 4];

impl From<i32> for MinoShape {
    fn from(value: i32) -> Self {
        match value {
            0 => MinoShape::I,
            1 => MinoShape::J,
            2 => MinoShape::L,
            3 => MinoShape::O,
            4 => MinoShape::S,
            5 => MinoShape::T,
            6 => MinoShape::Z,
            99 => MinoShape::NONE,
            _ => MinoShape::NONE,
        }
    }
}

impl MinoShape {
    pub fn to_ghost(mut self) -> Self {
        for row in &mut self.cells {
            for cell in row {
                if !cell.is_empty() {
                    *cell = TetrisCell::Ghost;
                }
            }
        }

        self
    }
}

impl MinoShape {
    // □□□□
    // ■■■■
    // □□□□
    // □□□□
    pub const I: Self = Self {
        mino: Mino::I,
        rotation_count: 0,
        cells: [
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
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
        ],
    };

    // □□■□
    // ■■■□
    // □□□□
    // □□□□
    pub const L: Self = Self {
        mino: Mino::L,
        rotation_count: 0,

        cells: [
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                L_DEFAULT_COLOR,
                TetrisCell::Empty,
            ],
            [
                L_DEFAULT_COLOR,
                L_DEFAULT_COLOR,
                L_DEFAULT_COLOR,
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

    // ■□□□
    // ■■■□
    // □□□□
    // □□□□
    pub const J: MinoShape = Self {
        mino: Mino::J,
        rotation_count: 0,

        cells: [
            [
                J_DEFAULT_COLOR,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
            [
                J_DEFAULT_COLOR,
                J_DEFAULT_COLOR,
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
        rotation_count: 0,

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
        rotation_count: 0,

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
        rotation_count: 0,

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

    // □■□□
    // ■■■□
    // □□□□
    // □□□□
    pub const T: Self = Self {
        mino: Mino::T,
        rotation_count: 0,

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

    // □□□□
    // □□□□
    // □□□□
    // □□□□
    pub const NONE: Self = Self {
        mino: Mino::ETC,
        rotation_count: 0,

        cells: [
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
            [
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
                TetrisCell::Empty,
            ],
        ],
    };
}
