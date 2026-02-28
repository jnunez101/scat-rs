use clap::Parser;

use crate::diagheader::parse_diag_version;

#[path ="./parsers/qualcomm/util.rs"]
mod util;
#[path="./iodevices/serial_device.rs"]
mod serial_device;
//#[path="./parsers/qualcomm/qualcommparser.rs"]
//mod qualcommparser;
#[path="./parsers/qualcomm/byte_parsers/diagheader.rs"]
mod diagheader;

trait ScatRSIO {
    fn write(&mut self, buf: &[u8]) -> ();
    fn read(&mut self) -> Vec<u8>;
}

/*trait ScatParser {
    fn new(scatrs_io: Box<dyn ScatRSIO>) -> Result<Self, ()>;
}*/

// Scold-RS a program for harvesting LTE and NR signal information from Qualcomm modems in RUST!
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
    io_device.write(&[0]);
    let packet = io_device.read();

    let (leftover_input, packet_info) =parse_diag_version(&packet).expect("ohea");

    print!("{}", packet_info)

}