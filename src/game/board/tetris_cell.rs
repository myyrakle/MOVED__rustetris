use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrisCell {
    Empty = "white",
    Ghost = "d3d3d3",
    Red = "red",
    Green = "green",
    Blue = "blue",
    Purple = "purple",
    Cyan = "cyan",
    Orange = "orange",
    Yellow = "yellow",
}

impl Default for TetrisCell {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::convert::TryFrom<i32> for TetrisCell {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(TetrisCell::Empty),
            1 => Ok(TetrisCell::Red),
            2 => Ok(TetrisCell::Green),
            3 => Ok(TetrisCell::Blue),
            4 => Ok(TetrisCell::Purple),
            5 => Ok(TetrisCell::Cyan),
            6 => Ok(TetrisCell::Orange),
            7 => Ok(TetrisCell::Yellow),
            8 => Ok(TetrisCell::Ghost),
            _ => Err(()),
        }
    }
}

impl TetrisCell {
    pub fn is_empty(&self) -> bool {
        self == &Self::Empty
    }

    pub fn to_color(&self) -> &str {
        match self {
            Self::Empty => "white",
            Self::Red => "red",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::Cyan => "cyan",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Ghost => "d3d3d3",
            _ => "while",
        }
    }
}
