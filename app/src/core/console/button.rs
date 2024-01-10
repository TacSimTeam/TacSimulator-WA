use crate::core::console::console_state::ConsoleState;
use crate::core::tac::Tac;
use std::ops::{Index, IndexMut};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

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

impl Index<ButtonName> for Vec<Button> {
    type Output = Button;

    fn index(&self, index: ButtonName) -> &Self::Output {
        match index {
            ButtonName::LeftAllowBtn => &self[0],
            ButtonName::RightAllowBtn => &self[1],
            ButtonName::ResetBtn => &self[2],
            ButtonName::RunBtn => &self[3],
            ButtonName::StopBtn => &self[4],
            ButtonName::SetaBtn => &self[5],
            ButtonName::IncaBtn => &self[6],
            ButtonName::DecaBtn => &self[7],
            ButtonName::WriteBtn => &self[8],
        }
    }
}

impl IndexMut<ButtonName> for Vec<Button> {
    fn index_mut(&mut self, index: ButtonName) -> &mut Self::Output {
        match index {
            ButtonName::LeftAllowBtn => &mut self[0],
            ButtonName::RightAllowBtn => &mut self[1],
            ButtonName::ResetBtn => &mut self[2],
            ButtonName::RunBtn => &mut self[3],
            ButtonName::StopBtn => &mut self[4],
            ButtonName::SetaBtn => &mut self[5],
            ButtonName::IncaBtn => &mut self[6],
            ButtonName::DecaBtn => &mut self[7],
            ButtonName::WriteBtn => &mut self[8],
        }
    }
}

pub type ConsoleEventType = fn(&mut ConsoleState, val: u8);
pub type TacEventType = fn(&mut Tac);

#[derive(PartialEq, Clone, Debug)]
pub enum ButtonEventType {
    ConsoleEvent(ConsoleEventType),
    TacEvent(TacEventType),
}

pub enum ButtonArg<'a> {
    Console(&'a mut ConsoleState),
    Tac(&'a mut Tac),
}

#[derive(PartialEq, Clone)]
pub struct Button {
    pos_x: f64,
    pos_y: f64,
    ctx: CanvasRenderingContext2d,
    pub event: Option<ButtonEventType>,
}

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

    pub fn set_event(&mut self, f: ButtonEventType) {
        self.event = Some(f);
    }

    pub fn on_click(&self, refer: ButtonArg, val: u8) {
        if self.event.is_some() {
            match self.event.as_ref().unwrap() {
                ButtonEventType::TacEvent(tac_event) => {
                    if let ButtonArg::Tac(tac) = refer {
                        (tac_event)(tac);
                    }
                }
                ButtonEventType::ConsoleEvent(console_event) => {
                    if let ButtonArg::Console(console) = refer {
                        (console_event)(console, val);
                    }
                }
            }
        }
    }
}
