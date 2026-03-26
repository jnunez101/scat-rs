#[path ="./parsers/qualcomm/util.rs"]
mod util;
#[path="./parsers/qualcomm/qualcommparser.rs"]
mod qualcommparser;
#[path="./iodevices/serial_device.rs"]
mod serial_device;
//#[path="./parsers/qualcomm/qualcommparser.rs"]
//mod qualcommparser;
#[path="./parsers/qualcomm/byte_parsers/diagheader.rs"]
mod diagheader;
#[path="./iodevices/device_interface.rs"]
mod device_interface;

use core::fmt;
use std::str::Utf8Error;

use clap::Parser;

pub type Result<T> = std::result::Result<T, ScatRSError>;


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

impl std::error::Error for ScatRSError {}

pub trait ScatRSIO {
    fn write(&mut self, buf: &[u8]) -> Result<()>;

    fn read(&mut self) -> Result<Vec<u8>>;
}

// Scat-RS a program for harvesting LTE and NR signal information from Qualcomm modems in RUST!
#[derive(Parser, Debug)]
#[command(version, about)]
struct ScatRSArgs {
    // Absolute path for the DIAG port of the modem
    #[arg(short, long)]
    serial: String,

    // Parser type only Qualcomm is supported right now
    #[arg(short, long)]
    parser_type: String,
}

fn main() {
    let args = ScatRSArgs::parse();
    let mut _io_device: Box<dyn ScatRSIO> = match args.parser_type.as_str() {
        "qc" => Box::new(serial_device::DeviceIO::from_string(args.serial).unwrap()),
        _ => panic!("ohea")
    };
}