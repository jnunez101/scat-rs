use crc::{self, CRC_16_IBM_SDLC};

fn sanitize_packet(buf: &mut Vec<u8>) {
    let mut index = 0;
    while index < buf.len() {
        if buf[index] == 0x7d {
            buf.insert(index, 0x5d);
            index+=2;
        }
        else if buf[index] == 0x7e {
            buf[index] = 0x7d;
            buf.insert(index, 0x5e);
            index+=2;
        }
        else {
            index+=1;
        }
    }
}

fn remove_sanitations_from_packet(buf: &mut Vec<u8>) {

}

fn generate_checksum(buf: &[u8]) -> [u8;2] {
    const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

    let checksum = X25.checksum(buf).to_be_bytes();
    
    checksum
}

pub fn generate_packet(buf: Vec<u8>) -> Vec<u8> {
    let mut new_buffer: Vec<u8> = buf;

    let converted_vec = new_buffer.clone().into_boxed_slice();

    let [checksum_high, checksum_low] = generate_checksum(&converted_vec);

    new_buffer.push(checksum_low);
    new_buffer.push(checksum_high);

    sanitize_packet(&mut new_buffer);

    new_buffer.push(0x7e);

    new_buffer
}