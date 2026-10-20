use arduino_hal::prelude::_embedded_hal_serial_Write as Write;
use bitflags::bitflags;

use crate::packet;

bitflags! {
    pub struct PinFlags: u8 {
        const DIGITAL_INPUT = 1 << 0;
        const DIGITAL_OUTPUT = 1 << 1;
        const ANALOG_INPUT = 1 << 2;
        const ANALOG_OUTPUT = 1 << 3;
        const RESERVED = 1 << 4;
    }
}

impl packet::PacketSerialize for PinFlags {
    fn serialize<W: Write<u8>>(&self, writer: &mut W) -> Result<(), W::Error> {
        macro_rules! write {
            ($($byte:expr),*$(,)?) => {
                { $( nb::block!(writer.write($byte))?; )* }
            };
        }

        if self.is_empty() {
            write!(b'0');
        } else {
            let mut is_first = true;
            if self.contains(PinFlags::DIGITAL_INPUT) {
                if !is_first {
                    write!(b'|');
                }
                write!(b'D', b'i', b'g', b'i', b't', b'a', b'l', b'I', b'n', b'p', b'u', b't');
                is_first = false;
            }
            if self.contains(PinFlags::DIGITAL_OUTPUT) {
                if !is_first {
                    write!(b'|');
                }
                write!(b'D', b'i', b'g', b'i', b't', b'a', b'l', b'O', b'u', b't', b'p', b'u', b't');
                is_first = false;
            }
            if self.contains(PinFlags::ANALOG_INPUT) {
                if !is_first {
                    write!(b'|');
                }
                write!(b'A', b'n', b'a', b'l', b'o', b'g', b'I', b'n', b'p', b'u', b't');
                is_first = true;
            }
            if self.contains(PinFlags::ANALOG_OUTPUT) {
                if !is_first {
                    write!(b'|');
                }
                write!(b'A', b'n', b'a', b'l', b'o', b'g', b'O', b'u', b't', b'p', b'u', b't');
                is_first = false;
            }
            if self.contains(PinFlags::RESERVED) {
                if !is_first {
                    write!(b'|');
                }
                write!(b'R', b'e', b's', b'e', b'r', b'v', b'e', b'd');
                // is_first = false;
            }
        }
        Ok(())
    }
}