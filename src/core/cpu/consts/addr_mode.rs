#[allow(dead_code)]
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
