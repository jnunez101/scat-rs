use core::fmt;

use nom::{bytes::complete::take};

use crate::{Result};


trait DiagStructure {
    fn parse(self, pkt: &[u8]) -> Result<Box<dyn DiagStructure>>;
}




pub struct QcDiagVersion {
    pub compile_date: String,
    pub compile_time: String,
    pub release_date: String,
    pub release_time: String,
    pub chipset: String
}

impl QcDiagVersion {
    fn from_args(
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
        write!(f, "compile {}/{} release_date: {}/{} chipset: {}", 
            self.compile_date,
            self.compile_time,
            self.release_date,
            self.release_time,
            self.chipset
        )
    }
}

impl DiagStructure for QcDiagVersion {
    fn parse(self, pkt: &[u8]) -> Result<Box<dyn DiagStructure>> {
        let (pkt, compile_date) = take::<usize, &[u8], nom::error::Error<_>>(11usize)(pkt)?;
        let (pkt, compile_time) = take::<usize, &[u8], nom::error::Error<_>>(8usize)(pkt)?;
        let (pkt, release_date)= take::<usize, &[u8], nom::error::Error<_>>(11usize)(pkt)?;
        let (pkt, release_time) = take::<usize, &[u8], nom::error::Error<_>>(8usize)(pkt)?;
        let (_, chipset) = take::<usize, &[u8], nom::error::Error<_>>(8usize)(pkt)?;

        Ok(Box::new(QcDiagVersion::from_args(compile_date, compile_time, release_date, release_time, chipset)?))
    }
}

