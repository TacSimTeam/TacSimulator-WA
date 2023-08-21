#[derive(Debug, Clone)]
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

// impl From<i16> for OPCode {
//     fn from(value: i16) -> Self {
//         match (value >> 11) as i8 {
//             0x00 => OPCode::NOP,
//             0x01 => OPCode::LD,
//             0x02 => OPCode::ST,
//             0x03 => OPCode::ADD,
//             0x04 => OPCode::SUB,
//             0x05 => OPCode::CMP,
//             0x06 => OPCode::AND,
//             0x07 => OPCode::OR,
//             0x08 => OPCode::XOR,
//             0x09 => OPCode::ADDS,
//             0x0a => OPCode::MUL,
//             0x0b => OPCode::DIV,
//             0x0c => OPCode::MOD,
//             0x10 => OPCode::SHLA,
//             0x11 => OPCode::SHLL,
//             0x12 => OPCode::SHRA,
//             0x13 => OPCode::SHRL,
//             0x14 => OPCode::JMP,
//             0x15 => OPCode::CALL,
//             0x16 => OPCode::IN,
//             0x17 => OPCode::OUT,
//             0x18 => OPCode::PUSH_POP,
//             0x1a => OPCode::RET_RETI,
//             0x1e => OPCode::SVC,
//             0x1f => OPCode::HALT,
//             _ => {}
//         }
//     }
// }
//
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
