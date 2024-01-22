#[derive(Clone, Copy, PartialOrd, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct TlbEntry {
    pub value: u32,
}

impl TlbEntry {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn get_high_8(&self) -> u8 {
        ((self.value & 0x00ff0000) >> 16) as u8
    }

    pub fn get_low_16(&self) -> u16 {
        (self.value & 0x0000ffff) as u16
    }

    pub fn set_high_8(&mut self, value: u8) {
        self.value = ((value as u32 & 0xff) << 16) | (self.value & 0xffff);
    }

    pub fn set_low_16(&mut self, value: u16) {
        self.value = (self.value & 0xffff0000) | (value as u32 & 0xffff);
    }

    pub fn get_page(&self) -> u8 {
        self.get_high_8()
    }

    pub fn get_frame(&self) -> u8 {
        (self.value & 0xff) as u8
    }

    pub fn is_valid(&self) -> bool {
        (self.value & (1 << 15)) != 0
    }

    pub fn is_readable(&self) -> bool {
        (self.value & (1 << 10)) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.value & (1 << 9)) != 0
    }

    pub fn is_executable(&self) -> bool {
        (self.value & (1 << 8)) != 0
    }

    pub fn set_reference_flag(&mut self) {
        self.value |= 1 << 12;
    }

    pub fn set_dirty_flag(&mut self) {
        self.value |= 1 << 11;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::memory::tlb::TlbEntry;

    #[test]
    fn test_tlb_util() {
        let mut entry = TlbEntry::new(0x00aaffbb);

        assert_eq!(entry.get_high_8(), 0xaa);
        assert_eq!(entry.get_low_16(), 0xffbb);

        entry.set_high_8(0x22);
        assert_eq!(entry.get_high_8(), 0x22);

        entry.set_low_16(0xff44);
        assert_eq!(entry.get_low_16(), 0xff44);

        entry.set_high_8(0xaa);
        entry.set_low_16(0xffbb);
        assert_eq!(entry.get_high_8(), 0xaa);
        assert_eq!(entry.get_low_16(), 0xffbb);

        assert!(entry.is_valid());
        assert!(entry.is_readable());
        assert!(entry.is_writable());
        assert!(entry.is_executable());
    }

    #[test]
    fn test_set_reference_flag() {
        let mut entry = TlbEntry::new(0);
        assert_eq!(entry.value & 1 << 12, 0);

        entry.set_reference_flag();
        assert_ne!(entry.get_low_16() & 1 << 12, 0);
    }
}
