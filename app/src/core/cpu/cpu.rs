use crate::core::cpu::alu::Alu;
use crate::core::cpu::consts::addr_mode::addr_mode;
use crate::core::cpu::consts::flags::flags;
use crate::core::cpu::consts::opcode::{jmp, opcode};
use crate::core::cpu::consts::reg_num::RegNum;
use crate::core::cpu::instruction::Instruction;
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::error::tlb_error::TlbError;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::io::io_host_controller::IOHostController;
use crate::core::io::io_map_addr::IOMapAddr;
use crate::core::memory::mmu::Mmu;
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::consts::INTERRUPT_VECTOR;

#[derive(PartialEq, Clone)]
pub struct Cpu {
    memory: Rc<RefCell<Mmu>>,
    register: Rc<RefCell<Register>>,
    psw: Rc<RefCell<Psw>>,
    intr_host: Rc<RefCell<IntrController>>,
    io_host: Rc<RefCell<IOHostController>>,
    alu: Alu,
    is_halt: bool,
}

impl Cpu {
    pub fn new(
        memory: Rc<RefCell<Mmu>>,
        register: Rc<RefCell<Register>>,
        psw: Rc<RefCell<Psw>>,
        intr_host: Rc<RefCell<IntrController>>,
        io_host: Rc<RefCell<IOHostController>>,
    ) -> Self {
        let alu = Alu::new(Rc::clone(&intr_host));
        Self {
            memory,
            register,
            psw,
            intr_host,
            io_host: Rc::clone(&io_host),
            alu,
            is_halt: false,
        }
    }

    pub fn run(&mut self) -> Option<Instruction> {
        if self.is_halt {
            return None;
        }

        if self.psw.borrow().check_flag(flags::ENABLE_INTR)
            || self.intr_host.borrow_mut().is_exception_occurred()
        {
            let intr_num = self.intr_host.borrow_mut().check_intr_num();
            if intr_num.is_some() {
                if let Err(_) = self.handle_interrupt(intr_num.unwrap()) {
                    return None;
                };
            }
        }

        let inst_data = match self.memory.borrow_mut().fetch(self.psw.borrow().get_pc()) {
            Ok(inst) => inst,
            Err(_) => {
                return None;
            }
        };
        let mut inst = self.decode(inst_data);
        inst.ea = match self.calc_effective_address(inst.addr_mode.clone(), inst.rx) {
            Ok(ea) => ea,
            Err(_) => {
                return None;
            }
        };
        // if !self.memory.borrow().is_ipl_mode() {
        //     gloo::console::log!(&format!("inst {:?}\nPC {:02X}\nSP {:02X}", inst, self.psw.borrow().get_pc(), self.register.borrow().read(13)));
        // }
        if let Err(_) = self.execute_instruction(inst.clone()) {
            return None;
        };
        Some(inst)
    }

    fn handle_interrupt(&self, intr_num: u8) -> Result<(), TlbError> {
        let tmp = self.psw.borrow().get_flag();
        self.psw.borrow_mut().set_priv_flag(true);
        self.psw
            .borrow_mut()
            .set_flag((tmp & !(flags::ENABLE_INTR)) | flags::PRIV);
        self.push_val(self.psw.borrow().get_pc())?;
        self.push_val(tmp)?;
        let addr = self
            .memory
            .borrow_mut()
            .read16(INTERRUPT_VECTOR + (intr_num * 2) as u16)?;

        self.psw.borrow_mut().jump(addr);
        Ok(())
    }

    fn decode(&self, inst_data: u16) -> Instruction {
        Instruction {
            opcode: (inst_data >> 11) as u8,
            addr_mode: ((inst_data >> 8)) as u8 & 0x07,
            rd: ((inst_data >> 4) & 0x000f) as u8,
            rx: ((inst_data) & 0x000f) as u8,
            ea: 0,
        }
    }

    fn calc_effective_address(&self, addr_mode: u8, rx: u8) -> Result<u16, TlbError> {
        let data = match self
            .memory
            .borrow_mut()
            .read16(self.psw.borrow().get_pc() + 2)
        {
            Ok(data) => data,
            Err(_) => {
                return Err(TlbError::TlbMiss);
            }
        };
        return Ok(match addr_mode {
            addr_mode::DIRECT => data,
            addr_mode::INDEXED => data + self.read_reg(rx),
            addr_mode::FP_RELATIVE => {
                (self.read_reg(RegNum::FP as u8) as i16 + (self.ext_signed_int_4(rx) * 2)) as u16
            }
            addr_mode::REG_INDIRECT | addr_mode::BYTE_REG_INDIRECT => self.read_reg(rx),
            _ => 0u16,
        });
    }

