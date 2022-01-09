use crate::types::tetris_cell::TetrisCell;

use super::colors::{
    MinoColor, I_DEFAULT_COLOR, J_DEFAULT_COLOR, L_DEFAULT_COLOR, O_DEFAULT_COLOR, S_DEFAULT_COLOR,
    T_DEFAULT_COLOR, Z_DEFAULT_COLOR,
};

pub type MinoShape = [[TetrisCell; 4]; 4];

pub type MinoType = (MinoShape, MinoColor);

pub const I: MinoType = (
    [
        [
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Full,
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
    I_DEFAULT_COLOR,
);

pub const L: MinoType = (
    [
        [
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Full,
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
    L_DEFAULT_COLOR,
);

pub const J: MinoType = (
    [
        [
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Empty,
            TetrisCell::Empty,
            TetrisCell::Full,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Empty,
            TetrisCell::Empty,
            TetrisCell::Full,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Empty,
            TetrisCell::Empty,
            TetrisCell::Full,
            TetrisCell::Empty,
        ],
    ],
    J_DEFAULT_COLOR,
);

pub const O: MinoType = (
    [
        [
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Empty,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Full,
            TetrisCell::Full,
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
    O_DEFAULT_COLOR,
);

pub const S: MinoType = (
    [
        [
            TetrisCell::Empty,
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Full,
            TetrisCell::Full,
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
    S_DEFAULT_COLOR,
);

pub const Z: MinoType = (
    [
        [
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Empty,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Empty,
            TetrisCell::Full,
            TetrisCell::Full,
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
    Z_DEFAULT_COLOR,
);

pub const T: MinoType = (
    [
        [
            TetrisCell::Empty,
            TetrisCell::Full,
            TetrisCell::Empty,
            TetrisCell::Empty,
        ],
        [
            TetrisCell::Full,
            TetrisCell::Full,
            TetrisCell::Full,
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
    T_DEFAULT_COLOR,
);
