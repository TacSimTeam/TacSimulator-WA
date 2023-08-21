use regex::Regex;
use gloo::file::File;
use crate::core::error::sd_io_error::SdIoError;
use crate::core::consts::SECTOR_SIZE;

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
        return if self.buf.is_none() {
            Err(SdIoError::SdIsNotOpen)
        } else {
            Ok(self.buf.as_ref().unwrap()[(SECTOR_SIZE * sect_addr) as usize..(SECTOR_SIZE * (sect_addr + 1)) as usize].to_vec())
        }
    }

    pub fn write_sect(&mut self, sect_addr: u32, data: Vec<u8>) -> Result<(), SdIoError> {
        if self.buf.is_none() {
            return Err(SdIoError::SdIsNotOpen)
        } else {
            for i in 0..SECTOR_SIZE {
                self.buf.as_mut().unwrap()[(sect_addr * SECTOR_SIZE + i) as usize] = data[i as usize];
            }
        }
        return  Ok(())
    }

    pub fn open_file(&mut self, file_path: String) -> Result<(), SdIoError> {
        let re = Regex::new(r"/\.(DMG)$/i").unwrap();
        if !re.is_match(file_path.as_str()) {
            return Err(SdIoError::FileIsNotDMG);
        }

        // TODO glooのFileに応じて適切に実装し直しが必要. ファイル入力時点で読まれてるからOk(())返すだけでいいかも
        // let mut buf: Vec<u8> = Vec::new();
        // self.file.read_to_end(&mut buf).unwrap();
        // self.buf = Some(buf);
        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        return self.buf.is_some()
    }

    pub fn get_file_name(&self) -> String {
        format!("{:?}", self.file.name())
    }
}
