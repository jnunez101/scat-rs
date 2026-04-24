use std::{io::{self, BufRead, BufReader, BufWriter, Write}, time::Duration};
use serialport::SerialPort;
use crate::{ScatRSError, ScatRSIO, Result, util::{generate_packet, remove_sanitations_from_packet}};

pub struct DeviceIO {
    writer: BufWriter<Box<dyn SerialPort>>,
    reader: BufReader<Box<dyn SerialPort>>,
}

impl DeviceIO {
    pub fn from_serial(serial: Box<dyn SerialPort>) -> io::Result<Self> {
        let writer = BufWriter::new(serial.try_clone()?);
        let reader = BufReader::new(serial);

        Ok(Self {writer, reader})
    }
    pub fn from_string(port_string: String) -> io::Result<Self> {
        let serial: Box<dyn SerialPort> = serialport::new(port_string, 115_200).timeout(Duration::from_secs(10)).open().expect("Failed to open Serial port");
        
        DeviceIO::from_serial(serial)
    }
}

impl From<std::io::Error> for ScatRSError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::InvalidData => ScatRSError::ParsingError,
            std::io::ErrorKind::WriteZero
            | std::io::ErrorKind::BrokenPipe => ScatRSError::WriteError(),
            _ => ScatRSError::ReadError(),
        }
    }
}


impl ScatRSIO for DeviceIO {
    fn write(&mut self, buf: &[u8]) -> Result<()>{
        let pkt = buf.to_vec();
        self.writer.write(&generate_packet(pkt))?;
        self.writer.flush()?;
        Ok(())
    }
    fn read(&mut self)-> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.reader.read_until(0x7e, &mut buf)?;

        //remove the crc
        // todo add a flag to actually validate the crc
        let _checksum_lower = buf.pop().unwrap();
        let _checksum_higher = buf.pop().unwrap();


        Ok(remove_sanitations_from_packet(buf))
    }
}
