pub trait IIOSerial {
    fn receive(&mut self) -> u8;
    fn send(&mut self, val: u8);
    fn set_receivable_intr_flag(&mut self, flag: bool);
    fn set_sendable_intr_flag(&mut self, flag: bool);
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;
}
