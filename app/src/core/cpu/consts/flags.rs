pub(crate) mod flags {
    pub const ENABLE_INTR: u16 = 0x0080;
    pub const PRIV: u16 = 0x0040;
    pub const IO_PRIV: u16 = 0x0020;
    pub const OVERFLOW: u16 = 0x0008;
    pub const CARRY: u16 = 0x0004;
    pub const SIGN: u16 = 0x0002;
    pub const ZERO: u16 = 0x0001;
}
