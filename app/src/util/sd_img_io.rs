use crate::core::consts::SECTOR_SIZE;
use crate::core::error::sd_io_error::SdIoError;
use gloo::file::File;
use regex::Regex;

#[derive(Clone, PartialEq)]
pub struct SDImgIo {
    file: File,
    buf: Option<Vec<u8>>,
}

impl SDImgIo {
    pub fn new(file: File) -> Self {
        Self { file, buf: None }
    }

    pub fn read_sect(&self, sect_addr: u32) -> Result<Vec<u8>, SdIoError> {
        let sect_addr = sect_addr as usize;
        return if self.buf.is_none() {
            Err(SdIoError::SdIsNotOpen)
        } else {
            Ok(
                self.buf.as_ref().unwrap()[SECTOR_SIZE * sect_addr..SECTOR_SIZE * (sect_addr + 1)]
                    .to_vec(),
            )
        };
    }

    pub fn write_sect(&mut self, sect_addr: u32, data: Vec<u8>) -> Result<(), SdIoError> {
        let sect_addr = sect_addr as usize;
        if self.buf.is_none() {
            return Err(SdIoError::SdIsNotOpen);
        } else {
            for i in 0..SECTOR_SIZE {
                self.buf.as_mut().unwrap()[sect_addr * SECTOR_SIZE + i] = data[i];
            }
        }
        return Ok(());
    }

    pub fn open_file(&mut self, file_path: String) -> Result<(), SdIoError> {
        let re = Regex::new(r"/\.(DMG)$/i").unwrap();
        if !re.is_match(file_path.as_str()) {
            return Err(SdIoError::FileIsNotDMG);
        }

        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        return self.buf.is_some();
    }

    pub fn get_file_name(&self) -> String {
        format!("{:?}", self.file.name())
    }
}
