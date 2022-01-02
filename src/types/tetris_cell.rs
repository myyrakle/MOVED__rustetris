use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum TetrisCell {
    Empty = 1,
    Full = 2,
}

impl std::convert::TryFrom<i32> for TetrisCell {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            1 => Ok(TetrisCell::Empty),
            2 => Ok(TetrisCell::Full),
            _ => Err(()),
        }
    }
}
