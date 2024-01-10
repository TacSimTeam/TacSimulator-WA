#[derive(Clone, Copy, PartialOrd, PartialEq)]
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

    // TODO それぞれ8ビットと16ビットを受け取るように変更
    pub fn set_high_8(&mut self, value: u32) {
        self.value = ((value & 0xff) << 16) | (self.value & 0xffff);
    }

    pub fn set_low_16(&mut self, value: u32) {
        self.value = (self.value & 0xffff0000) | (value & 0xffff);
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
    fn test_tlb_constructor() {
        let expected_tlb = TlbEntry { value: 0x009f00 };
        assert_eq!(expected_tlb, TlbEntry::new(0x009f00))
    }

    #[test]
    fn test_tlb_get_high_8() {
        let tlb = TlbEntry { value: 0x009f00 };
        assert_eq!(tlb.get_high_8(), 0x00u8)
    }

    #[test]
    fn test_tlb_is_valid() {
        let tlb = TlbEntry::new(0x009f00);
        assert!(tlb.is_valid())
    }

    #[test]
    fn test_tlb_get_low_16() {
        let tlb = TlbEntry::new(0x009f00);
        assert_eq!(tlb.get_low_16(), 0x9f00)
    }
}
