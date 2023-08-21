use crate::core::cpu::consts::addr_mode::AddrMode;
use crate::core::cpu::consts::opcode::OPCode;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub opcode: OPCode,
    pub addr_mode: AddrMode,
    pub rd: u8,
    pub rx: u8,
    pub ea: u16,
}
