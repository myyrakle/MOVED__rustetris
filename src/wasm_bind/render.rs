use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use crate::js_bind::body::body;
use crate::js_bind::request_animation_frame::request_animation_frame;
use crate::types::tetris_board::TetrisBoard;
use crate::types::tetris_cell::TetrisCell;

use super::draw::draw_block;

static BOARD_HEIGHT_SIZE: f64 = 600_f64;
static BOARD_WIDTH_SIZE: f64 = 300_f64;

#[wasm_bindgen]
pub fn render(board_unfolded: Vec<i32>, board_width: u8, board_height: u8) {
    let block_width_size = BOARD_WIDTH_SIZE / board_width as f64;
    let block_height_size = BOARD_HEIGHT_SIZE / board_height as f64;

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
    context.fill_rect(0.0, 0.0, board_width as f64, board_height as f64);
    context.set_stroke_style(&JsValue::from_str("black"));
    log::info!("{} {}", board_width, board_height);

    for x in 0..board_width {
        let x = x as usize;

        for y in 0..board_height {
            let y = y as usize;

            if tetris_board.cells[y][x] != TetrisCell::Empty {
                context.set_stroke_style(&JsValue::from_str("black"));
                // context.strokeStyle = 'black';
                // context.fillStyle = colors[board[ y ][ x ]-1];
                draw_block(
                    context.clone(),
                    x as f64,
                    y as f64,
                    block_width_size,
                    block_height_size,
                    "green",
                );
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

#[wasm_bindgen]
pub fn run_render() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        if i > 300 {
            body().set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        body().set_text_content(Some(&text));

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
