use crate::core::consts::SECTOR_SIZE;
use crate::core::error::sd_io_error::SdIoError;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::memory::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct SdHostController {
    idle_flag: bool,
    error_flag: bool,
    intr_flag: bool,
    mem_addr: u16,
    sec_addr_high: u16,
    sec_addr_low: u16,
    memory: Rc<RefCell<Memory>>,
    intr_sig: Rc<RefCell<IntrController>>,
    buf: Rc<RefCell<Vec<u8>>>,
}

impl SdHostController {
    pub fn new(
        memory: Rc<RefCell<Memory>>,
        intr_sig: Rc<RefCell<IntrController>>,
        dmg: Rc<RefCell<Vec<u8>>>,
    ) -> Self {
        Self {
            idle_flag: true,
            error_flag: false,
            intr_flag: false,
            mem_addr: 0,
            sec_addr_high: 0,
            sec_addr_low: 0,
            memory,
            intr_sig,
            buf: dmg,
        }
    }

    pub fn start_reading(&mut self) {
        if !self.idle_flag {
            return;
        }
        self.idle_flag = false;
        match self.read_sect(self.sec_addr()) {
            Ok(data) => {
                for (i, d) in data.iter().enumerate().take(SECTOR_SIZE) {
                    self.memory
                        .borrow_mut()
                        .write8(self.mem_addr + i as u16, *d);
                }
                self.idle_flag = true;
                if self.intr_flag {
                    self.intr_sig.borrow_mut().interrupt(Interrupt::MICRO_SD);
                }
            }
            Err(_) => {
                self.error_flag = true;
                self.idle_flag = false;
            }
        }
    }

    pub fn start_writing(&mut self) {
        if !self.idle_flag {
            return;
        }
        self.idle_flag = false;
        let mut data: Vec<u8> = Vec::new();
        for i in 0..SECTOR_SIZE {
            data.push(self.memory.borrow().read8(self.mem_addr + i as u16));
        }
        match self.write_sect(self.sec_addr(), data) {
            Ok(_) => {
                self.idle_flag = true;
                if self.intr_flag {
                    self.intr_sig.borrow_mut().interrupt(Interrupt::MICRO_SD);
                }
            }
            Err(_) => {
                self.error_flag = true;
                self.idle_flag = false;
            }
        }
    }

    pub fn get_mem_addr(&self) -> u16 {
        self.mem_addr
    }

    pub fn set_mem_addr(&mut self, val: u16) {
        self.mem_addr = val;
    }

    pub fn sec_addr(&self) -> u32 {
        ((self.get_sec_addr_high() as u32) << 16) | self.get_sec_addr_low() as u32
    }

    pub fn get_sec_addr_high(&self) -> u16 {
        self.sec_addr_high
    }

    pub fn get_sec_addr_low(&self) -> u16 {
        self.sec_addr_low
    }

    pub fn set_sec_addr_high(&mut self, addr_h: u16) {
        self.sec_addr_high = addr_h;
    }

    pub fn set_sec_addr_low(&mut self, addr_l: u16) {
        self.sec_addr_low = addr_l;
    }

    pub fn set_intr_flag(&mut self, flag: bool) {
        self.intr_flag = flag;
    }

    pub fn is_idle(&self) -> bool {
        self.idle_flag
    }

    pub fn is_error_occurred(&self) -> bool {
        self.error_flag
    }

    pub fn read_sect(&self, sect_addr: u32) -> Result<Vec<u8>, SdIoError> {
        let sect_addr = sect_addr as usize;
        return if self.buf.borrow().is_empty() {
            Err(SdIoError::SdIsNotOpen)
        } else {
            Ok(
                self.buf.borrow()[(SECTOR_SIZE * sect_addr)..(SECTOR_SIZE * (sect_addr + 1))]
                    .to_vec(),
            )
        };
    }

    pub fn write_sect(&mut self, sect_addr: u32, data: Vec<u8>) -> Result<(), SdIoError> {
        let sect_addr = sect_addr as usize;
        if self.buf.borrow().is_empty() {
            return Err(SdIoError::SdIsNotOpen);
        } else {
            for (i, d) in data.iter().enumerate().take(SECTOR_SIZE) {
                self.buf.borrow_mut()[sect_addr * SECTOR_SIZE + i] = *d;
            }
        }
        Ok(())
    }

    pub fn init(&self) {
        if self.intr_flag {
            self.intr_sig.borrow_mut().interrupt(Interrupt::MICRO_SD);
        }
    }

    pub fn reset(&mut self) {
        self.idle_flag = true;
        self.error_flag = false;
        self.intr_flag = false;
        self.mem_addr = 0;
        self.sec_addr_high = 0;
        self.sec_addr_low = 0;
    }

    pub fn is_sd_loaded(&self) -> bool {
        !self.buf.borrow().is_empty()
    }
}
