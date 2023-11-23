use crate::core::cpu::alu::Alu;
use crate::core::cpu::consts::addr_mode::AddrMode;
use crate::core::cpu::consts::flags::Flags;
use crate::core::cpu::consts::opcode::{OPCode, JMP};
use crate::core::cpu::consts::reg_num::RegNum;
use crate::core::cpu::instruction::Instruction;
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::io::io_host_controller::IOHostController;
use crate::core::io::io_map_addr::IOMapAddr;
use crate::core::memory::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

const INTERRUPT_VECTOR: u16 = 0xffe0;
#[derive(PartialEq, Clone)]
pub struct Cpu {
    memory: Rc<RefCell<Memory>>,
    register: Rc<RefCell<Register>>,
    psw: Rc<RefCell<Psw>>,
    intr_host: Rc<RefCell<IntrController>>,
    io_host: Rc<RefCell<IOHostController>>,
    alu: Alu,
    is_halt: bool,
}

impl Cpu {
    pub fn new(
        memory: Rc<RefCell<Memory>>,
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
        gloo::console::log!("core/cpu/cpu run");
        // TODO TLBミスとかどうやって実装する?Resultで返してあげるのが丸い気がする
        if self.is_halt {
            return None;
        }
        {
            let mut intr_host = self.intr_host.borrow_mut();
            if self.psw.borrow().check_flag(Flags::ENABLE_INTR as u8)
                || intr_host.is_exception_occurred()
            {
                let intr_num = intr_host.check_intr_num();
                if intr_num.is_some() {
                    self.handle_interrupt(intr_num.unwrap());
                }
            }
        }
        let inst_data = self.memory.borrow().fetch(self.psw.borrow().get_pc());
        gloo::console::log!(&format!("read instruction data: {:08b}", inst_data));
        let mut inst = self.decode(inst_data);
        gloo::console::log!(&format!("decoded instruction: {:?}", inst.clone()));
        inst.ea = self.calc_effective_address(inst.addr_mode.clone(), inst.rx);

        self.execute_instruction(inst.clone());
        gloo::console::log!("instruction executed!");
        Some(inst)
    }

    fn handle_interrupt(&self, intr_num: u8) {
        let mut psw = self.psw.borrow_mut();
        let tmp = psw.get_flag();
        psw.set_flag((tmp & !(Flags::ENABLE_INTR as u8)) | Flags::PRIV as u8);

        self.push_val(self.psw.borrow().get_pc());
        self.psw.borrow_mut().jump(
            self.memory
                .borrow()
                .read16(INTERRUPT_VECTOR + (intr_num * 2) as u16),
        )
    }

    fn decode(&self, inst_data: u16) -> Instruction {
        let opcode = OPCode::from_index((inst_data >> 11) as u8).unwrap();
        let addr_mode = AddrMode::from_index(((inst_data >> 8) as u8) & 0x07).unwrap();
        Instruction {
            opcode,
            addr_mode,
            rd: ((inst_data >> 4) & 0x000f) as u8,
            rx: (inst_data as u8) & 0x0f,
            ea: 0,
        }
    }

    fn calc_effective_address(&self, addr_mode: AddrMode, rx: u8) -> u16 {
        let data = self.memory.borrow().read16(self.psw.borrow().get_pc() + 2);
        return match addr_mode {
            AddrMode::DIRECT => data,
            AddrMode::INDEXED => data + self.read_reg(rx),
            AddrMode::FP_RELATIVE => {
                (self.read_reg(RegNum::FP as u8) + (self.ext_signed_int_4(rx) * 2) as u16) & 0xffff
            }
            AddrMode::REG_INDIRECT | AddrMode::BYTE_REG_INDIRECT => self.read_reg(rx),
            _ => 0u16,
        };
    }

    fn execute_instruction(&mut self, inst: Instruction) {
        gloo::console::log!(&format!("execute instruction: {:?}", inst.clone()));
        match inst.opcode {
            OPCode::NOP => self.psw.borrow_mut().next_pc(),
            OPCode::LD => {
                self.instr_ld(inst);
            }
            OPCode::ST => {
                self.instr_st(inst);
            }
            OPCode::ADD
            | OPCode::SUB
            | OPCode::CMP
            | OPCode::AND
            | OPCode::OR
            | OPCode::XOR
            | OPCode::ADDS
            | OPCode::MUL
            | OPCode::DIV
            | OPCode::MOD
            | OPCode::SHLA
            | OPCode::SHLL
            | OPCode::SHRA
            | OPCode::SHRL => {
                self.instr_calc(inst);
            }
            OPCode::JMP => {
                self.instr_jmp(inst);
            }
            OPCode::CALL => {
                self.instr_call(inst);
            }
            OPCode::IN => {
                self.instr_in(inst);
            }
            OPCode::OUT => {
                self.instr_out(inst);
            }
            OPCode::PUSH_POP => {
                self.instr_push_pop(inst);
            }
            OPCode::RET_RETI => {
                self.instr_return(inst);
            }
            OPCode::SVC => {
                self.instr_svc(inst);
            }
            OPCode::HALT => {
                self.instr_halt(inst);
            }
        }
    }

