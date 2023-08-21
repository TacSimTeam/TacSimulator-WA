use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SdIoError {
    FileIsNotDMG,
    SdIsNotOpen
}

impl Display for SdIoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SdIoError::FileIsNotDMG => write!(f, ".dmgのみ読み込み可能です"),
            SdIoError::SdIsNotOpen => write!(f, "イメージファイルが開かれていません")
        }
    }
}

impl Error for SdIoError {}
