#![allow(clippy::let_unit_value)]
use std::sync::{Arc, Mutex};

use crate::components::gamebox;
use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Model {
    value: Arc<Mutex<i64>>,
}
use yew::{function_component, html};

#[function_component(MainComponent)]
pub fn main_component() -> Html {
    html! {
        <div>
        <gamebox::Model/>
        </div>
    }
}
