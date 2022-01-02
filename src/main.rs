pub mod components;
pub mod functions;
pub mod minos;
pub mod options;
pub mod types;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::main::Model>();
}
