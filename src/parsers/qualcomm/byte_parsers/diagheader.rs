use core::fmt;
use std::path::Display;

use nom::{IResult, bytes::complete::take};

pub struct QcDiagVersion {
    compile_date: Vec<u8>,
    compile_time: Vec<u8>,
    release_date: Vec<u8>,
    release_time: Vec<u8>,
    chipset: Vec<u8>
}

impl QcDiagVersion {
    fn from_bytes(
        compile_date: &[u8],
        compile_time: &[u8],
        release_date: &[u8],
        release_time: &[u8],
        chipset: &[u8]
    ) -> Self {
        QcDiagVersion {
            compile_date: compile_date.to_vec(),
            compile_time: compile_time.to_vec(),
            release_date: release_date.to_vec(),
            release_time: release_time.to_vec(),
            chipset: chipset.to_vec()
        }
    }
}

impl fmt::Display for QcDiagVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "compile date: {} compile time: {} release_date: {} release time: {} chipset: {}", 
            std::str::from_utf8(&self.compile_date).unwrap(),
            std::str::from_utf8(&self.compile_time).unwrap(),
            std::str::from_utf8(&self.release_date).expect("msg"),
            std::str::from_utf8(&self.release_time).expect("msg"),
            std::str::from_utf8(&self.chipset).expect("msg")
        )
    }
}

pub fn parse_diag_version(input_buf: &[u8]) -> IResult<&[u8],QcDiagVersion> {
    let (input_buf, compile_date) = take(11usize)(input_buf)?;
    let (input_buf, compile_time) = take(8usize)(input_buf)?;
    let (input_buf, release_date)= take(11usize)(input_buf)?;
    let (input_buf, release_time) = take(8usize)(input_buf)?;
    let (input_buf, chipset) = take(8usize)(input_buf)?;

    Ok((input_buf, QcDiagVersion::from_bytes(compile_date, compile_time, release_date, release_time, chipset)))
}

