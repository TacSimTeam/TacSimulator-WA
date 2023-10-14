use crate::core::console::components::Components;
use crate::core::console::console_state::ConsoleState;
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::memory::memory::Memory;
use crate::util::image::load_image;
use std::cell::RefCell;
use std::panic::catch_unwind;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::{html, Component, Context, Html, MouseEvent, NodeRef, Properties};

pub struct Console {
    ctx: NodeRef,
    state: ConsoleState,
    components: Components,
    image: Option<HtmlImageElement>
}

pub enum Msg {
    FetchOk(HtmlImageElement),
    FetchFail,
    Clicked((i32, i32)),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub memory: Rc<RefCell<Memory>>,
    pub psw: Rc<RefCell<Psw>>,
    pub register: Rc<RefCell<Register>>,
}

impl Console {
    pub fn on_click(&mut self, x: i32, y: i32) -> bool {
        for b in self.components.buttons.iter() {
            if b.is_btn_clicked(x as f64, y as f64) {
                b.on_click(&mut self.state);
                return true;
            }
        }
        for s in self.components.switches.iter_mut() {
            if s.is_switch_clicked(x as f64, y as f64) {
                s.toggle_state();
                return true;
            }
        }
        false
    }

    fn update(&mut self) {
        if self.state.run_flag {
            self.draw_all();
            return;
        }
        self.update_rot_sw();
        // self.update_led();
        self.draw_all();
    }

    fn draw_all(&self) {
        if self.image.is_some() {
            let ctx: HtmlCanvasElement = self.ctx.cast().unwrap();
            let canvas_ctx = ctx.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
            canvas_ctx.draw_image_with_html_image_element(&self.image.as_ref().unwrap(), 0.0, 0.0).unwrap();
        }
        self.components.draw();
    }

    fn update_rot_sw(&mut self) {
        for i in 0..6 {
            if i == self.state.rot_current % 6 {
                self.components.register_led[i as usize].set_state(true);
            } else {
                self.components.register_led[i as usize].set_state(false);
            }
        }

        for i in 0..3 {
            if i == self.state.rot_current / 6 {
                self.components.flag_led[i as usize].set_state(true);
            } else {
                self.components.flag_led[i as usize].set_state(false);
            }
        }
    }

    fn update_led(&mut self) {
        todo!()
    }
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
        let Props {
            memory,
            psw,
            register,
        } = ctx.props();
        let memory = Rc::clone(memory);
        let psw = Rc::clone(psw);
        let register = Rc::clone(register);
        Self {
            ctx: NodeRef::default(),
            state: ConsoleState::new(memory, psw, register),
            components: Components::new(),
            image: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchOk(image) => {
                self.image = Some(image.clone());
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
                // TODO 各コンポーネントのonclickに座標を渡してまわる
                gloo::console::log!(format!("clicked {} {}", x, y));
                if self.on_click(x, y) {
                    self.update();
                    return true;
                }
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
