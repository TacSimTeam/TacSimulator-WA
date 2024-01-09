#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Flags {
    ENABLE_INTR = 0x0080,
    PRIV = 0x0040,
    IO_PRIV = 0x0020,
    OVERFLOW = 0x0008,
    CARRY = 0x0004,
    SIGN = 0x0002,
    ZERO = 0x0001,
}
