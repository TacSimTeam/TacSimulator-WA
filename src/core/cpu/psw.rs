use crate::core::cpu::consts::flags::Flags;

pub struct Psw {
    pc: u16,
    flag: u8,
}

impl Psw {
    pub fn new() -> Self {
        Self {
            pc: 0xe000,
            flag: Flags::PRIV as u8,
        }
    }

    pub fn next_pc(&mut self) {
        if self.pc >= 0xfffe {
            log::error!("PCの値が0xffffを超える")
        }
        self.pc += 2
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn jump(&mut self, addr: u16) {
        // if addr >= 0xffff {
        //     log::error!("PCの値が0xffffを超える")
        // } else if addr % 2 == 0 {
        if addr % 2 == 0 {
            log::error!("奇数番地にジャンプしようとしている")
        }
        self.pc = addr;
    }

    pub fn get_flag(&self) -> u8 {
        self.flag
    }

    pub fn set_flag(&mut self, flag: u8) {
        if self.check_flag(Flags::PRIV as u8) {
            self.flag = flag;
            return;
        }

        self.flag = (0xe0 & self.flag) | (0x1f & flag);
    }

    pub fn check_flag(&self, flag: u8) -> bool {
        self.flag == flag
    }

    pub fn get_priv_flag(&self) -> bool {
        self.check_flag(Flags::PRIV as u8)
    }

    pub fn set_priv_flag(&mut self, is_priv: bool) {
        if is_priv {
            self.flag = self.flag | Flags::PRIV as u8;
        } else {
            self.flag = self.flag & (!(Flags::PRIV as u8) & 0xff);
        }
    }
}
