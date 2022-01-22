use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum TetrisCell {
    Empty = "white",
    Red = "red",
    Green = "green",
    Blue = "blue",
    Purple = "purple",
    Cyan = "cyan",
    Orange = "orange",
    Yellow = "yellow",
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
            _ => Err(()),
        }
    }
}
