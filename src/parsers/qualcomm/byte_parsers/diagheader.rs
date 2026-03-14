use core::fmt;
use std::str::Utf8Error;

use nom::{IResult, bytes::complete::take};

use crate::device_interface::{ScatRSError, Result};

impl From<Utf8Error> for ScatRSError{
    fn from(err: Utf8Error) -> ScatRSError {
        ScatRSError::ParsingUTF8Error(err)
    }
}

pub struct QcDiagVersion {
    pub compile_date: String,
    pub compile_time: String,
    pub release_date: String,
    pub release_time: String,
    pub chipset: String
}

impl QcDiagVersion {
    fn from_bytes(
        compile_date: &[u8],
        compile_time: &[u8],
        release_date: &[u8],
        release_time: &[u8],
        chipset: &[u8]
    ) -> Result<Self> {
        Ok(QcDiagVersion {
            compile_date: std::str::from_utf8(compile_date)?.to_owned(),
            compile_time: std::str::from_utf8(compile_time)?.to_owned(),
            release_date: std::str::from_utf8(release_date)?.to_owned(),
            release_time: std::str::from_utf8(release_time)?.to_owned(),
            chipset: std::str::from_utf8(chipset)?.to_owned()
        })
    }
}

impl fmt::Display for QcDiagVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "compile date: {} compile time: {} release_date: {} release time: {} chipset: {}", 
            self.compile_date,
            self.compile_time,
            self.release_date,
            self.release_time,
            self.chipset
        )
    }
}

pub fn parse_diag_version(input_buf: &[u8]) -> IResult<&[u8],QcDiagVersion> {

    let (input_buf, compile_date) = take(11usize)(input_buf)?;
    let (input_buf, compile_time) = take(8usize)(input_buf)?;
    let (input_buf, release_date)= take(11usize)(input_buf)?;
    let (input_buf, release_time) = take(8usize)(input_buf)?;
    let (input_buf, chipset) = take(8usize)(input_buf)?;

    Ok((input_buf, QcDiagVersion::from_bytes(compile_date, compile_time, release_date, release_time, chipset)?))
}

