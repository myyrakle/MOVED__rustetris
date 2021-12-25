pub mod components;
pub mod functions;
pub mod minos;
pub mod options;
pub mod types;

fn main() {
    yew::start_app::<components::main::Model>();
}
