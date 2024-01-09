use crate::core::cpu::consts::flags::Flags;

#[derive(Clone, Debug, PartialEq)]
pub struct Psw {
    pc: u16,
    flag: u16,
}

impl Psw {
    pub fn new() -> Self {
        Self {
            pc: 0xe000,
            flag: Flags::PRIV as u16,
        }
    }

    pub fn next_pc(&mut self) {
        if self.pc >= 0xfffe {
            gloo::console::log!("PCの値が0xffffを超える")
        }
        self.pc += 2;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn jump(&mut self, addr: u16) {
        if addr % 2 != 0 {
            gloo::console::warn!("奇数番地にジャンプしようとしている")
        }
        self.pc = addr;
    }

    pub fn get_flag(&self) -> u16 {
        self.flag
    }

    pub fn set_flag(&mut self, flag: u16) {
        if self.check_flag(Flags::PRIV as u16) {
            self.flag = flag;
            return;
        }

        self.flag = (0xe0 & self.flag) | (0x1f & flag);
    }

    pub fn check_flag(&self, flag: u16) -> bool {
        self.flag & flag != 0
    }

    pub fn get_priv_flag(&self) -> bool {
        self.check_flag(Flags::PRIV as u16)
    }

    pub fn set_priv_flag(&mut self, is_priv: bool) {
        if is_priv {
            self.flag = self.flag | Flags::PRIV as u16;
        } else {
            self.flag = self.flag & (!(Flags::PRIV as u16) & 0xff);
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0xe000;
        self.flag = Flags::PRIV as u16;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::consts::flags::Flags;
    use crate::core::cpu::psw::Psw;

    #[test]
    fn test_psw_next_pc_jump() {
        let mut psw = Psw::new();
        psw.next_pc();
        assert_eq!(0xe002, psw.get_pc());
        psw.jump(0x1000);
        assert_eq!(0x1000, psw.get_pc());
    }

    #[test]
    fn test_psw_change_flag() {
        let mut psw = Psw::new();
        assert!(!psw.check_flag(Flags::ZERO as u16));
        psw.set_flag(Flags::ZERO as u16);
        assert!(psw.check_flag(Flags::ZERO as u16));

        assert!(!psw.check_flag(Flags::SIGN as u16));
        psw.set_flag(Flags::SIGN as u16);
        assert!(psw.check_flag(Flags::SIGN as u16));

        psw.reset();
        assert!(!psw.check_flag(Flags::ENABLE_INTR as u16));
        psw.set_flag(Flags::ENABLE_INTR as u16);
        assert!(psw.check_flag(Flags::ENABLE_INTR as u16));
    }
}
