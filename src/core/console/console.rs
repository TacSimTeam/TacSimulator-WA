use std::sync::{Arc, Mutex};
use crate::core::cpu::psw::Psw;
use crate::core::memory::memory::Memory;
use crate::util::image::load_image;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::{html, Component, Context, Html, MouseEvent, NodeRef, Properties};
use crate::core::console::components::Components;
use crate::core::console::console_state::ConsoleState;
use crate::core::cpu::register::Register;

pub struct Console {
    ctx: NodeRef,
    rot_current: u8,
    mem_addr: u16,
    state: ConsoleState,
    components: Components
}

pub enum Msg {
    FetchOk(HtmlImageElement),
    FetchFail,
    Clicked((i32, i32)),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub memory: Arc<Mutex<Memory>>,
    pub psw: Arc<Mutex<Psw>>,
    pub register: Arc<Mutex<Register>>
}

impl Component for Console {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            if let Ok(image) = load_image("static/TaC.png").await {
                Msg::FetchOk(image)
            } else {
                Msg::FetchFail
            }
        });
        let Props {memory, psw, register} = ctx.props();
        let memory = Arc::clone(memory);
        let psw = Arc::clone(psw);
        let register = Arc::clone(register);
        Self {
            ctx: NodeRef::default(),
            rot_current: 0u8,
            mem_addr: 0u16,
            state: ConsoleState::new(memory, psw, register),
            components: Components::new()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchOk(image) => {
                let canvas: HtmlCanvasElement = self.ctx.cast().unwrap();
                let canvas_ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                canvas_ctx
                    .draw_image_with_html_image_element(&image, 0.0, 0.0)
                    .unwrap();
                self.components.init(canvas_ctx);
                self.components.draw();
                true
            }
            Msg::Clicked((x, y)) => {
                // TODO ここから座標を元にどれがクリックされたか決める
                gloo::console::log!(format!("clicked {} {}", x, y));
                false
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <canvas
                id="canvas"
                ref={self.ctx.clone()}
                width="430px"
                height="390px"
                onclick={ctx.link().callback(|e: MouseEvent| Msg::Clicked((e.client_x(), e.client_y())))}
            />
        };
    }
}
