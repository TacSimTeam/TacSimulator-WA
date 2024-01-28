use crate::core::consts::MEMORY_SIZE;

#[derive(Clone, PartialOrd, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Memory {
    mem: Vec<u8>,
    size: i32,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: vec![0; MEMORY_SIZE as usize],
            size: MEMORY_SIZE,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }

    pub fn read16(&self, addr: u16) -> u16 {
        ((self.read8(addr) as u32) << 8 | (self.read8(addr + 1)) as u32) as u16
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        self.write8(addr, ((val & 0xff00) >> 8) as u8);
        self.write8(addr + 1, (val & 0x00ff) as u8)
    }

    pub fn fetch(&self, pc: u16) -> u16 {
        self.read16(pc)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::consts::MEMORY_SIZE;
    use crate::core::memory::memory::Memory;
    #[test]
    fn test_memory_structure_constructor() {
        let expected_memory = Memory {
            mem: vec![0; MEMORY_SIZE as usize],
            size: MEMORY_SIZE,
        };
        assert_eq!(expected_memory, Memory::new())
    }

    #[test]
    fn test_memory_write8_and_read8() {
        let mut mem = Memory::new();
        mem.write8(0, 255u8);
        let data = mem.read8(0);
        assert_eq!(255u8, data)
    }

    #[test]
    fn test_memory_write16_and_read16() {
        let mut mem = Memory::new();
        mem.write16(0, 0x1234);
        let data = mem.read16(0);
        assert_eq!(0x1234, data)
    }

    #[test]
    fn test_memory_fetch() {
        let mut mem = Memory::new();
        mem.write16(0, 0x1234);
        let data = mem.fetch(0);
        assert_eq!(0x1234, data)
    }
}
