use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::traits::io::device::io_serial::IIOSerial;
use web_sys::HtmlTextAreaElement;

pub struct TerminalIO {
    sendable_intr_flag: bool,
    recivable_intr_flag: bool,
    empty_flag: bool,
    buf: u8,
    terminal: HtmlTextAreaElement,
    intr_sig: IntrController,
}

impl TerminalIO {
    pub fn new(terminal: HtmlTextAreaElement, intr_sig: IntrController) -> Self {
        Self {
            sendable_intr_flag: false,
            recivable_intr_flag: false,
            empty_flag: false,
            buf: 0u8,
            terminal,
            intr_sig,
        }
    }
}

impl IIOSerial for TerminalIO {
    fn receive(&mut self, _val: u8) -> u8 {
        self.empty_flag = true;
        self.buf
    }

    fn send(&mut self, val: u8) {
        if val == 0x08 {
            self.terminal.set_inner_text(
                &(self.terminal.value()
                    + &self.terminal.value()[..self.terminal.value().len() - 1]),
            );
        } else {
            let ch = std::char::from_u32(val as u32)
                .unwrap()
                .to_string()
                .replace("/\r/", "");
            self.terminal.set_inner_text(&(self.terminal.value() + &ch));
            if ch.eq("/\n/") {
                self.terminal.set_scroll_top(self.terminal.scroll_height());
            }
        }

        if self.sendable_intr_flag {
            self.intr_sig.interrupt(Interrupt::RN4020_SENT);
        }
    }

    fn set_receivable_intr_flag(&mut self, flag: bool) {
        self.recivable_intr_flag = flag;
        if self.recivable_intr_flag && !self.empty_flag {
            self.intr_sig.interrupt(Interrupt::FT232RL_RECEIVED);
        }
    }

    fn set_sendable_intr_flag(&mut self, flag: bool) {
        self.sendable_intr_flag = flag;
        if self.sendable_intr_flag && self.empty_flag {
            self.intr_sig.interrupt(Interrupt::FT232RL_SENT)
        }
    }
    fn is_readable(&self) -> bool {
        !self.empty_flag
    }

    fn is_writable(&self) -> bool {
        self.empty_flag
    }
}

impl TerminalIO {
    fn input_key_down(&mut self, e: yew::KeyboardEvent) {
        self.buf = self.key_to_ascii(e.key());
        self.empty_flag = false;
        if self.recivable_intr_flag {
            self.intr_sig.interrupt(Interrupt::FT232RL_RECEIVED);
        }
    }

    fn key_to_ascii(&self, key: String) -> u8 {
        if key.len() != 1 {
            return match key.as_str() {
                "BackSpace" => 0x08,
                "Tab" => 0x09,
                "Enter" => 0x0d,
                "Escape" => 0x1b,
                "Delete" => 0x7f,
                _ => 0u8,
            };
        };
        if let Some(ch) = key.as_str().chars().next() {
            ch as u8
        } else {
            0u8
        }
    }

    fn reset(&mut self) {
        self.sendable_intr_flag = false;
        self.recivable_intr_flag = false;
        self.empty_flag = false;
        self.buf = 0;
    }
}
