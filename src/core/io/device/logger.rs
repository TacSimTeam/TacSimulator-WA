use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::traits::io::device::io_serial::IIOSerial;
use gloo::console;
use web_sys::{Document, HtmlInputElement};

pub struct Logger {
    sendable_intr_flag: bool,
    buf: String,
    intr_sig: IntrController,
    logger_switch: HtmlInputElement,
}

impl Logger {
    pub fn new(intr_sig: IntrController, logger_switch: HtmlInputElement) -> Self {
        Self {
            sendable_intr_flag: false,
            buf: String::new(),
            intr_sig,
            logger_switch,
        }
    }
}

impl IIOSerial for Logger {
    fn receive(&mut self, _val: u8) -> u8 {
        0u8
    }

    fn send(&mut self, val: u8) {
        if self.logger_switch.checked() {
            self.buf = String::new();
        } else if val == 0x08 {
            self.buf = self.buf[..self.buf.len() - 1].to_string();
        } else {
            let ch = std::char::from_u32(val as u32)
                .unwrap()
                .to_string()
                .replace("/\r/", "");
            self.buf = self.buf.clone() + &ch;
            if ch.eq("/\n/") {
                console::info!(self.buf.clone());
                self.buf = String::new();
            }
        }

        if self.sendable_intr_flag {
            self.intr_sig.interrupt(Interrupt::RN4020_SENT);
        }
    }

    fn set_receivable_intr_flag(&mut self, _flag: bool) {}

    fn set_sendable_intr_flag(&mut self, flag: bool) {
        self.sendable_intr_flag = flag;
        if self.sendable_intr_flag {
            self.intr_sig.interrupt(Interrupt::RN4020_SENT)
        }
    }

    fn is_readable(&self) -> bool {
        false
    }

    fn is_writable(&self) -> bool {
        true
    }
}
