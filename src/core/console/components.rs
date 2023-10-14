use crate::core::console::button::{Button, ConsoleEventType};
use crate::core::console::console_state::{ConsoleState, IConsoleState};
use crate::core::console::led::{Led, LedColor};
use crate::core::console::switch::Switch;
use web_sys::CanvasRenderingContext2d;

pub struct Components {
    pub addr_led: Vec<Led>,
    pub buttons: Vec<Button>,
    pub data_led: Vec<Led>,
    pub flag_led: Vec<Led>,
    pub register_led: Vec<Led>,
    pub switches: Vec<Switch>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            addr_led: Vec::default(),
            buttons: Vec::default(),
            data_led: Vec::default(),
            flag_led: Vec::default(),
            register_led: Vec::default(),
            switches: Vec::default(),
        }
    }

    pub fn init(&mut self, canvas_ctx: CanvasRenderingContext2d) {
        for i in 0..=3 {
            self.addr_led.push(Led::new(
                (375 - i * 42) as f64,
                44.0,
                LedColor::Red,
                canvas_ctx.clone(),
            ));
            self.data_led.push(Led::new(
                (375 - i * 42) as f64,
                94.0,
                LedColor::Green,
                canvas_ctx.clone(),
            ));
        }
        for i in 4..8 {
            self.addr_led.push(Led::new(
                (195 - (i - 4) * 42) as f64,
                44.0,
                LedColor::Red,
                canvas_ctx.clone(),
            ));
            self.data_led.push(Led::new(
                (195 - (i - 4) * 42) as f64,
                94.0,
                LedColor::Green,
                canvas_ctx.clone(),
            ));
        }
        for i in 0..3 {
            self.flag_led.push(Led::new(
                (358 + i * 26) as f64,
                152.0,
                LedColor::Yellow,
                canvas_ctx.clone(),
            ));
        }
        for i in 0..6 {
            self.register_led.push(Led::new(
                (112 + i * 34) as f64,
                152.0,
                LedColor::Yellow,
                canvas_ctx.clone(),
            ));
        }
        // LeftAllowBtn
        self.buttons
            .push(Button::new(54.0, 138.0, canvas_ctx.clone()));
        // RightAllowBtn
        self.buttons
            .push(Button::new(309.0, 138.0, canvas_ctx.clone()));
        // ResetBtn
        self.buttons
            .push(Button::new(7.0, 312.0, canvas_ctx.clone()));
        // RunBtn
        self.buttons
            .push(Button::new(138.0, 312.0, canvas_ctx.clone()));
        // StopBtn
        self.buttons
            .push(Button::new(180.0, 312.0, canvas_ctx.clone()));
        // SetaBtn
        self.buttons
            .push(Button::new(236.0, 312.0, canvas_ctx.clone()));
        // IncaBtn
        self.buttons
            .push(Button::new(278.0, 312.0, canvas_ctx.clone()));
        // DecaBtn
        self.buttons
            .push(Button::new(320.0, 312.0, canvas_ctx.clone()));
        // WriteBtn
        self.buttons
            .push(Button::new(362.0, 312.0, canvas_ctx.clone()));

        // D0 - D7
        for i in 0..=3 {
            self.switches.push(Switch::new(
                (362 - i * 42) as f64,
                226.0,
                canvas_ctx.clone(),
            ));
        }
        for i in 4..8 {
            self.switches.push(Switch::new(
                (180 - (i - 4) * 42) as f64,
                226.0,
                canvas_ctx.clone(),
            ));
        }
        // BREAK
        self.switches
            .push(Switch::new(54.0, 312.0, canvas_ctx.clone()));
        // STEP
        self.switches
            .push(Switch::new(96.0, 312.0, canvas_ctx.clone()));

        // 初期
        self.register_led[0].set_state(true);

        // 以降はイベントの追加
        self.buttons[0].set_event(IConsoleState::left_allow_btn_event);
        self.buttons[1].set_event(IConsoleState::right_allow_btn_event);
        self.buttons[5].set_event(IConsoleState::seta_btn_event);
        self.buttons[6].set_event(IConsoleState::inca_btn_event);
        self.buttons[7].set_event(IConsoleState::deca_btn_event);
        self.buttons[8].set_event(IConsoleState::write_btn_event);
    }

    pub fn draw(&self) {
        for l in &self.addr_led {
            l.draw();
        }
        for l in &self.data_led {
            l.draw();
        }
        for l in &self.flag_led {
            l.draw();
        }
        for b in &self.buttons {
            b.draw();
        }
        for s in &self.switches {
            s.draw();
        }
        for l in &self.register_led {
            l.draw();
        }
    }
}
