use std::sync::Arc;

use yew::prelude::*;

use crate::game::manager::GameManager;
use crate::js_bind::focus::focus;
use crate::wasm_bind::fill_rect;

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();
    let game_info = Arc::clone(&game_manager.game_info);

    let start_disabled = use_state(|| false);

    //let _start_disabled = start_disabled.clone();
    let onclick = {
        //let start_disabled = _start_disabled;

        Callback::from(move |_| {
            focus("gamebox");

            if !game_manager.on_play()
            /*Using different mutex objects "GameInfo" */
            {
                // start_disabled.set(true); // Enabling this causes problems.
                game_manager.start_game(); /*Using different mutex objects "GameInfo" */
            }
        })
    };

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            37 => {
                game_info.lock().unwrap().left_move();
            } // left move
            39 => {
                game_info.lock().unwrap().right_move();
            } // right move
            38 => {} // up move
            40 => {
                game_info.lock().unwrap().soft_drop();
            } // down move
            90 => {
                game_info.lock().unwrap().left_rotate();
            } // z
            88 => {
                game_info.lock().unwrap().right_rotate();
            } // x
            65 => {
                game_info.lock().unwrap().double_rotate();
            } // a
            32 => {
                game_info.lock().unwrap().hard_drop();
            } // spacebar
            16 => {
                game_info.lock().unwrap().hold();
            } // shift
            _ => {}
        }
    });

    html! {
        <span id="gamebox" tabindex="0" {onkeydown} onclick={Callback::from(|_| {
            log::info!("test");
            GameManager::empty_render();
        })}>
            <canvas id="hold-canvas" width="120" height="120"></canvas>
            <canvas id="game-canvas" width="300" height="600"></canvas>
            <canvas id="next-canvas" width="120" height="520"></canvas>
            <button onclick={onclick} disabled={*start_disabled}>{"Start"}</button>
        </span>
    }
}
