use crate::core::consts::SECTOR_SIZE;
use crate::core::error::sd_io_error::SdIoError;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::memory::memory::Memory;
use reqwest::blocking;

pub struct SdHostController {
    idle_flag: bool,
    error_flag: bool,
    intr_flag: bool,
    mem_addr: u16,
    sec_addr_high: u16,
    sec_addr_low: u16,
    memory: Memory,
    intr_sig: IntrController,
    buf: Vec<u8>,
}

impl SdHostController {
    pub fn new(memory: Memory, intr_sig: IntrController) -> Self {
        let mut sd_host_controller = Self {
            idle_flag: true,
            error_flag: false,
            intr_flag: false,
            mem_addr: 0,
            sec_addr_high: 0,
            sec_addr_low: 0,
            memory,
            intr_sig,
            buf: Vec::default(),
        };
        sd_host_controller.fetch();
        sd_host_controller
    }

    pub fn start_reading(&mut self) {
        if !self.idle_flag {
            return;
        }
        self.idle_flag = false;
        let data = self.read_sect(self.sec_addr()).unwrap();
    }

    pub fn get_mem_addr(&mut self) -> u16 {
        self.mem_addr
    }

    pub fn sec_addr(&self) -> u32 {
        (self.get_sec_addr_high() << 16 | self.get_sec_addr_low()) as u32
    }

    pub fn get_sec_addr_high(&self) -> u16 {
        self.sec_addr_high
    }

    pub fn get_sec_addr_low(&self) -> u16 {
        self.sec_addr_low
    }

    fn set_sec_addr_high(&mut self, addr_h: u16) {
        self.sec_addr_high = addr_h;
    }

    fn set_sec_addr_low(&mut self, addr_l: u16) {
        self.sec_addr_low = addr_l;
    }

    fn set_intr_flag(&mut self, flag: bool) {
        self.intr_flag = flag;
    }

    pub fn is_idle(&self) -> bool {
        self.idle_flag
    }

    pub fn is_error_occurred(&self) -> bool {
        self.error_flag
    }

    fn fetch(&mut self) {
        // TODO 適切なエラー処理を施す
        let res = blocking::get("http://localhost:3000/assets/sample.dmg").unwrap();
        self.buf = res.bytes().unwrap().to_vec();
    }

    pub fn read_sect(&self, sect_addr: u32) -> Result<Vec<u8>, SdIoError> {
        return if self.buf.is_empty() {
            Err(SdIoError::SdIsNotOpen)
        } else {
            Ok(self.buf
                [(SECTOR_SIZE * sect_addr) as usize..(SECTOR_SIZE * (sect_addr + 1)) as usize]
                .to_vec())
        };
    }

    pub fn write_sect(&mut self, sect_addr: u32, data: Vec<u8>) -> Result<(), SdIoError> {
        if self.buf.is_empty() {
            return Err(SdIoError::SdIsNotOpen);
        } else {
            for i in 0..SECTOR_SIZE {
                self.buf[(sect_addr * SECTOR_SIZE + i) as usize] = data[i as usize];
            }
        }
        Ok(())
    }
}
