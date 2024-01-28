pub(crate) mod opcode {
    pub const NOP: u8 = 0x00;
    pub const LD: u8 = 0x01;
    pub const ST: u8 = 0x02;
    pub const ADD: u8 = 0x03;
    pub const SUB: u8 = 0x04;
    pub const CMP: u8 = 0x05;
    pub const AND: u8 = 0x06;
    pub const OR: u8 = 0x07;
    pub const XOR: u8 = 0x08;
    pub const ADDS: u8 = 0x09;
    pub const MUL: u8 = 0x0a;
    pub const DIV: u8 = 0x0b;
    pub const MOD: u8 = 0x0c;
    pub const SHLA: u8 = 0x10;
    pub const SHLL: u8 = 0x11;
    pub const SHRA: u8 = 0x12;
    pub const SHRL: u8 = 0x13;
    pub const JMP: u8 = 0x14;
    pub const CALL: u8 = 0x15;
    pub const IN: u8 = 0x16;
    pub const OUT: u8 = 0x17;
    pub const PUSH_POP: u8 = 0x18;
    pub const RET_RETI: u8 = 0x1a;
    pub const SVC: u8 = 0x1e;
    pub const HALT: u8 = 0x1f;
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::consts::opcode::opcode;

    #[test]
    fn test_opcode_from_u8() {
        let no_instruction: u16 = 0x0000;
        let op = (no_instruction >> 11) as u8;
        assert_eq!(opcode::NOP, op);

        let ld_instruction: u16 = 0x0800;
        let op = (ld_instruction >> 11) as u8;
        assert_eq!(opcode::LD, op);

        let st_instruction: u16 = 0x1000;
        let op = (st_instruction >> 11) as u8;
        assert_eq!(opcode::ST, op);

        let add_instruction: u16 = 0x1800;
        let op = (add_instruction >> 11) as u8;
        assert_eq!(opcode::ADD, op);

        let sub_instruction: u16 = 0x2000;
        let op = (sub_instruction >> 11) as u8;
        assert_eq!(opcode::SUB, op);

        let cmp_instruction: u16 = 0x2800;
        let op = (cmp_instruction >> 11) as u8;
        assert_eq!(opcode::CMP, op);

        let and_instruction: u16 = 0x3000;
        let op = (and_instruction >> 11) as u8;
        assert_eq!(opcode::AND, op);

        let or_instruction: u16 = 0x3800;
        let op = (or_instruction >> 11) as u8;
        assert_eq!(opcode::OR, op);

        let xor_instruction: u16 = 0x4000;
        let op = (xor_instruction >> 11) as u8;
        assert_eq!(opcode::XOR, op);

        let adds_instruction: u16 = 0x4800;
        let op = (adds_instruction >> 11) as u8;
        assert_eq!(opcode::ADDS, op);

        let mul_instruction: u16 = 0x5000;
        let op = (mul_instruction >> 11) as u8;
        assert_eq!(opcode::MUL, op);

        let div_instruction: u16 = 0x5800;
        let op = (div_instruction >> 11) as u8;
        assert_eq!(opcode::DIV, op);

        let mod_instruction: u16 = 0x6000;
        let op = (mod_instruction >> 11) as u8;
        assert_eq!(opcode::MOD, op);

        let shla_instruction: u16 = 0x8000;
        let op = (shla_instruction >> 11) as u8;
        assert_eq!(opcode::SHLA, op);

        let shll_instruction: u16 = 0x8800;
        let op = (shll_instruction >> 11) as u8;
        assert_eq!(opcode::SHLL, op);

        let shra_instruction: u16 = 0x9000;
        let op = (shra_instruction >> 11) as u8;
        assert_eq!(opcode::SHRA, op);

        let shrl_instruction: u16 = 0x9800;
        let op = (shrl_instruction >> 11) as u8;
        assert_eq!(opcode::SHRL, op);

        let jmp_instruction: u16 = 0xa000;
        let op = (jmp_instruction >> 11) as u8;
        assert_eq!(opcode::JMP, op);

        let call_instruction: u16 = 0xa800;
        let op = (call_instruction >> 11) as u8;
        assert_eq!(opcode::CALL, op);

        let in_instruction: u16 = 0xb000;
        let op = (in_instruction >> 11) as u8;
        assert_eq!(opcode::IN, op);

        let out_instruction: u16 = 0xb800;
        let op = (out_instruction >> 11) as u8;
        assert_eq!(opcode::OUT, op);

        let push_instruction: u16 = 0xc000;
        let op = (push_instruction >> 11) as u8;
        assert_eq!(opcode::PUSH_POP, op);

        let ret_instruction: u16 = 0xd000;
        let op = (ret_instruction >> 11) as u8;
        assert_eq!(opcode::RET_RETI, op);

        let svc_instruction: u16 = 0xf000;
        let op = (svc_instruction >> 11) as u8;
        assert_eq!(opcode::SVC, op);

        let halt_instruction: u16 = 0xff00;
        let op = (halt_instruction >> 11) as u8;
        assert_eq!(opcode::HALT, op);
    }
}

pub(crate) mod jmp {
    pub const JZ: u8 = 0;
    pub const JC: u8 = 1;
    pub const JM: u8 = 2;
    pub const JO: u8 = 3;
    pub const JGT: u8 = 4;
    pub const JGE: u8 = 5;
    pub const JLE: u8 = 6;
    pub const JLT: u8 = 7;
    pub const JNZ: u8 = 8;
    pub const JNC: u8 = 9;
    pub const JNM: u8 = 10;
    pub const JNO: u8 = 11;
    pub const JHI: u8 = 12;
    pub const JLS: u8 = 14;
    pub const JMP: u8 = 15;
}
