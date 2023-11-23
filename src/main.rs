use crate::core::tac_wrap::TacWrap as Tac;
use gloo_net::http::Request;
use std::cell::RefCell;
use std::rc::Rc;
use yew::{function_component, html, use_effect_with, use_state, Html};

mod core;
mod util;

async fn fetch_dmg() -> Vec<u8> {
    // let res: Vec<u8> = Request::get("http://localhost:3000/assets/Test-8queen-8m.dmg")
    let res: Vec<u8> = Request::get("http://localhost:3000/assets/TacOS-vm-8m.dmg")
        .send()
        .await
        .unwrap()
        .binary()
        .await
        .unwrap();
    return res;
}
#[function_component]
fn App() -> Html {
    let dmg = use_state(|| vec![]);
    {
        let dmg = dmg.clone();
        use_effect_with((), move |_| {
            let dmg = dmg.clone();
            wasm_bindgen_futures::spawn_local(async move {
                dmg.set(fetch_dmg().await);
            });
            || ()
        });
    }
    return html! {
        <Tac dmg={Rc::new(RefCell::new((*dmg).clone()))}/>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
