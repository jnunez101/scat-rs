use crate::{Result, ScatRSError, ScatRSIO, diagheader::{DiagClass, DiagExtBuildId, DiagVersion}, qualcommparser::diagconstcmd::{DIAG_EVENT_REPORT_F, DIAG_EXT_BUILD_ID_F, DIAG_LOG_CONFIG_F, DIAG_VERNO_F, LOG_CONFIG_RETRIEVE_ID_RANGES_OP}};


#[path="./byte_parsers/diagheader.rs"]
pub mod diagheader;

#[path="./message_constants/diagconstcmd.rs"]
pub mod diagconstcmd;

pub struct QualcommDiagParser {
    device: Box<dyn ScatRSIO>
}

impl From<std::str::Utf8Error> for ScatRSError{
    fn from(err: std::str::Utf8Error) -> Self {
        ScatRSError::ParsingUTF8Error(err)
    }
}

impl<E> From<nom::Err<E>> for ScatRSError{
    fn from(err: nom::Err<E>) -> Self {
        match err {
            nom::Err::Incomplete(_) => ScatRSError::ParsingError, // Pkt too short
            nom::Err::Failure(_) => panic!("Faliure parsing"), // panic?
            nom::Err::Error(_) => ScatRSError::ParsingError, // recoverable?
        }
    }
}

impl QualcommDiagParser {
    fn clear_diag() {
        println!("Stopping Diag");
        //Static event reporting 
        //self.io_device.write_then_read_discard(util.generate_packet(struct.pack('<BB', diagcmd.DIAG_EVENT_REPORT_F, 0x00)), 0x1000)
        //self.io_device.write_then_read_discard(util.generate_packet(struct.pack('<LL', diagcmd.DIAG_LOG_CONFIG_F, diagcmd.LOG_CONFIG_DISABLE_OP)), 0x1000)
        //self.io_device.write_then_read_discard(util.generate_packet(struct.pack('<BBHHH', diagcmd.DIAG_EXT_MSG_CONFIG_F, 0x05, 0x0000, 0x0000, 0x0000)), 0x1000)
    }

    pub fn init_diag(mut self) -> Result<()> {
        println!("Initializing diag");
        let _ = self.device.write(&DIAG_VERNO_F);
        let mut pkt = self.device.read()?;

        let version = DiagVersion::parse(&pkt)?;
        print!("{}", version);

        let  _ = self.device.write(&DIAG_EXT_BUILD_ID_F);
        pkt = self.device.read()?;

        let build_id = DiagExtBuildId::parse(&pkt)?;
        print!("{}", build_id);

        // maybe I should remove the array from all the definitions?
        let  _ = self.device.write(&[DIAG_EVENT_REPORT_F[0], 0x00]);
        pkt = self.device.read()?;

        // Need to send DIAG_LOG_CONFIG_F with 3 padded bytes after wards combined with LOG_CONFIG_RETRIEVE_ID_RANGES_OP with 3 bytes afterwards
        // todo figure out a clean way to handle this in rust
        // maybe something like python with struct.pack('<LL')
        let _ = self.device.write(&[DIAG_LOG_CONFIG_F[0],0,0,0,LOG_CONFIG_RETRIEVE_ID_RANGES_OP as u8,0,0,0]);
        pkt = self.device.read()?;
        




        Ok(())
    }
}
