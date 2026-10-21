use arduino_hal::prelude::_embedded_hal_serial_Write as Write;
use bitflags::bitflags;

use crate::packet::{self, Char};

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct PinFlags: u8 {
        const DIGITAL_INPUT = 1 << 0;
        const DIGITAL_OUTPUT = 1 << 1;
        const ANALOG_INPUT = 1 << 2;
        const ANALOG_OUTPUT = 1 << 3;
        const RESERVED = 1 << 4;
    }
}

impl packet::PacketSerialize for PinFlags {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
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
                    if whitespace {
                        <Char![' ', '|', ' ']>::new().serialize(writer, whitespace)?;
                    } else {
                        <Char!['|']>::new().serialize(writer, whitespace)?;
                    }
                }
                <Char!['d', 'i', 'g', 'i', 't', 'a', 'l', 'I', 'n', 'p', 'u', 't']>::new().serialize(writer, whitespace)?;
                is_first = false;
            }
            if self.contains(PinFlags::DIGITAL_OUTPUT) {
                if !is_first {
                    if whitespace {
                        <Char![' ', '|', ' ']>::new().serialize(writer, whitespace)?;
                    } else {
                        <Char!['|']>::new().serialize(writer, whitespace)?;
                    }
                }
                <Char!['d', 'i', 'g', 'i', 't', 'a', 'l', 'O', 'u', 't', 'p', 'u', 't']>::new().serialize(writer, whitespace)?;
                is_first = false;
            }
            if self.contains(PinFlags::ANALOG_INPUT) {
                if !is_first {
                    if whitespace {
                        <Char![' ', '|', ' ']>::new().serialize(writer, whitespace)?;
                    } else {
                        <Char!['|']>::new().serialize(writer, whitespace)?;
                    }
                }
                <Char!['a', 'n', 'a', 'l', 'o', 'g', 'I', 'n', 'p', 'u', 't']>::new().serialize(writer, whitespace)?;
                is_first = false;
            }
            if self.contains(PinFlags::ANALOG_OUTPUT) {
                if !is_first {
                    if whitespace {
                        <Char![' ', '|', ' ']>::new().serialize(writer, whitespace)?;
                    } else {
                        <Char!['|']>::new().serialize(writer, whitespace)?;
                    }
                }
                <Char!['a', 'n', 'a', 'l', 'o', 'g', 'O', 'u', 't', 'p', 'u', 't']>::new().serialize(writer, whitespace)?;
                is_first = false;
            }
            if self.contains(PinFlags::RESERVED) {
                if !is_first {
                    if whitespace {
                        <Char![' ', '|', ' ']>::new().serialize(writer, whitespace)?;
                    } else {
                        <Char!['|']>::new().serialize(writer, whitespace)?;
                    }
                }
                <Char!['r', 'e', 's', 'e', 'r', 'v', 'e', 'd']>::new().serialize(writer, whitespace)?;
            }
        }
        Ok(())
    }
}