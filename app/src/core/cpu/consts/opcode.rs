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
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
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
    pub fn from_index(index: u8) -> Option<Self> {
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
