//! A simple framed request and response protocol.

use core::convert::Infallible;
use arduino_hal::prelude::{_embedded_hal_serial_Read as Read};

mod crc;
pub use crc::*;

pub mod incoming;
pub mod outgoing;

pub const MAGIC: [u8; 2] = [b'H', b'B'];

pub const MAX_PAYLOAD_SIZE: usize = u8::MAX as usize;

/// In big endian:               | magic       | len | payload          | checksum |
pub const MAX_PACKET_SIZE: usize = MAGIC.len() + 1   + MAX_PAYLOAD_SIZE + 2        ;

pub enum FrameError<T = Infallible> {
    NotMagic,
    ChecksumMismatch,
    FromRead(T),
}

impl<T> From<T> for FrameError<T> {
    fn from(value: T) -> Self { FrameError::FromRead(value) }
}

pub fn read_frame<'a, R: Read<u8>>(
    reader: &mut R,
    buffer: &'a mut heapless::Deque<u8, MAX_PACKET_SIZE>)
    -> Result<&'a mut [u8], FrameError<R::Error>> {
    let mut cursor = 0;
    let mut read = || {
        let byte = if cursor == buffer.len() {
            match nb::block!(reader.read()) {
                Ok(byte) => {
                    buffer.push_back(byte).map_err(drop).unwrap();
                    byte
                },
                Err(x) => {
                    return Err(FrameError::FromRead(x));
                },
            }
        } else {
            *buffer.get(cursor).unwrap()
        };
        cursor += 1;
        Ok(byte)
    };

    for byte in MAGIC {
        if read()? != byte {
            return Err(FrameError::NotMagic);
        }
    }

    let len = read()?;

    let mut crc = CRC16::default();
    for _ in 0..len {
        crc.resume(read()?);
    }
    if crc.complete() == u16::from_be_bytes([read()?, read()?]) {
        Ok(&mut buffer.make_contiguous()[MAGIC.len() + 1..][..len as usize])
    } else {
        Err(FrameError::ChecksumMismatch)
    }
}

pub fn surround_frame(buffer: &mut heapless::Deque<u8, MAX_PACKET_SIZE>) {
    let mut crc = CRC16::default();
    for byte in buffer.iter() {
        crc.resume(*byte);
    }

    buffer.push_front(buffer.len().try_into().map_err(drop).unwrap()).map_err(drop).unwrap();
    for byte in MAGIC.into_iter().rev() {
        buffer.push_front(byte).map_err(drop).unwrap();
    }
    for byte in crc.complete().to_be_bytes() {
        buffer.push_back(byte).map_err(drop).unwrap();
    }
}
