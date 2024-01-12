use crate::core::console::button::ButtonName;
use crate::core::console::button::{Button, ButtonEventType};
use crate::core::console::led::{Led, LedColor};
use crate::core::console::switch::{Switch, SwitchName};
use crate::core::traits::console::console::IConsoleStateAction;
use web_sys::CanvasRenderingContext2d;

#[derive(PartialEq, Clone, Default)]
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
        // RUN Led
        self.flag_led
            .push(Led::new(409.0, 69.0, LedColor::Red, canvas_ctx.clone()));
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

        // イベントの追加
        self.buttons[ButtonName::LeftAllowBtn].set_event(ButtonEventType::ConsoleEvent(
            IConsoleStateAction::left_allow_btn_event,
        ));
        self.buttons[ButtonName::RightAllowBtn].set_event(ButtonEventType::ConsoleEvent(
            IConsoleStateAction::right_allow_btn_event,
        ));
        self.buttons[ButtonName::SetaBtn].set_event(ButtonEventType::ConsoleEvent(
            IConsoleStateAction::seta_btn_event,
        ));
        self.buttons[ButtonName::IncaBtn].set_event(ButtonEventType::ConsoleEvent(
            IConsoleStateAction::inca_btn_event,
        ));
        self.buttons[ButtonName::DecaBtn].set_event(ButtonEventType::ConsoleEvent(
            IConsoleStateAction::deca_btn_event,
        ));
        self.buttons[ButtonName::WriteBtn].set_event(ButtonEventType::ConsoleEvent(
            IConsoleStateAction::write_btn_event,
        ));
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

    pub fn switch_state(&self) -> u8 {
        let mut state = 0;
        for (i, s) in self.switches.iter().enumerate() {
            if s.get_state() && i < 8 {
                state = state | (1 << i);
            }
        }
        state
    }

    pub fn get_data_switches(&self) -> u8 {
        self.switch_state()
    }

    pub fn get_func_switch(&self) -> u8 {
        0u8
    }

    pub fn get_break_switch(&self) -> bool {
        self.switches[SwitchName::BREAK].get_state()
    }

    pub fn get_step_switch(&self) -> bool {
        self.switches[SwitchName::STEP].get_state()
    }

    pub fn set_led_lamps(&mut self, val: u16) {
        self.addr_led[7].set_state((val & (1 << 15)) != 0);
        self.addr_led[6].set_state((val & (1 << 14)) != 0);
        self.addr_led[5].set_state((val & (1 << 13)) != 0);
        self.addr_led[4].set_state((val & (1 << 12)) != 0);
        self.addr_led[3].set_state((val & (1 << 11)) != 0);
        self.addr_led[2].set_state((val & (1 << 10)) != 0);
        self.addr_led[1].set_state((val & (1 << 9)) != 0);
        self.addr_led[0].set_state((val & (1 << 8)) != 0);
        self.data_led[7].set_state((val & (1 << 7)) != 0);
        self.data_led[6].set_state((val & (1 << 6)) != 0);
        self.data_led[5].set_state((val & (1 << 5)) != 0);
        self.data_led[4].set_state((val & (1 << 4)) != 0);
        self.data_led[3].set_state((val & (1 << 3)) != 0);
        self.data_led[2].set_state((val & (1 << 2)) != 0);
        self.data_led[1].set_state((val & (1 << 1)) != 0);
        self.data_led[0].set_state((val & (1 << 0)) != 0);
        self.draw();
    }

    pub fn set_run_led_state(&mut self, val: bool) {
        self.flag_led[3].set_state(val);
    }
}
