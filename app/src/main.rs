use crate::core::tac_wrap::TacWrap as Tac;
use crate::util::fetch::{
    fetch_and_convert_into_vector, get_dmg_path, update_dmg, Dmg, FetchError,
};
use app::consts::BASE_URL;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::InputEvent;
use yew::html::onclick::Event;
use yew::{function_component, html, use_state, Callback, Html};
use yew_hooks::{use_async, UseAsyncHandle};

mod core;
mod util;

#[function_component(Simulator)]
fn simulator() -> Html {
    let mut dmg_data = Rc::new(RefCell::new(vec![]));
    let dmg_name = use_state(|| String::new());
    let is_login = use_state(|| false);
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
        let is_login = is_login.clone();
        let dmg_name = dmg_name.clone();
        async move {
            let path = get_dmg_path((*user_name).clone(), (*password).clone())
                .await
                .unwrap();
            dmg_name.set(path.clone());
            is_login.set(true);
            fetch_and_convert_into_vector(BASE_URL.to_string() + &format!("dmg/{}", path)).await
        }
    });
    let get_dmg = {
        let dmg = dmg.clone();
        Callback::from(move |_| {
            dmg.run();
        })
    };
    let save = use_async({
        let clone = Rc::clone(&dmg_data);
        let user_name = (*user_name).clone();
        async move {
            let data = clone.borrow().clone();
            update_dmg(user_name, data).await
        }
    });
    let save_btn_onclick: Callback<Event> = {
        let save = save.clone();
        Callback::from(move |_| {
            save.run();
        })
    };
    return html! {
        <main class={"layout"}>

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
                    dmg_data.borrow_mut().clear();
                    for (i, data) in data.get_data().iter().enumerate() {
                        dmg_data.borrow_mut().push(*data);
                    }
                    html! {
                        <>
                            <Tac dmg={Rc::clone(&dmg_data)}/>
                        </>
                    }
                } else {
                    html! {
                        <></>
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
        {
            if !(*is_login).clone() {
                html!{
                    <form class={"login_form"}>
                        <div class={"form_el_wrap"}>
                            <label for={"user_name"}>{"ユーザー名"}</label>
                            <input type={"text"} name={"user_name"} id={"user_name"} required={true} oninput={user_name_on_input} value={(*user_name).clone()} placeholder={"UserName"} />
                        </div>
                        <div class={"form_el_wrap"}>
                            <label for={"user_password"}>{"パスワード"}</label>
                            <input type={"password"} name={"user_password"} id={"user_password"} required={true} oninput={password_on_input} value={(*password).clone()} placeholder={"PASSWORD"}/>
                        </div>

                        <button onclick={get_dmg} disabled={dmg.loading} class={"login_btn"}>{"ログイン"}</button>
                    </form>
                }
            } else {
                html! {
                    <div class={"save_btn_area"}>
                        <button onclick={save_btn_onclick} class={"save_btn"}>{"保存"}</button>
                    </div>
                }
            }
        }
        </main>
    };
}
fn main() {
    yew::Renderer::<Simulator>::new().render();
}
