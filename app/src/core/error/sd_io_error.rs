use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SdIoError {
    SdIsNotOpen,
}

impl Display for SdIoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SdIoError::SdIsNotOpen => write!(f, "イメージファイルが開かれていません"),
        }
    }
}

impl Error for SdIoError {}
