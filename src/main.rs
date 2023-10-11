#![feature(unboxed_closures)]

use std::sync::{Arc, Mutex};
use crate::core::console::console::{Console, Props};
use yew::{function_component, html, Html};
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::memory::memory::Memory;

mod core;
mod ui;
mod util;

type ConsoleProps = Props;
#[function_component]
fn App() -> Html {
    let memory = Arc::new(Mutex::new(Memory::new()));
    let psw = Arc::new(Mutex::new(Psw::new()));
    let register = Arc::new(Mutex::new(Register::new(Arc::clone(&psw))));
    let console_props = Props {memory, psw, register};
    return html! {
        <Console {..console_props} />
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
