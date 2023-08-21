#[derive(Debug, Clone)]
pub enum AddrMode {
    DIRECT,
    INDEXED,
    IMMEDIATE,
    FP_RELATIVE,
    REG_TO_REG,
    SHORT_IMMDIATE,
    REG_INDIRECT,
    BYTE_REG_INDIRECT,
}

// impl AddrMode {
//     pub fn is_two_word_instruction(&self) -> bool {
//         AddrMode::DIRECT as i32 <= self as i32 && self as i32 <= AddrMode::IMMEDIATE as i32
//     }
// }

// impl From<i16> for AddrMode {
//     fn from(value: i16) -> Self {
//         match (value >> 8) & 0x07 {
//             0x00 => AddrMode::DIRECT,
//             0x01 => AddrMode::INDEXED,
//             0x02 => AddrMode::IMMEDIATE,
//             0x03 => AddrMode::FP_RELATIVE,
//             0x04 => AddrMode::REG_TO_REG,
//             0x05 => AddrMode::SHORT_IMMDIATE,
//             0x06 => AddrMode::REG_INDIRECT,
//             0x07 => AddrMode::BYTE_REG_INDIRECT,
//             _ => ()
//         }
//     }
// }
