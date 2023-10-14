#![feature(unboxed_closures)]

use crate::core::console::console::{Console, Props};
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::memory::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;
use yew::{function_component, html, Html};

mod core;
mod ui;
mod util;

type ConsoleProps = Props;
#[function_component]
fn App() -> Html {
    // TODO 最終的にはTaCコンポーネントのの中でこれ作って配置してあげる
    let memory = Rc::new(RefCell::new(Memory::new()));
    let psw = Rc::new(RefCell::new(Psw::new()));
    let register = Rc::new(RefCell::new(Register::new(Rc::clone(&psw))));
    return html! {
        <Console memory={memory} psw={psw} register={register} />
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
