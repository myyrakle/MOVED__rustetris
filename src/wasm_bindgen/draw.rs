use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn draw_block(
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
    block_width_size: f64,
    block_height_size: f64,
    color: &str,
) {
    context.set_stroke_style(&JsValue::from_str("black")); // 테두리 색상
    context.set_fill_style(&JsValue::from_str(color)); // 내부 색상
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
