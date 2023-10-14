use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::memory::memory::Memory;
use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;

pub struct ConsoleState {
    memory: Rc<RefCell<Memory>>,
    psw: Rc<RefCell<Psw>>,
    register: Rc<RefCell<Register>>,
    pub rot_current: u8,
    mem_addr: u16,
    pub run_flag: bool,
}

impl ConsoleState {
    pub fn new(
        memory: Rc<RefCell<Memory>>,
        psw: Rc<RefCell<Psw>>,
        register: Rc<RefCell<Register>>,
    ) -> Self {
        Self {
            memory,
            psw,
            register,
            rot_current: 0u8,
            mem_addr: 0u16,
            run_flag: false,
        }
    }
}

pub trait IConsoleState {
    fn left_allow_btn_event(&mut self);
    fn right_allow_btn_event(&mut self);
    fn seta_btn_event(&mut self);
    fn inca_btn_event(&mut self);
    fn deca_btn_event(&mut self);
    fn write_btn_event(&mut self);

    fn read_sw_value(&self) -> u8;
    fn push_sw_value_to_reg(&mut self);
    fn write_mem_data(&mut self, data: u16);
    fn get_mem_addr(&self) -> u16;
    fn get_mem_data(&self) -> u16;
    fn read_mem_data(&self) -> u16;
    fn read_reg(&self) -> u16;
}

impl IConsoleState for ConsoleState {
    fn left_allow_btn_event(&mut self) {
        gloo::console::log!("left allow btn clicked");
        if self.rot_current != 0 {
            self.rot_current -= 1;
            gloo::console::log!(format!("current {}", self.rot_current));
        }
    }

    fn right_allow_btn_event(&mut self) {
        gloo::console::log!("right allow btn clicked");
        if self.rot_current != 17 {
            self.rot_current += 1;
            gloo::console::log!(format!("current {}", self.rot_current));
        }
    }

    fn seta_btn_event(&mut self) {
        gloo::console::log!("seta btn clicked");
        self.mem_addr = (self.mem_addr << 8) | (self.read_sw_value() & 0xff) as u16;
    }

    fn inca_btn_event(&mut self) {
        gloo::console::log!("inca btn clicked");
        if self.mem_addr == 0xfffe {
            self.mem_addr = 0u16;
        } else {
            self.mem_addr += 2u16;
        }
    }

    fn deca_btn_event(&mut self) {
        gloo::console::log!("deca btn clicked");
        if self.mem_addr == 0 {
            self.mem_addr = 0xfffe;
        } else {
            self.mem_addr -= 2u16;
        }
    }

    fn write_btn_event(&mut self) {
        gloo::console::log!("write btn clicked");
        self.push_sw_value_to_reg();
    }

    fn read_sw_value(&self) -> u8 {
        // TODO スイッチの状態をとってくる実装をどうしようかな
        0u8
    }

    fn push_sw_value_to_reg(&mut self) {
        let val = self.read_sw_value();
        match self.rot_current {
            14u8 => self
                .psw
                .borrow_mut()
                .jump((((self.psw.borrow().get_pc() & 0x00ff) << 8) | (val & 0x00ff) as u16)),
            15u8 => self.psw.borrow_mut().set_flag(
                ((((self.psw.borrow().get_flag() & 0x00ff) as u16) << 8) | ((val & 0x00ff) as u16))
                    as u8,
            ),
            16u8 | 17u8 => {
                self.write_mem_data(((self.get_mem_addr() & 0x00ff) << 8) | (val & 0x00ff) as u16)
            }
            _ => self.register.borrow_mut().write(
                self.rot_current,
                (self.read_reg() & 0x00ff) << 8 | (val & 0x00ff) as u16,
            ),
        }
    }

    fn write_mem_data(&mut self, data: u16) {
        let addr = self.mem_addr & 0xfffe;
        self.memory.borrow_mut().write16(addr, data);
    }

    fn get_mem_addr(&self) -> u16 {
        self.mem_addr
    }

    fn get_mem_data(&self) -> u16 {
        self.read_mem_data()
    }

    fn read_mem_data(&self) -> u16 {
        let addr = self.mem_addr & 0xfffe;
        self.memory.borrow().read16(addr)
    }

    fn read_reg(&self) -> u16 {
        match self.rot_current {
            14u8 => self.psw.borrow().get_pc(),
            15u8 => self.psw.borrow().get_flag().into(),
            16u8 => self.read_mem_data(),
            17u8 => self.mem_addr,
            _ => self.register.borrow().read(self.rot_current),
        }
    }
}
