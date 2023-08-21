pub const MEMORY_SIZE: i32 = 64 * 1024;
pub const SECTOR_SIZE: u32 = 512;
pub const TLB_ENTRY_SIZE:u32 = 16;

pub const ERROR_CAUSE_MEMORY_VIOLATION: u8 = 0x01;
pub const ERROR_CAUSE_BAD_ADDRESS: u8 = 0x02;