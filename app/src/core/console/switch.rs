use std::ops::{Index, IndexMut};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

const SWITCH_BASE_WIDTH: f64 = 30.0;
const SWITCH_BASE_HEIGHT: f64 = 36.0;
const SWITCH_BASE_RADIUS: f64 = 12.0;
const SWITCH_BODY_WIDTH: f64 = 8.0;
const SWITCH_BODY_HEIGHT: f64 = 22.0;
const SWITCH_HEAD_HEIGHT: f64 = 6.0;

const SWITCH_BASE_COLOR: &str = "#252525";
const SWITCH_HEAD_COLOR: &str = "#993300";
const SWITCH_BODY_COLOR: &str = "#f8f8f8";

pub enum SwitchName {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    BREAK,
    STEP,
}

impl Index<SwitchName> for Vec<Switch> {
    type Output = Switch;

    fn index(&self, index: SwitchName) -> &Self::Output {
        match index {
            SwitchName::D0 => &self[0],
            SwitchName::D1 => &self[1],
            SwitchName::D2 => &self[2],
            SwitchName::D3 => &self[3],
            SwitchName::D4 => &self[4],
            SwitchName::D5 => &self[5],
            SwitchName::D6 => &self[6],
            SwitchName::D7 => &self[7],
            SwitchName::BREAK => &self[8],
            SwitchName::STEP => &self[9],
        }
    }
}

impl IndexMut<SwitchName> for Vec<Switch> {
    fn index_mut(&mut self, index: SwitchName) -> &mut Self::Output {
        match index {
            SwitchName::D0 => &mut self[0],
            SwitchName::D1 => &mut self[1],
            SwitchName::D2 => &mut self[2],
            SwitchName::D3 => &mut self[3],
            SwitchName::D4 => &mut self[4],
            SwitchName::D5 => &mut self[5],
            SwitchName::D6 => &mut self[6],
            SwitchName::D7 => &mut self[7],
            SwitchName::BREAK => &mut self[8],
            SwitchName::STEP => &mut self[9],
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Switch {
    pos_x: f64,
    pos_y: f64,
    state: bool,
    ctx: CanvasRenderingContext2d,
}

impl Switch {
    pub fn new(pos_x: f64, pos_y: f64, ctx: CanvasRenderingContext2d) -> Self {
        Self {
            pos_x,
            pos_y,
            state: false,
            ctx,
        }
    }

    pub fn draw(&self) {
        self.ctx.begin_path();
        self.ctx
            .arc(
                self.pos_x + SWITCH_BASE_WIDTH / 2.0,
                self.pos_y + SWITCH_BASE_HEIGHT / 2.0,
                SWITCH_BASE_RADIUS,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        self.ctx
            .set_fill_style(&JsValue::from_str(SWITCH_BASE_COLOR));
        self.ctx.fill();

        self.ctx.begin_path();
        if self.state {
            self.ctx.rect(
                self.pos_x + SWITCH_BASE_WIDTH / 2.0 - SWITCH_BODY_WIDTH / 2.0,
                self.pos_y + SWITCH_BASE_HEIGHT / 2.0 - SWITCH_BODY_WIDTH / 2.0,
                SWITCH_BODY_WIDTH,
                -SWITCH_BODY_HEIGHT,
            );
            self.ctx
                .set_fill_style(&JsValue::from_str(SWITCH_BODY_COLOR));
            self.ctx.fill();
            self.ctx.begin_path();
            self.ctx.rect(
                self.pos_x + SWITCH_BASE_WIDTH / 2.0 - SWITCH_BODY_WIDTH / 2.0,
                self.pos_y + SWITCH_BASE_HEIGHT / 2.0
                    - SWITCH_BODY_WIDTH / 2.0
                    - SWITCH_BODY_HEIGHT
                    - SWITCH_HEAD_HEIGHT,
                SWITCH_BODY_WIDTH,
                SWITCH_HEAD_HEIGHT,
            );
            self.ctx
                .set_fill_style(&JsValue::from_str(SWITCH_HEAD_COLOR));
            self.ctx.fill();
        } else {
            self.ctx.rect(
                self.pos_x + SWITCH_BASE_WIDTH / 2.0 - SWITCH_BODY_WIDTH / 2.0,
                self.pos_y + SWITCH_BASE_HEIGHT / 2.0 - SWITCH_BODY_WIDTH / 2.0,
                SWITCH_BODY_WIDTH,
                SWITCH_BODY_HEIGHT,
            );
            self.ctx
                .set_fill_style(&JsValue::from_str(SWITCH_BODY_COLOR));
            self.ctx.fill();
            self.ctx.begin_path();
            self.ctx.rect(
                self.pos_x + SWITCH_BASE_WIDTH / 2.0 - SWITCH_BODY_WIDTH / 2.0,
                self.pos_y + SWITCH_BASE_HEIGHT / 2.0 - SWITCH_BODY_WIDTH / 2.0
                    + SWITCH_BODY_HEIGHT,
                SWITCH_BODY_WIDTH,
                SWITCH_HEAD_HEIGHT,
            );
            self.ctx
                .set_fill_style(&JsValue::from_str(SWITCH_HEAD_COLOR));
            self.ctx.fill();
        }
    }

    pub fn is_switch_clicked(&self, click_x: f64, click_y: f64) -> bool {
        return self.pos_x <= click_x
            && click_x <= self.pos_x + SWITCH_BASE_WIDTH
            && self.pos_y <= click_y
            && click_y <= self.pos_y + SWITCH_BASE_HEIGHT;
    }

    pub fn toggle_state(&mut self) {
        self.state = !self.state;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }
}
