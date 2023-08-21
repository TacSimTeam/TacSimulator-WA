use crate::core::interrupt::intr_controller::IntrController;
use crate::core::memory::memory::Memory;

pub struct SdHostController {
    idle_flag: bool,
    error_flag: bool,
    intr_flag: bool,
    mem_addr: u16,
    sec_addr_high: u16,
    sec_addr_low: u16,
    memory: Memory,
    intr_sig: IntrController,
}

impl SdHostController {
    pub fn new(memory: Memory, intr_sig: IntrController) -> Self {
        Self {
            idle_flag: true,
            error_flag: false,
            intr_flag: false,
            mem_addr: 0,
            sec_addr_high: 0,
            sec_addr_low: 0,
            memory,
            intr_sig,
        }
    }

    pub fn start_reading(&mut self) {
        if !self.idle_flag {
            return;
        }
    }
}
