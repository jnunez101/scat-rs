use std::{io::{self, BufRead, BufReader, BufWriter, Write}, time::Duration};

use serialport::SerialPort;

use crate::ScatRSIO;

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


impl ScatRSIO for DeviceIO {
    fn write(&mut self, buf: &[u8]) -> (){
        self.writer.write(buf);
        self.writer.flush();
    }
    fn read(&mut self)-> Vec<u8> {
        let mut buf = vec![0;1024];
        self.reader.read_until(0x7e, &mut buf);

        buf
    }
}
