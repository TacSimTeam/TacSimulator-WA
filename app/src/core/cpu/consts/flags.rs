#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Flags {
    ENABLE_INTR = 0x80,
    PRIV = 0x40,
    IO_PRIV = 0x20,
    OVERFLOW = 0x08,
    CARRY = 0x04,
    SIGN = 0x02,
    ZERO = 0x01,
}
