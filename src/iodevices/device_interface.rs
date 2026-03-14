use core::fmt;
use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, ScatRSError>;

pub trait ScatRSIO {
    fn write(&mut self, buf: Vec<u8>) -> std::io::Result<()>;
    fn read(&mut self) -> std::io::Result<Vec<u8>>;
}

#[derive(Debug, Clone)]
pub enum ScatRSError {
    ParsingError,
    ParsingUTF8Error(Utf8Error),
    ReadError(),
    WriteError(),
}
impl fmt::Display for ScatRSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Error with ScatRSIO")
    }
}