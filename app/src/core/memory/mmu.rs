use crate::core::consts::{ERROR_CAUSE_BAD_ADDRESS, ERROR_CAUSE_MEMORY_VIOLATION, TLB_ENTRY_SIZE};
use crate::core::cpu::psw::Psw;
use crate::core::error::tlb_error::TlbError;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::ipl::IPL;
use crate::core::memory::memory::Memory;
use crate::core::memory::tlb::TlbEntry;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub struct Mmu {
    memory: Rc<RefCell<Memory>>,
    intr_sig: Rc<RefCell<IntrController>>,
    priv_sig: Rc<RefCell<Psw>>,
    tlbs: Vec<TlbEntry>,
    ipl_mode: bool,
    mmu_mode: bool,
    err_addr: u16,
    err_cause: u8,
    tlb_miss_page: u8,
}

impl Mmu {
    pub fn new(
        memory: Rc<RefCell<Memory>>,
        intr_sig: Rc<RefCell<IntrController>>,
        priv_sig: Rc<RefCell<Psw>>,
    ) -> Self {
        Self {
            memory,
            intr_sig,
            priv_sig,
            tlbs: vec![TlbEntry::new(0); TLB_ENTRY_SIZE as usize],
            ipl_mode: false,
            mmu_mode: false,
            err_addr: 0,
            err_cause: 0,
            tlb_miss_page: 0,
        }
    }

    pub fn read8(&mut self, addr: u16) -> Result<u8, TlbError> {
        let mut addr = addr;
        if self.mmu_mode && !self.priv_sig.borrow().get_priv_flag() {
            let mut entry = self.v_addr_to_entry(addr)?;
            if !entry.is_readable() {
                self.report_mem_vio_error(addr);
                return Ok(0u8);
            }
            entry.set_reference_flag();
            addr = ((entry.get_frame() as u16) << 8) | (addr & 0x00ff);
        }
        Ok(self.memory.borrow().read8(addr))
    }

    pub fn write8(&mut self, addr: u16, val: u8) -> Result<(), TlbError> {
        let mut addr = addr;
        if self.ipl_mode && addr >= 0xe000 {
            return Err(TlbError::ReadOnly);
        }
        if self.mmu_mode && !self.priv_sig.borrow().get_priv_flag() {
            let mut entry = self.v_addr_to_entry(addr)?;
            if !entry.is_writable() {
                self.report_mem_vio_error(addr);
                return Ok(());
            }
            entry.set_reference_flag();
            entry.set_dirty_flag();
            addr = ((entry.get_frame() as u16) << 8) | (addr & 0x00ff);
        }
        self.memory.borrow_mut().write8(addr, val);

        Ok(())
    }

    pub fn read16(&mut self, addr: u16) -> Result<u16, TlbError> {
        let mut addr = addr;
        if addr % 2 == 1 {
            self.report_bad_addr_error(addr);
            return Ok(0);
        }

        if self.mmu_mode && !self.priv_sig.borrow().get_priv_flag() {
            let mut entry = self.v_addr_to_entry(addr)?;
            if !entry.is_readable() {
                self.report_mem_vio_error(addr);
                return Ok(0);
            }
            entry.set_reference_flag();
            addr = (entry.get_frame() as u16) << 8 | (addr & 0x00ff);
        }
        Ok(self.memory.borrow().read16(addr))
    }

    pub fn write16(&mut self, addr: u16, val: u16) -> Result<(), TlbError> {
        let mut addr = addr;
        if addr % 2 == 1 {
            self.report_bad_addr_error(addr);
            return Ok(());
        }

        if self.ipl_mode && addr >= 0xe000 {
            return Err(TlbError::ReadOnly);
        }

        if self.mmu_mode & !self.priv_sig.borrow().get_priv_flag() {
            let mut entry = self.v_addr_to_entry(addr)?;

            if !entry.is_writable() {
                self.report_mem_vio_error(addr);
                return Ok(());
            }

            entry.set_reference_flag();
            entry.set_dirty_flag();
            addr = (entry.get_frame() as u16) << 8 | (addr & 0x00ff);
        }
        self.memory.borrow_mut().write16(addr, val);
        Ok(())
    }

    pub fn fetch(&mut self, pc: u16) -> Result<u16, TlbError> {
        let mut pc = pc;
        if pc % 2 == 1 {
            self.report_bad_addr_error(pc);
            return Ok(0);
        }
        if self.mmu_mode && !self.priv_sig.borrow().get_priv_flag() {
            let mut entry = match self.v_addr_to_entry(pc) {
                Ok(entry) => entry,
                Err(e) => {
                    return Err(e);
                }
            };
            if !entry.is_executable() {
                self.report_mem_vio_error(pc);
                return Ok(0);
            }

            entry.set_reference_flag();
            pc = (entry.get_frame() as u16) << 8 | (pc & 0x00ff);
        }
        return Ok(self.memory.borrow().fetch(pc));
    }

    fn v_addr_to_entry(&mut self, v_addr: u16) -> Result<TlbEntry, TlbError> {
        let page = ((v_addr & 0xff00) >> 8) as u8;
        let entry_num = self.search_tlb_num(page);
        if entry_num.is_none() {
            self.report_tlb_miss_error(page);
            return Err(TlbError::TlbMiss);
        }
        Ok(self.tlbs[entry_num.unwrap() as usize])
    }

