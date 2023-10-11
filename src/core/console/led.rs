use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const LED_RADIUS: f64 = 10.0;

const LED_COLOR_RED_LIGHT: &str = "#ff0000";
const LED_COLOR_RED_DARK: &str = "#400000";
const LED_COLOR_GREEN_LIGHT: &str = "#00ff00";
const LED_COLOR_GREEN_DARK: &str = "#004000";
const LED_COLOR_YELLOW_LIGHT: &str = "#FFFF00";
const LED_COLOR_YELLOW_DARK: &str = "#DAA520";

pub enum LedColor {
    Red,
    Yellow,
    Green,
}

pub struct Led {
    pos_x: f64,
    pos_y: f64,
    state: bool,
    color: LedColor,
    ctx: CanvasRenderingContext2d,
}

impl Led {
    pub fn new(pos_x: f64, pos_y: f64, color: LedColor, ctx: CanvasRenderingContext2d) -> Self {
        Self {
            pos_x,
            pos_y,
            state: false,
            color,
            ctx,
        }
    }

    pub fn draw(&self) {
        self.ctx.begin_path();
        self.ctx
            .arc(
                self.pos_x,
                self.pos_y,
                LED_RADIUS,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        self.ctx
            .set_fill_style(&JsValue::from_str(match self.color {
                LedColor::Red => {
                    if self.state {
                        LED_COLOR_RED_LIGHT
                    } else {
                        LED_COLOR_RED_DARK
                    }
                }
                LedColor::Yellow => {
                    if self.state {
                        LED_COLOR_YELLOW_LIGHT
                    } else {
                        LED_COLOR_YELLOW_DARK
                    }
                }
                LedColor::Green => {
                    if self.state {
                        LED_COLOR_GREEN_LIGHT
                    } else {
                        LED_COLOR_GREEN_DARK
                    }
                }
            }));
        self.ctx.fill();
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }
}
