use core::fmt;

use nom::{bytes::complete::take};

use crate::{Result, qualcommparser::diagconstcmd::{LOG_CONFIG_DISABLE_OP, LOG_CONFIG_GET_LOGMASK_OP, LOG_CONFIG_RETRIEVE_ID_RANGES_OP, LOG_CONFIG_RETRIEVE_VALID_MASK_OP, LOG_CONFIG_SET_MASK_OP}};


pub trait DiagClass: fmt::Display {
    fn parse(pkt: &[u8]) -> Result<Box<dyn DiagClass>>
    where 
    Self: Sized;
}

pub struct DiagLogConfig {
    pkt_id: u16,
    cmd_id: u16,
    payload: String,
}

impl fmt::Display for DiagLogConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cmd_id {
            LOG_CONFIG_DISABLE_OP  => write!(f, "Log Config: Disable, Extra: {}", self.payload),
            LOG_CONFIG_RETRIEVE_ID_RANGES_OP => write!(f, "Log Config: Disable, Extra: {}", self.payload),
            LOG_CONFIG_RETRIEVE_VALID_MASK_OP => write!(f, "Log Config: Disable, Extra: {}", self.payload),
            LOG_CONFIG_SET_MASK_OP => write!(f, "Log Config: Disable, Extra: {}", self.payload),
            LOG_CONFIG_GET_LOGMASK_OP => write!(f, "Log Config: Disable, Extra: {}", self.payload),
            _ => write!(f, "UNKNOWN ACTION IN LOG CONFIG"),
        }
    }
}

pub struct DiagLogHeader {
    cmd: u8, // Should be 0x10
    reserved: u8,
    length1: u16,
    length2: u16,
    log_id: u8,
    timestamp: u64,
}


pub struct DiagVersion {
    pub compile_date: String,
    pub compile_time: String,
    pub release_date: String,
    pub release_time: String,
    pub chipset: String
}

impl fmt::Display for DiagVersion {
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

impl DiagClass for DiagVersion {
    fn parse(pkt: &[u8]) -> Result<Box<dyn DiagClass>> {
        let (pkt, compile_date) = take::<usize, &[u8], nom::error::Error<_>>(11usize)(pkt)?;
        let (pkt, compile_time) = take::<usize, &[u8], nom::error::Error<_>>(8usize)(pkt)?;
        let (pkt, release_date)= take::<usize, &[u8], nom::error::Error<_>>(11usize)(pkt)?;
        let (pkt, release_time) = take::<usize, &[u8], nom::error::Error<_>>(8usize)(pkt)?;
        let (_, chipset) = take::<usize, &[u8], nom::error::Error<_>>(8usize)(pkt)?;

        Ok(Box::new(DiagVersion{
            compile_date: std::str::from_utf8(compile_date)?.to_owned(),
            compile_time: std::str::from_utf8(compile_time)?.to_owned(),
            release_date: std::str::from_utf8(release_date)?.to_owned(),
            release_time: std::str::from_utf8(release_time)?.to_owned(),
            chipset: std::str::from_utf8(chipset)?.to_owned()
        }))
    }
}


pub struct DiagExtBuildId {
    build: String
}

impl fmt::Display for DiagExtBuildId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Build ID: {}", self.build)
    }
}

impl DiagClass for DiagExtBuildId {
    fn parse(pkt: &[u8]) -> Result<Box<dyn DiagClass>>{
        // the build ID starts 12 bytes into the pkt
        let (pkt, _) = take::<usize, &[u8], nom::error::Error<_>>(12usize)(pkt)?;
        Ok(Box::new(DiagExtBuildId{
            build: std::str::from_utf8(pkt)?.to_owned()
        }))
    }
}

