#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

impl AddrMode {
    pub fn from_index(index: u8) -> Option<AddrMode> {
        match index {
            0 => Some(AddrMode::DIRECT),
            1 => Some(AddrMode::INDEXED),
            2 => Some(AddrMode::IMMEDIATE),
            3 => Some(AddrMode::FP_RELATIVE),
            4 => Some(AddrMode::REG_TO_REG),
            5 => Some(AddrMode::SHORT_IMMDIATE),
            6 => Some(AddrMode::REG_INDIRECT),
            7 => Some(AddrMode::BYTE_REG_INDIRECT),
            _ => None,
        }
    }
}
