use crate::types::tetris_cell::TetrisCell;

use super::colors::{
    I_DEFAULT_COLOR, J_DEFAULT_COLOR, L_DEFAULT_COLOR, O_DEFAULT_COLOR, S_DEFAULT_COLOR,
    T_DEFAULT_COLOR, Z_DEFAULT_COLOR,
};

pub type MinoShape = [[TetrisCell; 4]; 4];

pub const I: MinoShape = [
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
];

pub const L: MinoShape = [
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
];

pub const J: MinoShape = [
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
];

pub const O: MinoShape = [
    [
        O_DEFAULT_COLOR,
        O_DEFAULT_COLOR,
        TetrisCell::Empty,
        TetrisCell::Empty,
    ],
    [
        O_DEFAULT_COLOR,
        O_DEFAULT_COLOR,
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
];

pub const S: MinoShape = [
    [
        TetrisCell::Empty,
        S_DEFAULT_COLOR,
        TetrisCell::Purple,
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
];

pub const Z: MinoShape = [
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
];

pub const T: MinoShape = [
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
];
