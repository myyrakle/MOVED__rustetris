pub mod components;
pub mod functions;
pub mod js_bind;
pub mod minos;
pub mod options;
pub mod types;
pub mod wasm_bind;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::main::Model>();
}
