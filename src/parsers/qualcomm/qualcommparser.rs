use crate::{Result, ScatRSError, ScatRSIO, qualcommparser::diagconstcmd::DIAG_VERNO_F};


#[path="./byte_parsers/diagheader.rs"]
mod diagheader;

#[path="./message_constants/diagconstcmd.rs"]
mod diagconstcmd;

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

    fn init_diag(mut self) -> Result<()> {
        println!("Initializing diag");
        let _ = self.device.write(&DIAG_VERNO_F);
        let _pkt = self.device.read()?;
        Ok(())
    }
}
