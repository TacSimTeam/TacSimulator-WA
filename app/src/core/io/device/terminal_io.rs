use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::traits::io::device::io_serial::IIOSerial;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlTextAreaElement;
use yew::NodeRef;

#[derive(PartialEq, Clone)]
pub struct TerminalIO {
    sendable_intr_flag: bool,
    recivable_intr_flag: bool,
    empty_flag: bool,
    buf: u8,
    terminal: NodeRef,
    intr_sig: Rc<RefCell<IntrController>>,
}

impl TerminalIO {
    pub fn new(terminal: NodeRef, intr_sig: Rc<RefCell<IntrController>>) -> Self {
        Self {
            sendable_intr_flag: false,
            recivable_intr_flag: false,
            empty_flag: true,
            buf: 0u8,
            terminal,
            intr_sig,
        }
    }
}

impl IIOSerial for TerminalIO {
    fn receive(&mut self) -> u8 {
        self.empty_flag = true;
        self.buf
    }

    fn send(&mut self, val: u8) {
        let terminal = self.terminal.cast::<HtmlTextAreaElement>().unwrap();
        if val == 0x08 {
            terminal.set_value(&terminal.value()[..terminal.value().len() - 1]);
        } else {
            let ch = std::char::from_u32(val as u32)
                .unwrap()
                .to_string()
                .replace('\r', "");
            terminal.set_value(&(terminal.value() + &ch));
            if ch.eq("\n") {
                terminal.set_scroll_top(terminal.scroll_height());
            }
        }

        if self.sendable_intr_flag {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::FT232RL_SENT);
        }
    }

    fn set_receivable_intr_flag(&mut self, flag: bool) {
        self.recivable_intr_flag = flag;
        if self.recivable_intr_flag && !self.empty_flag {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::FT232RL_RECEIVED);
        }
    }

    fn set_sendable_intr_flag(&mut self, flag: bool) {
        self.sendable_intr_flag = flag;
        if self.sendable_intr_flag && self.empty_flag {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::FT232RL_SENT)
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
    pub fn input_key_down(&mut self, e: yew::KeyboardEvent) {
        self.buf = self.key_to_ascii(e.key());
        self.empty_flag = false;
        if self.recivable_intr_flag {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::FT232RL_RECEIVED);
        }
    }

    fn key_to_ascii(&self, key: String) -> u8 {
        if key.len() != 1 {
            return match key.as_str() {
                "Backspace" => 0x08,
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

    pub fn reset(&mut self) {
        self.sendable_intr_flag = false;
        self.recivable_intr_flag = false;
        self.empty_flag = true;
        self.buf = 0;
    }
}
