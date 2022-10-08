use yew::prelude::*;

use crate::game::manager::GameManager;

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();

    let onclick = move |_| {
        game_manager.start_game();
    };

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            37 => {
                log::info!("left")
            } // left move
            39 => {
                log::info!("right")
            } // right move
            38 => {
                log::info!("up")
            } // up move
            40 => {
                log::info!("down")
            } // down move
            90 => {
                log::info!("z")
            } // z
            88 => {
                log::info!("x")
            } // x
            65 => {
                log::info!("a")
            } // a
            32 => {
                log::info!("space")
            } // spacebar
            16 => {
                log::info!("shift")
            } // shift
            _ => {}
        }
        // get the pressed key using web-sys out of the event
    });

    html! {
        <span {onkeydown}>
            <canvas id="gamebox" width="300" height="600"></canvas>
            <button onclick={onclick}>{"Start"}</button>
        </span>
    }
}
