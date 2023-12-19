#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum OPCode {
    NOP = 0x00,
    LD = 0x01,
    ST = 0x02,
    ADD = 0x03,
    SUB = 0x04,
    CMP = 0x05,
    AND = 0x06,
    OR = 0x07,
    XOR = 0x08,
    ADDS = 0x09,
    MUL = 0x0a,
    DIV = 0x0b,
    MOD = 0x0c,
    SHLA = 0x10,
    SHLL = 0x11,
    SHRA = 0x12,
    SHRL = 0x13,

    JMP = 0x14,

    CALL = 0x15,
    IN = 0x16,
    OUT = 0x17,

    PUSH_POP = 0x18,

    RET_RETI = 0x1a,
    SVC = 0x1e,
    HALT = 0x1f,
}

impl OPCode {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x00 => Some(OPCode::NOP),
            0x01 => Some(OPCode::LD),
            0x02 => Some(OPCode::ST),
            0x03 => Some(OPCode::ADD),
            0x04 => Some(OPCode::SUB),
            0x05 => Some(OPCode::CMP),
            0x06 => Some(OPCode::AND),
            0x07 => Some(OPCode::OR),
            0x08 => Some(OPCode::XOR),
            0x09 => Some(OPCode::ADDS),
            0x0a => Some(OPCode::MUL),
            0x0b => Some(OPCode::DIV),
            0x0c => Some(OPCode::MOD),
            0x10 => Some(OPCode::SHLA),
            0x11 => Some(OPCode::SHLL),
            0x12 => Some(OPCode::SHRA),
            0x13 => Some(OPCode::SHRL),
            0x14 => Some(OPCode::JMP),
            0x15 => Some(OPCode::CALL),
            0x16 => Some(OPCode::IN),
            0x17 => Some(OPCode::OUT),
            0x18 => Some(OPCode::PUSH_POP),
            0x1a => Some(OPCode::RET_RETI),
            0x1e => Some(OPCode::SVC),
            0x1f => Some(OPCode::HALT),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::consts::opcode::OPCode;

    #[test]
    fn test_opcode_from_u8() {
        let no_instruction: u16 = 0x0000;
        let op = OPCode::from_u8((no_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::NOP), op);

        let ld_instruction: u16 = 0x0800;
        let op = OPCode::from_u8((ld_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::LD), op);

        let st_instruction: u16 = 0x1000;
        let op = OPCode::from_u8((st_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::ST), op);

        let add_instruction: u16 = 0x1800;
        let op = OPCode::from_u8((add_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::ADD), op);

        let sub_instruction: u16 = 0x2000;
        let op = OPCode::from_u8((sub_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::SUB), op);

        let cmp_instruction: u16 = 0x2800;
        let op = OPCode::from_u8((cmp_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::CMP), op);

        let and_instruction: u16 = 0x3000;
        let op = OPCode::from_u8((and_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::AND), op);

        let or_instruction: u16 = 0x3800;
        let op = OPCode::from_u8((or_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::OR), op);

        let xor_instruction: u16 = 0x4000;
        let op = OPCode::from_u8((xor_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::XOR), op);

        let adds_instruction: u16 = 0x4800;
        let op = OPCode::from_u8((adds_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::ADDS), op);

        let mul_instruction: u16 = 0x5000;
        let op = OPCode::from_u8((mul_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::MUL), op);

        let div_instruction: u16 = 0x5800;
        let op = OPCode::from_u8((div_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::DIV), op);

        let mod_instruction: u16 = 0x6000;
        let op = OPCode::from_u8((mod_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::MOD), op);

        let shla_instruction: u16 = 0x8000;
        let op = OPCode::from_u8((shla_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::SHLA), op);

        let shll_instruction: u16 = 0x8800;
        let op = OPCode::from_u8((shll_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::SHLL), op);

        let shra_instruction: u16 = 0x9000;
        let op = OPCode::from_u8((shra_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::SHRA), op);

        let shrl_instruction: u16 = 0x9800;
        let op = OPCode::from_u8((shrl_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::SHRL), op);

        let jmp_instruction: u16 = 0xa000;
        let op = OPCode::from_u8((jmp_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::JMP), op);

        let call_instruction: u16 = 0xa800;
        let op = OPCode::from_u8((call_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::CALL), op);

        let in_instruction: u16 = 0xb000;
        let op = OPCode::from_u8((in_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::IN), op);

        let out_instruction: u16 = 0xb800;
        let op = OPCode::from_u8((out_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::OUT), op);

        let push_instruction: u16 = 0xc000;
        let op = OPCode::from_u8((push_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::PUSH_POP), op);

        let ret_instruction: u16 = 0xd000;
        let op = OPCode::from_u8((ret_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::RET_RETI), op);

        let svc_instruction: u16 = 0xf000;
        let op = OPCode::from_u8((svc_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::SVC), op);

        let halt_instruction: u16 = 0xff00;
        let op = OPCode::from_u8((halt_instruction >> 11) as u8);
        assert_eq!(Some(OPCode::HALT), op);
    }
}

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Debug)]
pub enum JMP {
    JZ,
    JC,
    JM,
    JO,
    JGT,
    JGE,
    JLE,
    JLT,
    JNZ,
    JNC,
    JNM,
    JNO,
    JHI,
    JLS,
    JMP,
}

impl JMP {
    pub fn from_u8(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::JZ),
            1 => Some(Self::JC),
            2 => Some(Self::JM),
            3 => Some(Self::JO),
            4 => Some(Self::JGT),
            5 => Some(Self::JGE),
            6 => Some(Self::JLE),
            7 => Some(Self::JLT),
            8 => Some(Self::JNZ),
            9 => Some(Self::JNC),
            10 => Some(Self::JNM),
            11 => Some(Self::JNO),
            12 => Some(Self::JHI),
            14 => Some(Self::JLS),
            15 => Some(Self::JMP),
            _ => None,
        }
    }
}