    fn execute_instruction(&mut self, inst: Instruction) -> Result<(), TlbError> {
        match inst.opcode {
            opcode::NOP => {
                self.psw.borrow_mut().next_pc();
                return Ok(());
            }
            opcode::LD => self.instr_ld(inst),
            opcode::ST => self.instr_st(inst),
            opcode::ADD
            | opcode::SUB
            | opcode::CMP
            | opcode::AND
            | opcode::OR
            | opcode::XOR
            | opcode::ADDS
            | opcode::MUL
            | opcode::DIV
            | opcode::MOD
            | opcode::SHLA
            | opcode::SHLL
            | opcode::SHRA
            | opcode::SHRL => self.instr_calc(inst),
            opcode::JMP => {
                self.instr_jmp(inst);
                return Ok(());
            }
            opcode::CALL => self.instr_call(inst),
            opcode::IN => {
                self.instr_in(inst);
                return Ok(());
            }
            opcode::OUT => {
                self.instr_out(inst);
                return Ok(());
            }
            opcode::PUSH_POP => self.instr_push_pop(inst),
            opcode::RET_RETI => self.instr_return(inst),
            opcode::SVC => {
                self.instr_svc(inst);
                return Ok(());
            }
            opcode::HALT => {
                self.instr_halt(inst);
                return Ok(());
            }
            _ => {
                return Ok(());
            }
        }
    }

    fn instr_ld(&self, inst: Instruction) -> Result<(), TlbError> {
        let data = self.load_operand(inst.addr_mode.clone(), inst.rx, inst.ea)?;
        self.write_reg(inst.rd, data);
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
        Ok(())
    }

    fn instr_st(&self, inst: Instruction) -> Result<(), TlbError> {
        let data = self.read_reg(inst.rd);
        if inst.addr_mode == addr_mode::BYTE_REG_INDIRECT {
            if let Err(e) = self
                .memory
                .borrow_mut()
                .write8(inst.ea, (0x00ff & data) as u8)
            {
                return Err(e);
            };
        } else {
            if let Err(e) = self.memory.borrow_mut().write16(inst.ea, data) {
                return Err(e);
            };
        }
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
        Ok(())
    }

    fn instr_calc(&mut self, inst: Instruction) -> Result<(), TlbError> {
        let v1 = self.read_reg(inst.rd);
        let v2 = self.load_operand(inst.addr_mode, inst.rx, inst.ea)?;
        let ans = self.alu.calc(inst.opcode, v1, v2);
        self.change_flag(inst.opcode, ans, v1, v2);

        if inst.opcode != opcode::CMP {
            self.write_reg(inst.rd, (ans & 0xffff) as u16);
        }
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
        Ok(())
    }

