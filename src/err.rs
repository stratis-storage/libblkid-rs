use std::{error::Error, fmt::{self, Display}, io};

pub type Result<T> = std::result::Result<T, BlkidErr>;

#[derive(Debug)]
pub enum BlkidErr {
    IO(io::Error),
    Null(std::ffi::NulError),
    PositiveReturnCode,
    InvalidConv,
    Other(String),
}

impl Display for BlkidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlkidErr::IO(ref e) => write!(f, "There was an IO error: {}", e),
            BlkidErr::Null(ref e) => write!(f, "Null error during string conversion: {}", e),
            BlkidErr::PositiveReturnCode => write!(f, "Positive return code found when <= 0 was expected"),
            BlkidErr::InvalidConv => write!(f, "The requested conversion was unsuccessful"),
            BlkidErr::Other(ref s) => write!(f, "{}", s),
        }
    }
}

impl Error for BlkidErr {}
