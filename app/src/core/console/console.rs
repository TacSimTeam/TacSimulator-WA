use crate::core::console::button::ButtonArg;
use crate::core::console::components::Components;
use crate::core::console::console_state::ConsoleState;
use crate::core::tac::Tac;
use crate::util::image::load_image;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::functional::use_context;
use yew::{function_component, html, Component, Context, Html, MouseEvent, NodeRef, Properties};

#[function_component(ConsoleWrap)]
pub fn console_wrap(props: &Props) -> Html {
    let tac = use_context::<Rc<RefCell<Tac>>>().unwrap();
    return html! {
        <Console state={Rc::clone(&props.state)} component={Rc::clone(&props.component)} tac={tac}/>
    };
}

pub struct Console {
    ctx: NodeRef,
    pub state: Rc<RefCell<ConsoleState>>,
    pub components: Rc<RefCell<Components>>,
    image: Option<HtmlImageElement>,
    tac_ref: Rc<RefCell<Tac>>,
}

pub enum Msg {
    FetchOk(HtmlImageElement),
    FetchFail,
    Clicked((i32, i32)),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub state: Rc<RefCell<ConsoleState>>,
    pub component: Rc<RefCell<Components>>,
}

#[derive(PartialEq, Properties)]
pub struct ConsoleProps {
    pub state: Rc<RefCell<ConsoleState>>,
    pub component: Rc<RefCell<Components>>,
    pub tac: Rc<RefCell<Tac>>,
}

impl Console {
    pub fn on_click(&mut self, x: i32, y: i32) -> bool {
        let val = self.components.borrow().switch_state();
        let mut is_click = false;
        for (index, b) in self.components.borrow().buttons.iter().enumerate() {
            if b.is_btn_clicked(x as f64, y as f64) {
                if index == 2 || index == 3 || index == 4 {
                    b.on_click(ButtonArg::Tac(&mut self.tac_ref.borrow_mut()), 0);
                } else {
                    b.on_click(ButtonArg::Console(&mut self.state.borrow_mut()), val);
                }
                is_click = true;
            }
        }
        for s in self.components.borrow_mut().switches.iter_mut() {
            if s.is_switch_clicked(x as f64, y as f64) {
                s.toggle_state();
                is_click = true;
            }
        }
        self.update();
        return is_click;
    }

    fn update(&mut self) {
        if self.state.borrow().run_flag {
            self.draw_all();
            return;
        }
        self.update_rot_sw();
        self.update_led();
        self.draw_all();
    }

    fn draw_all(&self) {
        if self.image.is_some() {
            let ctx: HtmlCanvasElement = self.ctx.cast().unwrap();
            let canvas_ctx = ctx
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            canvas_ctx
                .draw_image_with_html_image_element(&self.image.as_ref().unwrap(), 0.0, 0.0)
                .unwrap();
        }
        self.components.borrow().draw();
    }

    fn update_rot_sw(&mut self) {
        for i in 0..6 {
            if i == self.state.borrow().rot_current % 6 {
                self.components.borrow_mut().register_led[i as usize].set_state(true);
            } else {
                self.components.borrow_mut().register_led[i as usize].set_state(false);
            }
        }

        for i in 0..3 {
            if i == self.state.borrow().rot_current / 6 {
                self.components.borrow_mut().flag_led[i as usize].set_state(true);
            } else {
                self.components.borrow_mut().flag_led[i as usize].set_state(false);
            }
        }
    }

    fn update_led(&mut self) {
        let mut val = self.read_reg();
        if self.state.borrow().rot_current == 17 {
            val = val & 0xfffe;
        }
        self.set_led_lamps(val);
    }

    fn read_reg(&self) -> u16 {
        match self.state.borrow().rot_current {
            14u8 => self.state.borrow().psw.borrow().get_pc(),
            15u8 => self.state.borrow().psw.borrow().get_flag().into(),
            16u8 => self.read_mem_data(),
            17u8 => self.state.borrow().mem_addr,
            _ => self
                .state
                .borrow()
                .register
                .borrow()
                .read(self.state.borrow().rot_current),
        }
    }

    fn read_mem_data(&self) -> u16 {
        let addr = self.state.borrow().mem_addr;
        let binding = self.state.borrow();
        let memory = binding.memory.borrow();
        (memory.read8(addr) as u16) << 8 | memory.read8(addr + 1) as u16
    }

    pub fn set_led_lamps(&mut self, val: u16) {
        self.components.borrow_mut().set_led_lamps(val);
    }
}

impl Component for Console {
    type Message = Msg;
    type Properties = ConsoleProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            if let Ok(image) = load_image("static/TaC.png").await {
                Msg::FetchOk(image)
            } else {
                Msg::FetchFail
            }
        });
        let ConsoleProps {
            state,
            component,
            tac,
        } = ctx.props();
        Self {
            ctx: NodeRef::default(),
            state: Rc::clone(state),
            components: Rc::clone(component),
            image: None,
            tac_ref: Rc::clone(tac),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                self.components.borrow_mut().init(canvas_ctx);
                self.components.borrow().draw();
                self.update();
                self.tac_ref.borrow_mut().init();
                true
            }
            Msg::Clicked((x, y)) => {
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