    fn instr_jmp(&self, inst: Instruction) {
        let z_flag = self.psw.borrow().check_flag(flags::ZERO);
        let c_flag = self.psw.borrow().check_flag(flags::CARRY);
        let s_flag = self.psw.borrow().check_flag(flags::SIGN);
        let v_flag = self.psw.borrow().check_flag(flags::OVERFLOW);

        match inst.rd {
            jmp::JZ => {
                if z_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JC => {
                if c_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JM => {
                if s_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JO => {
                if v_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JGT => {
                if !(z_flag || (!s_flag && v_flag) || (s_flag && !v_flag)) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JGE => {
                if !((!s_flag && v_flag) || (s_flag && !v_flag)) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JLE => {
                if z_flag || (!s_flag && v_flag) || (s_flag && !v_flag) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JLT => {
                if (!s_flag && v_flag) || (s_flag && !v_flag) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JNZ => {
                if !z_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JNC => {
                if !c_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JNM => {
                if !s_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JNO => {
                if !v_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JHI => {
                if !(z_flag || c_flag) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JLS => {
                if z_flag || c_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            jmp::JMP => {
                self.psw.borrow_mut().jump(inst.ea);
                return;
            }
            _ => {}
        }
        self.psw.borrow_mut().next_pc();
        self.psw.borrow_mut().next_pc();
    }

    fn instr_call(&self, inst: Instruction) -> Result<(), TlbError> {
        self.push_val(self.psw.borrow().get_pc() + 4)?;
        self.psw.borrow_mut().jump(inst.ea);
        Ok(())
    }

    fn instr_push_pop(&self, inst: Instruction) -> Result<(), TlbError> {
        if inst.addr_mode == addr_mode::DIRECT {
            self.push_val(self.read_reg(inst.rd))?;
        } else if inst.addr_mode == addr_mode::REG_TO_REG {
            let pop_val = self.pop_val()?;
            self.write_reg(inst.rd, pop_val);
        }
        self.psw.borrow_mut().next_pc();
        Ok(())
    }

    fn instr_return(&self, inst: Instruction) -> Result<(), TlbError> {
        if inst.addr_mode == addr_mode::DIRECT {
            let pop_val = self.pop_val()?;
            self.psw.borrow_mut().jump(pop_val);
        } else if inst.addr_mode == addr_mode::REG_TO_REG {
            let f = self.pop_val()?;
            let pc = self.pop_val()?;
            self.psw.borrow_mut().set_flag(f);
            self.psw.borrow_mut().jump(pc);
        }
        Ok(())
    }

    fn instr_in(&self, inst: Instruction) {
        if self.psw.borrow().check_flag(flags::PRIV as u16)
            || self.psw.borrow().check_flag(flags::IO_PRIV as u16)
        {
            self.write_reg(
                inst.rd,
                self.io_host
                    .borrow_mut()
                    .input(IOMapAddr::from_u8(inst.ea as u8)),
            );
        } else {
            self.intr_host
                .borrow_mut()
                .interrupt(Interrupt::EXCP_PRIV_ERROR);
        }
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
    }

    fn instr_out(&self, inst: Instruction) {
        if self.psw.borrow().check_flag(flags::PRIV)
            || self.psw.borrow().check_flag(flags::IO_PRIV)
        {
            self.io_host
                .borrow_mut()
                .output(IOMapAddr::from_u8(inst.ea as u8), self.read_reg(inst.rd));
        } else {
            self.intr_host
                .borrow_mut()
                .interrupt(Interrupt::EXCP_PRIV_ERROR);
        }
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
    }

    fn instr_svc(&self, _inst: Instruction) {
        self.intr_host.borrow_mut().interrupt(Interrupt::EXCP_SVC);
        self.psw.borrow_mut().next_pc();
    }

    fn instr_halt(&mut self, _inst: Instruction) {
        if self.psw.borrow().check_flag(flags::PRIV) {
            self.is_halt = true;
        } else {
            self.intr_host
                .borrow_mut()
                .interrupt(Interrupt::EXCP_PRIV_ERROR);
        }
        self.psw.borrow_mut().next_pc();
    }

    fn read_reg(&self, num: u8) -> u16 {
        if num == RegNum::FLAG as u8 {
            return self.psw.borrow().get_flag();
        } else {
            self.register.borrow().read(num)
        }
    }

    fn change_flag(&self, op: u8, ans: u32, v1: u16, v2: u16) {
        let ans_msb = (ans & 0x8000) as u16;
        let v1_msb = v1 & 0x8000;
        let v2_msb = v2 & 0x8000;

        let mut flags = self.psw.borrow().get_flag() & 0xf0;
        if op == opcode::ADD {
            if v1_msb == v2_msb && ans_msb != v1_msb {
                flags |= flags::OVERFLOW;
            }
        } else if op == opcode::SUB || op == opcode::CMP {
            if v1_msb != v2_msb && ans_msb != v1_msb {
                flags |= flags::OVERFLOW;
            }
        }

        if opcode::ADD <= op && op <= opcode::CMP {
            if (ans & 0x10000) != 0 {
                flags |= flags::CARRY;
            }
        } else if opcode::SHLA <= op && op <= opcode::SHRL && v2 == 1 {
            if (ans & 0x10000) != 0 {
                flags |= flags::CARRY;
            }
        }

        if ans_msb != 0 {
            flags |= flags::SIGN;
        }

        if (ans & 0xffff) == 0 {
            flags |= flags::ZERO;
        }
        self.psw.borrow_mut().set_flag(flags);
    }

    fn load_operand(&self, addr_mode: u8, rx: u8, dsp: u16) -> Result<u16, TlbError> {
        return match addr_mode {
            addr_mode::DIRECT
            | addr_mode::INDEXED
            | addr_mode::FP_RELATIVE
            | addr_mode::REG_INDIRECT => self.memory.borrow_mut().read16(dsp),
            addr_mode::IMMEDIATE => self
                .memory
                .borrow_mut()
                .read16(self.psw.borrow().get_pc() + 2),
            addr_mode::REG_TO_REG => Ok(self.read_reg(rx)),
            addr_mode::SHORT_IMMEDIATE => Ok(self.ext_signed_int_4(rx) as u16),
            addr_mode::BYTE_REG_INDIRECT => Ok(self.memory.borrow_mut().read8(dsp)? as u16),
            _ => Ok(0),
        };
    }

    fn is_two_word_instruction(&self, addr_mode: u8) -> bool {
        // return AddrMode::DIRECT <= addr_mode && addr_mode <= AddrMode::IMMEDIATE;
        return addr_mode == addr_mode::DIRECT
            || addr_mode == addr_mode::INDEXED
            || addr_mode == addr_mode::IMMEDIATE;
    }

    fn write_reg(&self, reg_num: u8, val: u16) {
        if reg_num == RegNum::FLAG as u8 {
            self.psw.borrow_mut().set_flag(val);
        } else {
            self.register.borrow_mut().write(reg_num, val);
        }
    }

    fn ext_signed_int_4(&self, val: u8) -> i16 {
        let mut val = val as u16;
        if (val & 0x0008) != 0 {
            val |= 0xfff0;
        }
        val as i16
    }

    fn push_val(&self, val: u16) -> Result<(), TlbError> {
        self
            .memory
            .borrow_mut()
            .write16(self.read_reg(RegNum::SP as u8) - 2, val)?;
        self.write_reg(RegNum::SP as u8, self.read_reg(RegNum::SP as u8) - 2);
        Ok(())
    }

    fn pop_val(&self) -> Result<u16, TlbError> {
        let val = self
            .memory
            .borrow_mut()
            .read16(self.read_reg(RegNum::SP as u8));
        self.write_reg(RegNum::SP as u8, self.read_reg(RegNum::SP as u8) + 2);
        val
    }

    pub fn reset(&mut self) {
        self.is_halt = false;
    }

    pub fn is_halt(&self) -> bool {
        self.is_halt
    }
}
