use crate::core::console::console::Console;
use std::default::Default;
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Node};
use yew::{Html, NodeRef};
use crate::core::console::console_state::ConsoleState;

const BUTTON_WIDTH: f64 = 30.0;
const BUTTON_HEIGHT: f64 = 36.0;
const BUTTON_RADIUS: f64 = 10.0;
const BUTTON_COLOR_EDGE: &str = "#252525";
const BUTTON_COLOR_BODY: &str = "#f8f8f8";

pub enum ButtonName {
    LeftAllowBtn,
    RightAllowBtn,
    ResetBtn,
    RunBtn,
    StopBtn,
    SetaBtn,
    IncaBtn,
    DecaBtn,
    WriteBtn,
}

pub type ConsoleEventType = fn(&mut ConsoleState);

pub struct Button {
    pos_x: f64,
    pos_y: f64,
    ctx: CanvasRenderingContext2d,
    event: Option<ConsoleEventType>,
}

fn default_func() {}

impl Button {
    pub fn new(pos_x: f64, pos_y: f64, ctx: CanvasRenderingContext2d) -> Self {
        Self {
            pos_x,
            pos_y,
            ctx,
            event: None,
        }
    }

    pub fn draw(&self) {
        self.ctx.begin_path();
        self.ctx
            .arc(
                self.pos_x + BUTTON_WIDTH / 2.0,
                self.pos_y + BUTTON_HEIGHT / 2.0,
                BUTTON_RADIUS + 2.0,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        self.ctx
            .set_fill_style(&JsValue::from_str(BUTTON_COLOR_EDGE));
        self.ctx.fill();

        self.ctx.begin_path();
        self.ctx
            .arc(
                self.pos_x + BUTTON_WIDTH / 2.0,
                self.pos_y + BUTTON_HEIGHT / 2.0,
                BUTTON_RADIUS,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        self.ctx
            .set_fill_style(&JsValue::from_str(BUTTON_COLOR_BODY));
        self.ctx.fill();
    }

    pub fn is_btn_clicked(&self, click_x: f64, click_y: f64) -> bool {
        return self.pos_x <= click_x
            && click_x <= self.pos_x + BUTTON_WIDTH
            && self.pos_y <= click_y
            && click_y <= self.pos_y + BUTTON_HEIGHT;
    }

    pub fn set_event(&mut self, f: ConsoleEventType) {
        self.event = Some(f);
    }

    pub fn on_click(&mut self, click_x: f64, click_y: f64, state: &mut ConsoleState) {
        if self.is_btn_clicked(click_x, click_y) && self.event.is_some() {
            (self.event.unwrap())(state);
        }
    }
}
