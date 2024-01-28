pub(crate) mod addr_mode {
    pub const DIRECT: u8 = 0;
    pub const INDEXED: u8 = 1;
    pub const IMMEDIATE: u8 = 2;
    pub const FP_RELATIVE: u8 = 3;
    pub const REG_TO_REG: u8 = 4;
    pub const SHORT_IMMEDIATE: u8 = 5;
    pub const REG_INDIRECT: u8 = 6;
    pub const BYTE_REG_INDIRECT: u8 = 7;
}
