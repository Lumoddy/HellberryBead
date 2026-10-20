use arduino_hal::prelude::{_embedded_hal_serial_Read as Read, _embedded_hal_serial_Write as Write};

use crate::packet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinMode {
    Ignored,
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    AnalogOutput,
}

impl packet::PacketSerialize for PinMode {
    fn serialize<W: Write<u8>>(&self, writer: &mut W) -> Result<(), W::Error> {
        macro_rules! write {
            ( $( $byte:expr ),* $(,)? ) => {
                { $( nb::block!(writer.write( $byte ))?; )* }
            };
        }

        match *self {
            Self::Ignored => write!(b'I', b'g', b'n', b'o', b'r', b'e', b'd'),
            Self::DigitalInput => write!(b'D', b'i', b'g', b'i', b't', b'a', b'l', b'I', b'n', b'p', b'u', b't'),
            Self::DigitalOutput => write!(b'D', b'i', b'g', b'i', b't', b'a', b'l', b'O', b'u', b't', b'p', b'u', b't'),
            Self::AnalogInput => write!(b'A', b'n', b'a', b'l', b'o', b'g', b'I', b'n', b'p', b'u', b't'),
            Self::AnalogOutput => write!(b'A', b'n', b'a', b'l', b'o', b'g', b'O', b'u', b't', b'p', b'u', b't'),
        }
        Ok(())
    }
}

impl packet::PacketDeserialize for PinMode {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>)
        -> Result<Result<Self, Self::Error>, R::Error> {
        let mut first = match peek.take() {
            Some(x) => x,
            None => nb::block!(reader.read())?,
        };

        while let b' ' | b'\r' | b'\n' = first {
            first = nb::block!(reader.read())?;
        }

        // Treat this like lexing an identifier, which means that an otherwise
        // valid "Ignored" should not return a valid value if its followed by
        // eg. an "_" making "Ignored_".
        macro_rules! complete {
            ($result:expr $(,)?) => {
                match nb::block!(reader.read())? {
                    b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' => Err(()),
                    byte => {
                        *peek = Some(byte);
                        Ok( $result )
                    },
                }
            };
        }

        macro_rules! sequence {
            ($($pat:pat),*$(,)? $( => $result:expr )?) => {
                {
                    $(
                        #[allow(unused_parens)]
                        let ($pat) = nb::block!(reader.read())? else {
                            return Ok(Err(()));
                        };
                    )*

                    $( complete!( $result ) )?
                }
            };
        }

        Ok(match first {
            b'_' => complete!(PinMode::Ignored),
            b'I' | b'i' => match nb::block!(reader.read())? {
                b'g' => sequence!(
                    b'n',
                    b'o',
                    b'r',
                    b'e',
                    b'd',
                    => PinMode::Ignored),
                byte => {
                    *peek = Some(byte);
                    Ok(PinMode::Ignored)
                },
            },
            b'D' | b'd' => match nb::block!(reader.read())? {
                b'I' => complete!(PinMode::DigitalInput),
                b'O' | b'o' => complete!(PinMode::DigitalOutput),
                b'i' => match nb::block!(reader.read())? {
                    b'g' => {
                        sequence!(
                            b'i',
                            b't',
                            b'a',
                            b'l');

                        match nb::block!(reader.read())? {
                            b'I' | b'i' => sequence!(
                                b'n',
                                b'p',
                                b'u',
                                b't',
                                => PinMode::DigitalInput),
                            b'O' | b'o' => sequence!(
                                b'u',
                                b't',
                                b'p',
                                b'u',
                                b't',
                                => PinMode::DigitalOutput),
                            byte => {
                                *peek = Some(byte);
                                Ok(PinMode::Ignored)
                            },
                        }
                    },
                    byte => {
                        *peek = Some(byte);
                        Ok(PinMode::DigitalInput)
                    },
                },
                byte => {
                    *peek = Some(byte);
                    Ok(PinMode::Ignored)
                },
            },
            b'A' | b'a' => match nb::block!(reader.read())? {
                b'I' | b'i' => complete!(PinMode::AnalogInput),
                b'O' | b'o' => complete!(PinMode::AnalogOutput),
                b'n' => {
                    sequence!(
                        b'a',
                        b'l',
                        b'o',
                        b'g');

                    match nb::block!(reader.read())? {
                        b'I' | b'i' => sequence!(
                            b'n',
                            b'p',
                            b'u',
                            b't',
                            => PinMode::AnalogInput),
                        b'O' | b'o' => sequence!(
                            b'u',
                            b't',
                            b'p',
                            b'u',
                            b't',
                            => PinMode::AnalogOutput),
                        byte => {
                            *peek = Some(byte);
                            Ok(PinMode::Ignored)
                        },
                    }
                },
                byte => {
                    *peek = Some(byte);
                    Ok(PinMode::Ignored)
                },
            },
            byte => {
                *peek = Some(byte);
                Err(())
            },
        })
    }
}