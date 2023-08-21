use crate::core::cpu::psw::Psw;

pub struct Register {
    generals: Vec<u16>,
    fp: u16,
    ssp: u16,
    usp: u16,
    priv_sig: Psw,
}

impl Register {
    pub fn new(priv_sig: Psw) -> Self {
        Self {
            generals: vec![0; 12],
            fp: 0,
            ssp: 0,
            usp: 0,
            priv_sig,
        }
    }

    fn read(&self, num: u8) -> u16 {
        match num {
            12 => self.fp,
            13 => {
                if self.priv_sig.get_priv_flag() {
                    self.ssp
                } else {
                    self.usp
                }
            }
            14 => self.usp & 0xffff,
            _ => self.generals[num as usize],
        }
    }

    fn write(&mut self, num: u8, val: u16) {
        match num {
            12 => self.fp = val & 0xffff,
            13 => {
                if self.priv_sig.get_priv_flag() {
                    self.ssp = val & 0xffff
                } else {
                    self.usp = val & 0xffff
                }
            }
            14 => self.usp = val & 0xffff,
            _ => self.generals[num as usize] = val & 0xffff,
        }
    }

    fn reset(&mut self) {
        self.generals.fill(0);
        self.fp = 0;
        self.ssp = 0;
        self.usp = 0;
    }
}
