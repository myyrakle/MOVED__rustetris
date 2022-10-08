use std::sync::Arc;

use yew::prelude::*;

use crate::game::manager::GameManager;

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();
    let game_info = Arc::clone(&game_manager.game_info);

    let onclick = move |_| {
        if !game_manager.on_play() {
            game_manager.start_game();
        } else {
            log::info!("이미 시작함")
        }
    };

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        let mut game_info = game_info.lock().unwrap();

        match event.key_code() {
            37 => {
                game_info.left_move();
            } // left move
            39 => {
                game_info.right_move();
            } // right move
            38 => {} // up move
            40 => {
                game_info.soft_drop();
            } // down move
            90 => {
                game_info.left_rotate();
            } // z
            88 => {
                game_info.right_rotate();
            } // x
            65 => {
                game_info.double_rotate();
            } // a
            32 => {
                game_info.hard_drop();
            } // spacebar
            16 => {
                game_info.hold();
            } // shift
            _ => {}
        }
        // get the pressed key using web-sys out of the event
    });

    html! {
        <span tabindex="0" {onkeydown}>
            <canvas id="gamebox" width="300" height="600"></canvas>
            <button onclick={onclick}>{"Start"}</button>
        </span>
    }
}
