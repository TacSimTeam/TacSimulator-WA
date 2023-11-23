use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::traits::io::device::io_serial::IIOSerial;

pub struct StdOutput {
    sendable_intr_flag: bool,
    recivable_intr_flag: bool,
    empty_flag: bool,
    buf: u8,
    intr_sig: IntrController,
}

impl StdOutput {
    pub fn new(intr_sig: IntrController) -> Self {
        Self {
            sendable_intr_flag: false,
            recivable_intr_flag: false,
            empty_flag: false,
            buf: 0u8,
            intr_sig,
        }
    }
}

impl IIOSerial for StdOutput {
    fn receive(&mut self) -> u8 {
        self.empty_flag = true;
        self.buf
    }

    fn send(&mut self, val: u8) {
        if val != 0x08 {
            let ch = std::char::from_u32(val as u32)
                .unwrap()
                .to_string()
                .replace("/\r/", "");
            print!("{}", &ch);
        }

        if self.sendable_intr_flag {
            self.intr_sig.interrupt(Interrupt::FT232RL_SENT);
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
            self.intr_sig.interrupt(Interrupt::FT232RL_SENT);
        }
    }

    fn is_readable(&self) -> bool {
        !self.empty_flag
    }

    fn is_writable(&self) -> bool {
        self.empty_flag
    }
}
