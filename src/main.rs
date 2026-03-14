#[path ="./parsers/qualcomm/util.rs"]
mod util;
#[path="./iodevices/serial_device.rs"]
mod serial_device;
//#[path="./parsers/qualcomm/qualcommparser.rs"]
//mod qualcommparser;
#[path="./parsers/qualcomm/byte_parsers/diagheader.rs"]
mod diagheader;
#[path="./iodevices/device_interface.rs"]
mod device_interface;

use clap::Parser;
use crate::{device_interface::ScatRSIO, diagheader::parse_diag_version};


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
    let mut io_device: Box<dyn ScatRSIO> = match args.parser_type.as_str() {
        "qc" => Box::new(serial_device::DeviceIO::from_string(args.serial).expect("Instance")),
        _ => panic!("ohea")
    };
}