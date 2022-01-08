use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

use crate::types::tetris_board::TetrisBoard;
use crate::types::tetris_cell::TetrisCell;

#[wasm_bindgen]
pub fn draw_block(
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
    block_width_size: f64,
    block_height_size: f64,
    color: &str,
) {
    context.set_fill_style(&JsValue::from_str(color));
    context.fill_rect(
        block_width_size * x,
        block_height_size * y,
        block_width_size - 1.0,
        block_height_size - 1.0,
    );
    context.stroke_rect(
        block_width_size * x,
        block_height_size * y,
        block_width_size - 1.0,
        block_height_size - 1.0,
    );
}

static BOARD_HEIGHT_SIZE: f64 = 600_f64;
static BOARD_WIDTH_SIZE: f64 = 300_f64;

#[wasm_bindgen]
pub fn render(board_unfolded: Vec<i32>, board_width: u8, board_height: u8) {
    let block_width_size = BOARD_WIDTH_SIZE / board_width;
    let block_height_size = BOARD_HEIGHT_SIZE / board_height;

    let tetris_board = TetrisBoard::from_unfold(board_unfolded, board_width, board_height);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("gamebox").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // 흰색으로 세팅
    context.set_fill_style(&JsValue::from_str("#4aa8d8"));
    context.fill_rect(0.0, 0.0, 11111 as f64, 11111 as f64);
    //context.fill_rect(0.0, 0.0, board_width as f64, board_height as f64);

    for x in 0..board_width {
        let x = x as usize;

        for y in 0..board_height {
            let y = y as usize;

            if tetris_board.cells[y][x] != TetrisCell::Empty {
                context.set_stroke_style(&JsValue::from_str("black"));
                // context.strokeStyle = 'black';
                // context.fillStyle = colors[board[ y ][ x ]-1];
                // drawBlock( x, y );
            }
        }
    }

    context.set_fill_style(&JsValue::from_str("red"));
    context.set_stroke_style(&JsValue::from_str("black"));

    for y in 0..4 {
        for x in 0..4 {
            if tetris_board.cells[y][x] != TetrisCell::Empty {
                // context.fillStyle = colors[ current[ y ][ x ] - 1 ];
                // drawBlock( currentX + x, currentY + y );
            }
        }
    }

    //  const BLOCK_W = W / COLS, BLOCK_H = H / ROWS;

    // // xy 좌표에 사각형을 그림
    // function drawBlock( x, y )
    // {
    //     context.fillRect( BLOCK_W * x, BLOCK_H * y, BLOCK_W - 1 , BLOCK_H - 1 );
    //     context.strokeRect( BLOCK_W * x, BLOCK_H * y, BLOCK_W - 1 , BLOCK_H - 1 );
    // }
}