    fn search_tlb_num(&self, page: u8) -> Option<u8> {
        for i in 0..TLB_ENTRY_SIZE {
            if self.tlbs[i as usize].is_valid() && self.tlbs[i as usize].get_page() == page {
                return Some(i as u8);
            }
        }
        None
    }

    fn report_mem_vio_error(&mut self, addr: u16) {
        self.err_addr = addr;
        self.err_cause |= ERROR_CAUSE_MEMORY_VIOLATION;
        self.intr_sig
            .borrow_mut()
            .interrupt(Interrupt::EXCP_MEMORY_ERROR)
    }

    fn report_bad_addr_error(&mut self, addr: u16) {
        self.err_addr = addr;
        self.err_cause |= ERROR_CAUSE_BAD_ADDRESS;
        self.intr_sig
            .borrow_mut()
            .interrupt(Interrupt::EXCP_MEMORY_ERROR)
    }

    fn report_tlb_miss_error(&mut self, page: u8) {
        self.tlb_miss_page = page;
        self.intr_sig
            .borrow_mut()
            .interrupt(Interrupt::EXCP_TLB_MISS);
    }

    pub fn get_tlb_high_8(&self, entry_num: u8) -> u8 {
        self.tlbs[entry_num as usize].get_high_8()
    }

    pub fn get_tlb_low_16(&self, entry_num: u8) -> u16 {
        self.tlbs[entry_num as usize].get_low_16()
    }

    pub fn set_tlb_high_8(&mut self, entry_num: u8, val: u32) {
        self.tlbs[entry_num as usize].set_high_8(val);
    }

    pub fn set_tlb_low_16(&mut self, entry_num: u8, val: u32) {
        self.tlbs[entry_num as usize].set_low_16(val);
    }

    pub fn get_error_addr(&self) -> u16 {
        self.err_addr
    }

    pub fn get_error_page(&self) -> u8 {
        self.tlb_miss_page
    }

    pub fn get_error_cause(&mut self) -> u8 {
        let cause = self.err_cause;
        self.err_cause = 0;
        cause
    }

    pub fn detach_ipl(&mut self) {
        self.ipl_mode = false;
        for i in 0xe000..=0xffff {
            self.memory.borrow_mut().write8(i as u16, 0);
        }
        // self.memory.borrow_mut().write16(INTERRUPT_VECTOR + 8 * 2, 13578);
    }

    pub fn enable(&mut self) {
        self.mmu_mode = true;
    }

    pub fn load_ipl(&mut self) {
        if self.ipl_mode {
            return;
        }
        for (i, v) in IPL.iter().enumerate() {
            self.memory
                .borrow_mut()
                .write16(0xe000 + (i * 2) as u16, *v);
        }
        self.ipl_mode = true;
    }
    pub fn reset(&mut self) {
        for i in 0..=0xffff {
            self.memory.borrow_mut().write8(i, 0);
        }
        let _ = self.tlbs.iter_mut().map(|e| {
            e.reset();
        });
        self.ipl_mode = false;
        self.mmu_mode = false;
        self.err_addr = 0;
        self.err_cause = 0;
        self.tlb_miss_page = 0;
    }

    pub fn is_ipl_mode(&self) -> bool {
        self.ipl_mode
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::psw::Psw;
    use crate::core::error::tlb_error::TlbError;
    use crate::core::interrupt::intr_controller::IntrController;
    use crate::core::memory::memory::Memory;
    use crate::core::memory::mmu::Mmu;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_mmu_write_read() {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let intr_sig = Rc::new(RefCell::new(IntrController::new()));
        let psw = Rc::new(RefCell::new(Psw::new()));
        let mut mmu = Mmu::new(memory, intr_sig, psw);

        assert_eq!(mmu.write16(0x1000, 1), Ok(()));
        assert_eq!(mmu.read16(0x1000), Ok(1));
        assert_eq!(mmu.read16(0x2000), Ok(0));
    }

    #[test]
    fn test_ipl_load() {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let intr_sig = Rc::new(RefCell::new(IntrController::new()));
        let psw = Rc::new(RefCell::new(Psw::new()));
        let mut mmu = Mmu::new(memory, intr_sig, psw);

        mmu.load_ipl();
        assert_eq!(mmu.write16(0xe000, 1), Err(TlbError::ReadOnly));
        assert_eq!(mmu.write16(0xfffe, 1), Err(TlbError::ReadOnly));
        mmu.detach_ipl();

        assert_eq!(mmu.write16(0xe000, 1), Ok(()));
        assert_eq!(mmu.read16(0xe000), Ok(1));
        assert_eq!(mmu.write16(0xfffe, 1), Ok(()));
        assert_eq!(mmu.read16(0xfffe), Ok(1));
    }

    #[test]
    fn test_tlb_entries() {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let intr_sig = Rc::new(RefCell::new(IntrController::new()));
        let psw = Rc::new(RefCell::new(Psw::new()));
        let mut mmu = Mmu::new(memory, intr_sig, psw);

        mmu.set_tlb_high_8(1, 0xff);
        mmu.set_tlb_low_16(1, 0xffff);
        assert_eq!(mmu.get_tlb_high_8(1), 0xff);
        assert_eq!(mmu.get_tlb_low_16(1), 0xffff);
    }
}
