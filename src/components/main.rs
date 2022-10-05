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

struct Foo {
    foo: Arc<Mutex<i64>>,
}

impl yew::scheduler::Runnable for Foo {
    fn run(self: Box<Self>) {
        let mut foo = self.foo.lock().unwrap();

        *foo += 1;
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                // the value has changed so we need to
                // re-render for it to appear on the page

                yew::scheduler::push(Box::new(Foo {
                    foo: Arc::clone(&self.value),
                }));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        html! {
            <div>
                <gamebox::Model/>
                <h1>{ self.value.lock().unwrap() }</h1>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "Add One" }</button>
            </div>
        }
    }
}
