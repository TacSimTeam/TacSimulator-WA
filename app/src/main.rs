use crate::core::tac_wrap::TacWrap as Tac;
use crate::util::fetch::{fetch_and_convert_into_vector, get_dmg_path, Dmg, FetchError};
use app::consts::BASE_URL;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::InputEvent;
use yew::{function_component, html, use_state, Callback, Html};
use yew_hooks::{use_async, UseAsyncHandle};

mod core;
mod util;

#[function_component(Simulator)]
fn simulator() -> Html {
    let user_name = use_state(|| "".to_string());
    let user_name_on_input = {
        let user_name = user_name.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();
            match value {
                None => user_name.set("".to_string()),
                Some(value) => user_name.set((*user_name).clone() + &value),
            }
        })
    };
    let password = use_state(|| "".to_string());
    let password_on_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();
            match value {
                None => password.set("".to_string()),
                Some(value) => password.set((*password).clone() + &value),
            }
        })
    };
    let dmg: UseAsyncHandle<Dmg, FetchError> = use_async({
        let user_name = user_name.clone();
        let password = password.clone();
        async move {
            let path = get_dmg_path((*user_name).clone(), (*password).clone())
                .await
                .unwrap();
            fetch_and_convert_into_vector(BASE_URL.to_string() + &format!("dmg/{}", path)).await
        }
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
            <form>
                <div>
                    <label for={"user_name"}>{"ユーザー名"}</label>
                    <input type={"text"} name={"user_name"} id={"user_name"} required={true} oninput={user_name_on_input} value={(*user_name).clone()} />
                </div>
                <div>
                    <label for={"user_password"}>{"パスワード"}</label>
                    <input type={"text"} name={"user_password"} id={"user_password"} required={true} oninput={password_on_input} value={(*password).clone()}/>
                </div>

                <button {onclick} disabled={dmg.loading}>{"ログイン"}</button>
            </form>
        </>
    };
}
fn main() {
    yew::Renderer::<Simulator>::new().render();
}
