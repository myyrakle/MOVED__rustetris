use yew::prelude::*;

use crate::game::{event::Event, manager::GameManager};

#[function_component(GameBox)]
pub fn game_box() -> Html {
    let game_manager = GameManager::new();
    let event_sender = game_manager.get_event_sender();

    let onclick = move |_| {
        if !game_manager.on_play() {
            game_manager.start_game();
        } else {
            log::info!("이미 시작함")
        }
    };

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        match event.key_code() {
            37 => {
                event_sender.send(Event::LeftMove).unwrap();
            } // left move
            39 => {
                event_sender.send(Event::RightMove).unwrap();
            } // right move
            38 => {} // up move
            40 => {
                event_sender.send(Event::SoftDrop).unwrap();
            } // down move
            90 => {
                event_sender.send(Event::LeftRotate).unwrap();
            } // z
            88 => {
                event_sender.send(Event::RightRotate).unwrap();
            } // x
            65 => {
                event_sender.send(Event::DoubleRotate).unwrap();
            } // a
            32 => {
                event_sender.send(Event::HardDrop).unwrap();
            } // spacebar
            16 => {
                event_sender.send(Event::Hold).unwrap();
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
