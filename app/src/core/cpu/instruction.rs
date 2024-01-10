#[derive(Clone, Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub addr_mode: u8,
    pub rd: u8,
    pub rx: u8,
    pub ea: u16,
}
