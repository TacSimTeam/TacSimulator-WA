use crate::core::tac_wrap::TacWrap as Tac;
use crate::util::fetch::fetch_and_convert_into_vector;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use yew::{function_component, html, Callback, Html};
use yew_hooks::use_async;

mod core;
mod util;

#[function_component]
fn App() -> Html {
    let dmg = use_async(async move {
        fetch_and_convert_into_vector(String::from("http://localhost:3000/TacOS-vm-8m-debug.dmg"))
            .await
    });
    let onclick = {
        let dmg = dmg.clone();
        Callback::from(move |_| {
            dmg.run();
        })
    };
    return html! {
        <>
            {
                if dmg.loading {
                    html! {
                        <></>
                    }
                } else {
                    html! {
                        <></>
                    }
                }
            }
            {
                if let Some(data) = &dmg.data {
                    html! {
                        <>
                            <Tac dmg={Rc::new(RefCell::new(data.get_data()))}/>
                        </>
                    }
                } else {
                    html! {
                        <p>{"Loading ..."}</p>
                    }
                }
            }
            {
                if let Some(error) = &dmg.error {
                    html! {
                        <p>{format!("{:?}", error)}</p>
                    }
                } else {
                    html! {
                        <></>
                    }
                }
            }
            <button {onclick} disabled={dmg.loading}>{"ログイン"}</button>
        </>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
