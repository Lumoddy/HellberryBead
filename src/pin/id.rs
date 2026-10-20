use arduino_hal::hal::port;
use arduino_hal::prelude::{_embedded_hal_serial_Read as Read, _embedded_hal_serial_Write as Write};

use crate::packet;

pub(super) trait ConstPinId {
    const ID: PinId;
}

macro_rules! impl_id
{
    ($($id:ident($port:ty) = $name:literal),*$(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum PinId {
            $( $id ,)*
        }

        impl PinId {
            pub const fn name(&self) -> &'static str {
                match *self {
                    $( Self:: $id => $name ,)*
                }
            }
        }

        $(
            impl ConstPinId for $port {
                const ID: PinId = PinId:: $id;
            }
        )*
    };
}

impl_id! {
    A0(port::PC0) = "A0",
    A1(port::PC1) = "A1",
    A2(port::PC2) = "A2",
    A3(port::PC3) = "A3",
    A4(port::PC4) = "A4",
    A5(port::PC5) = "A5",
    D0(port::PD0) = "D0",
    D1(port::PD1) = "D1",
    D2(port::PD2) = "D2",
    D3(port::PD3) = "D3",
    D4(port::PD4) = "D4",
    D5(port::PD5) = "D5",
    D6(port::PD6) = "D6",
    D7(port::PD7) = "D7",
    D8(port::PB0) = "D8",
    D9(port::PB1) = "D9",
    D10(port::PB2) = "D10",
    D11(port::PB3) = "D11",
    D12(port::PB4) = "D12",
    D13(port::PB5) = "D13",
}

impl packet::PacketSerialize for PinId {
    fn serialize<W: Write<u8>>(&self, writer: &mut W) -> Result<(), W::Error> {
        macro_rules! write {
            ($($byte:expr),*$(,)?) => {
                { $( nb::block!(writer.write($byte))?; )* }
            };
        }

        match *self {
            Self::A0 => write!(b'A', b'0'),
            Self::A1 => write!(b'A', b'1'),
            Self::A2 => write!(b'A', b'2'),
            Self::A3 => write!(b'A', b'3'),
            Self::A4 => write!(b'A', b'4'),
            Self::A5 => write!(b'A', b'5'),
            Self::D0 => write!(b'D', b'0'),
            Self::D1 => write!(b'D', b'1'),
            Self::D2 => write!(b'D', b'2'),
            Self::D3 => write!(b'D', b'3'),
            Self::D4 => write!(b'D', b'4'),
            Self::D5 => write!(b'D', b'5'),
            Self::D6 => write!(b'D', b'6'),
            Self::D7 => write!(b'D', b'7'),
            Self::D8 => write!(b'D', b'8'),
            Self::D9 => write!(b'D', b'9'),
            Self::D10 => write!(b'D', b'1', b'0'),
            Self::D11 => write!(b'D', b'1', b'1'),
            Self::D12 => write!(b'D', b'1', b'2'),
            Self::D13 => write!(b'D', b'1', b'3'),
        }
        Ok(())
    }
}

impl packet::PacketDeserialize for PinId {
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
        // valid "D2" should not return a valid value if its followed by eg. a
        // "3" making "D23".
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

        Ok(match first {
            b'A' | b'a' => match nb::block!(reader.read())? {
                b'0' => complete!(PinId::A0),
                b'1' => complete!(PinId::A1),
                b'2' => complete!(PinId::A2),
                b'3' => complete!(PinId::A3),
                b'4' => complete!(PinId::A4),
                b'5' => complete!(PinId::A5),
                byte => {
                    *peek = Some(byte);
                    Ok(PinId::A0)
                },
            },
            b'D' | b'd' => match nb::block!(reader.read())? {
                b'0' => complete!(PinId::D0),
                b'1' => match nb::block!(reader.read())? {
                    b'0' => complete!(PinId::D10),
                    b'1' => complete!(PinId::D11),
                    b'2' => complete!(PinId::D12),
                    b'3' => complete!(PinId::D13),
                    byte => {
                        *peek = Some(byte);
                        Ok(PinId::D0)
                    },
                },
                b'2' => complete!(PinId::D2),
                b'3' => complete!(PinId::D3),
                b'4' => complete!(PinId::D4),
                b'5' => complete!(PinId::D5),
                b'6' => complete!(PinId::D6),
                b'7' => complete!(PinId::D7),
                b'8' => complete!(PinId::D8),
                b'9' => complete!(PinId::D9),
                byte => {
                    *peek = Some(byte);
                    Err(())
                },
            },
            b'0' => complete!(PinId::D0),
            b'1' => match nb::block!(reader.read())? {
                b'0' => complete!(PinId::D10),
                b'1' => complete!(PinId::D11),
                b'2' => complete!(PinId::D12),
                b'3' => complete!(PinId::D13),
                byte => {
                    *peek = Some(byte);
                    Ok(PinId::D0)
                },
            },
            b'2' => complete!(PinId::D2),
            b'3' => complete!(PinId::D3),
            b'4' => complete!(PinId::D4),
            b'5' => complete!(PinId::D5),
            b'6' => complete!(PinId::D6),
            b'7' => complete!(PinId::D7),
            b'8' => complete!(PinId::D8),
            b'9' => complete!(PinId::D9),
            byte => {
                *peek = Some(byte);
                Err(())
            },
        })
    }
}