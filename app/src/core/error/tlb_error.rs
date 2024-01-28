use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum TlbError {
    ReadOnly,
    TlbMiss,
}

impl Display for TlbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TlbError::TlbMiss => write!(f, "Tlbミス"),
            TlbError::ReadOnly => write!(f, "ReadOnly"),
        }
    }
}

impl Error for TlbError {}
