use crate::core::consts::{ERROR_CAUSE_BAD_ADDRESS, ERROR_CAUSE_MEMORY_VIOLATION, TLB_ENTRY_SIZE};
use crate::core::cpu::psw::Psw;
use crate::core::error::tlb_error::TlbError;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::memory::memory::Memory;
use crate::core::memory::tlb::TlbEntry;

pub struct Mmu {
    memory: Memory,
    intr_sig: IntrController,
    priv_sig: Psw,
    tlbs: Vec<TlbEntry>,
    ipl_mode: bool,
    mmu_mode: bool,
    err_addr: u16,
    err_cause: u8,
    tlb_miss_page: u8,
}

impl Mmu {
    pub fn new(memory: Memory, intr_sig: IntrController, priv_sig: Psw) -> Self {
        Self {
            memory,
            intr_sig,
            priv_sig,
            tlbs: vec![TlbEntry::new(0);TLB_ENTRY_SIZE as usize],
            ipl_mode: false,
            mmu_mode: false,
            err_addr: 0,
            err_cause: 0u8,
            tlb_miss_page: 0
        }
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        if self.mmu_mode && !self.priv_sig.get_priv_flag() {
            // TODO 適切なエラー処理を実装
            let mut entry = self.v_addr_to_entry(addr).unwrap();
            if !entry.is_readable() {
                self.report_mem_vio_error(addr);
                return 0u8;
            }
            entry.set_reference_flag();
            let addr = ((entry.get_frame() as u16) << 8) | (addr & 0x00ff);
        }
        self.memory.read8(addr)
    }

    pub fn write8(&mut self, addr: u16, val: u8) -> Result<(), TlbError> {
        if self.ipl_mode && addr >= 0xe000 {
            return Err(TlbError::ReadOnly);
        }
        if self.mmu_mode && !self.priv_sig.get_priv_flag() {
            // TODO 適切なエラー処理を実装
            let mut entry = self.v_addr_to_entry(addr).unwrap();
            if !entry.is_readable() {
                self.report_mem_vio_error(addr);
                return Ok(());
            }
            entry.set_reference_flag();
            entry.set_dirty_flag();
            let addr = ((entry.get_frame() as u16) << 8) | (addr & 0x00ff);
        }
        self.memory.write8(addr, val);

        Ok(())
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        if addr % 2 == 1 {
            self.report_bad_addr_error(addr);
            return 0;
        }
        if self.mmu_mode && !self.priv_sig.get_priv_flag() {
            // TODO 適切なエラー処理を実装
            let mut entry = self.v_addr_to_entry(addr).unwrap();
            if !entry.is_readable() {
                self.report_mem_vio_error(addr);
                return 0;
            }
            entry.set_reference_flag();
            let addr = (entry.get_frame() as u16) << 8 | (addr & 0x00ff);
        }
        return self.memory.read16(addr);
    }

    pub fn write16(&mut self, addr: u16, val: u16) -> Result<(), TlbError> {
        if addr % 2 == 1 {
            self.report_bad_addr_error(addr);
            return Ok(());
        }

        if self.ipl_mode && addr >= 0xe000 {
            return Err(TlbError::ReadOnly);
        }

        if self.mmu_mode & !self.priv_sig.get_priv_flag() {
            // TODO 適切なエラー処理を実装
            let mut entry = self.v_addr_to_entry(addr).unwrap();

            if !entry.is_writable() {
                self.report_mem_vio_error(addr);
                return Ok(());
            }

            entry.set_reference_flag();
            entry.set_dirty_flag();
            let addr = (entry.get_frame() as u16) << 8 | (addr & 0x00ff);
        }
        self.memory.write16(addr, val);
        Ok(())
    }

    pub fn fetch(&mut self, pc: u16) -> u16 {
        if pc % 2 == 1 {
            self.report_bad_addr_error(pc);
            return 0;
        }
        if self.mmu_mode && !self.priv_sig.get_priv_flag() {
            // TODO 適切なエラー処理を実装
            let mut entry = self.v_addr_to_entry(pc).unwrap();
            if !entry.is_executable() {
                self.report_mem_vio_error(pc);
                return 0;
            }

            entry.set_reference_flag();
            let pc = (entry.get_frame() as u16) << 8 | (pc & 0x00ff);
        }
        return self.memory.fetch(pc);
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
        self.intr_sig.interrupt(Interrupt::EXCP_MEMORY_ERROR)
    }

    fn report_bad_addr_error(&mut self, addr: u16) {
        self.err_addr = addr;
        self.err_cause |= ERROR_CAUSE_BAD_ADDRESS;
        self.intr_sig.interrupt(Interrupt::EXCP_MEMORY_ERROR)
    }

    fn report_tlb_miss_error(&mut self, page: u8) {
        self.tlb_miss_page = page;
        self.intr_sig.interrupt(Interrupt::EXCP_TLB_MISS);
    }
}