    fn instr_ld(&self, inst: Instruction) {
        let data = self.load_operand(inst.addr_mode.clone(), inst.rx, inst.ea);
        self.write_reg(inst.rd, data);
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
    }

    fn instr_st(&self, inst: Instruction) {
        let data = self.read_reg(inst.rd);
        if inst.addr_mode == AddrMode::BYTE_REG_INDIRECT {
            self.memory
                .borrow_mut()
                .write8(inst.ea, (0x00ff & data) as u8);
        } else {
            self.memory.borrow_mut().write16(inst.ea, data);
        }
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
    }

    fn instr_calc(&mut self, inst: Instruction) {
        let v1 = self.read_reg(inst.rd) as i16;
        let v2 = self.load_operand(inst.addr_mode, inst.rx, inst.ea) as i16;
        let ans = self.alu.calc(inst.opcode, v1, v2);
        self.change_flag(inst.opcode, ans, v1, v2);

        if inst.opcode != OPCode::CMP {
            self.write_reg(inst.rd, ans as u16);
        }
        self.psw.borrow_mut().next_pc();
        if self.is_two_word_instruction(inst.addr_mode) {
            self.psw.borrow_mut().next_pc();
        }
    }

    fn instr_jmp(&self, inst: Instruction) {
        let z_flag = self.psw.borrow().check_flag(Flags::ZERO as u8);
        let c_flag = self.psw.borrow().check_flag(Flags::CARRY as u8);
        let s_flag = self.psw.borrow().check_flag(Flags::SIGN as u8);
        let v_flag = self.psw.borrow().check_flag(Flags::OVERFLOW as u8);

        match JMP::from_index(inst.rd) {
            None => {
                return;
            }
            Some(JMP::JZ) => {
                if z_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JC) => {
                if c_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JM) => {
                if s_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JO) => {
                if v_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JGT) => {
                if !(z_flag || (!s_flag && v_flag) || (s_flag && !v_flag)) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JGE) => {
                if !((!s_flag && v_flag) || (s_flag && !v_flag)) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JLE) => {
                if z_flag || (!s_flag && v_flag) || (s_flag && !v_flag) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JLT) => {
                if (!s_flag && v_flag) || (s_flag && !v_flag) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JNZ) => {
                if !z_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JNC) => {
                if !c_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JNM) => {
                if !s_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JNO) => {
                if !v_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JHI) => {
                if !(z_flag || c_flag) {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JLS) => {
                if z_flag || c_flag {
                    self.psw.borrow_mut().jump(inst.ea);
                    return;
                }
            }
            Some(JMP::JMP) => {
                self.psw.borrow_mut().jump(inst.ea);
                return;
            }
        }
        self.psw.borrow_mut().next_pc();
        self.psw.borrow_mut().next_pc();
    }

    fn instr_call(&self, inst: Instruction) {
        self.push_val(self.psw.borrow().get_pc() + 4);
        self.psw.borrow_mut().jump(inst.ea);
    }

    fn instr_push_pop(&self, inst: Instruction) {
        if inst.addr_mode as u8 == 0x00 {
            self.push_val(self.read_reg(inst.rd));
        } else if inst.addr_mode == AddrMode::REG_TO_REG {
            // アドレスモードが0x04になるのはREG_TO_REG
            self.write_reg(inst.rd, self.pop_val());
        }
        self.psw.borrow_mut().next_pc();
    }

    fn instr_return(&self, inst: Instruction) {
        gloo::console::log!("src/core/cpu/cpu instr_return");
        if inst.addr_mode as u8 == 0x00 {
            let addr = self.pop_val();
            self.psw.borrow_mut().jump(addr);
        } else if inst.addr_mode as u8 == 0x04 {
            let f = self.pop_val();
            let pc = self.pop_val();
            self.psw.borrow_mut().set_flag(f as u8);
            self.psw.borrow_mut().jump(pc);
        }
    }

    fn instr_in(&self, inst: Instruction) {
        if self.psw.borrow().check_flag(Flags::PRIV as u8)
            || self.psw.borrow().check_flag(Flags::IO_PRIV as u8)
        {
            self.write_reg(
                inst.rd,
                self.io_host
                    .borrow_mut()
                    .input(IOMapAddr::from_u8(inst.ea as u8).unwrap()),
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
        if self.psw.borrow().check_flag(Flags::PRIV as u8)
            || self.psw.borrow().check_flag(Flags::IO_PRIV as u8)
        {
            self.io_host.borrow_mut().output(
                IOMapAddr::from_u8(inst.ea as u8).unwrap(),
                self.read_reg(inst.rd),
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

    fn instr_svc(&self, _inst: Instruction) {
        self.intr_host.borrow_mut().interrupt(Interrupt::EXCP_SVC);
        self.psw.borrow_mut().next_pc();
    }

    fn instr_halt(&mut self, _inst: Instruction) {
        if self.psw.borrow().check_flag(Flags::PRIV as u8) {
            self.is_halt = true;
        } else {
            self.intr_host
                .borrow_mut()
                .interrupt(Interrupt::EXCP_PRIV_ERROR);
        }
        self.psw.borrow_mut().next_pc();
    }

    fn read_reg(&self, rx: u8) -> u16 {
        if rx == RegNum::FLAG as u8 {
            return self.psw.borrow().get_flag() as u16;
        } else {
            self.register.borrow().read(rx)
        }
    }

    fn change_flag(&self, op: OPCode, ans: i32, v1: i16, v2: i16) {
        let ans_msb = ans as u16 & 0x8000;
        let v1_msb = v1 as u16 & 0x8000;
        let v2_msb = v2 as u16 & 0x8000;

        let mut flags = self.psw.borrow().get_flag() & 0xf0;
        if op == OPCode::ADD {
            if v1_msb == v2_msb && ans_msb != v1_msb {
                flags |= Flags::OVERFLOW as u8;
            }
        } else if op == OPCode::SUB || op == OPCode::CMP {
            if v1_msb != v2_msb && ans_msb != v1_msb {
                flags |= Flags::OVERFLOW as u8;
            }
        }

        if OPCode::ADD <= op && op <= OPCode::CMP {
            if (ans & 0x10000) != 0 {
                flags |= Flags::CARRY as u8;
            }
        }
        if OPCode::SHLA <= op && op <= OPCode::SHRL && v2 == 1 {
            if (ans & 0x10000) != 0 {
                flags |= Flags::CARRY as u8;
            }
        }

        if ans_msb != 0 {
            flags |= Flags::SIGN as u8;
        }

        if (ans & 0xffff) != 0 {
            flags |= Flags::ZERO as u8;
        }

        self.psw.borrow_mut().set_flag(flags);
    }

    fn load_operand(&self, addr_mode: AddrMode, rx: u8, dsp: u16) -> u16 {
        return match addr_mode {
            AddrMode::DIRECT
            | AddrMode::INDEXED
            | AddrMode::FP_RELATIVE
            | AddrMode::REG_INDIRECT => self.memory.borrow().read16(dsp),
            AddrMode::IMMEDIATE => self.memory.borrow().read16(self.psw.borrow().get_pc() + 2),
            AddrMode::REG_TO_REG => self.read_reg(rx),
            AddrMode::SHORT_IMMDIATE => self.ext_signed_int_4(rx) as u16,
            AddrMode::BYTE_REG_INDIRECT => self.memory.borrow().read8(dsp) as u16,
        };
    }

    fn is_two_word_instruction(&self, addr_mode: AddrMode) -> bool {
        return AddrMode::DIRECT <= addr_mode && addr_mode <= AddrMode::IMMEDIATE;
    }

    fn write_reg(&self, reg_num: u8, val: u16) {
        gloo::console::log!("src/core/cpu/cpu write_reg");
        if reg_num == RegNum::FLAG as u8 {
            self.psw.borrow();
        } else {
            self.register.borrow_mut().write(reg_num, val);
        }
    }

    fn ext_signed_int_4(&self, val: u8) -> i16 {
        let mut val = val as u16;
        if (val & 0x08) != 0 {
            val |= 0xfff0;
        }
        val as i16
    }

    fn push_val(&self, val: u16) {
        gloo::console::log!("src/core/cpu/cpu push_val");
        gloo::console::info!(self.read_reg(RegNum::SP as u8));
        self.memory
            .borrow_mut()
            .write16((self.read_reg(RegNum::SP as u8) - 2), val);
        // TODO 下にスタックが伸びるのにu8だとまずいのでとりあえず+にしておく本来は-2
        self.write_reg(RegNum::SP as u8, self.read_reg(RegNum::SP as u8) - 2);
    }

    fn pop_val(&self) -> u16 {
        gloo::console::log!("src/core/cpu/cpu pop_val");
        gloo::console::info!(self.read_reg(RegNum::SP as u8));
        let val = self
            .memory
            .borrow_mut()
            .read16(self.read_reg(RegNum::SP as u8));
        // TODO pushと同じ理由
        self.write_reg(RegNum::SP as u8, self.read_reg(RegNum::SP as u8) + 2);
        val
    }

    pub fn reset(&mut self) {
        self.is_halt = false;
    }
}
