use arduino_hal::hal::port;
use arduino_hal::prelude::{_embedded_hal_serial_Read as Read, _embedded_hal_serial_Write as Write};

use crate::packet;
use crate::packet::Char;

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
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        match self {
            Self::A0 => <Char!['a', '0']>::new().serialize(writer, whitespace),
            Self::A1 => <Char!['a', '1']>::new().serialize(writer, whitespace),
            Self::A2 => <Char!['a', '2']>::new().serialize(writer, whitespace),
            Self::A3 => <Char!['a', '3']>::new().serialize(writer, whitespace),
            Self::A4 => <Char!['a', '4']>::new().serialize(writer, whitespace),
            Self::A5 => <Char!['a', '5']>::new().serialize(writer, whitespace),
            Self::D0 => <Char!['d', '0']>::new().serialize(writer, whitespace),
            Self::D1 => <Char!['d', '1']>::new().serialize(writer, whitespace),
            Self::D2 => <Char!['d', '2']>::new().serialize(writer, whitespace),
            Self::D3 => <Char!['d', '3']>::new().serialize(writer, whitespace),
            Self::D4 => <Char!['d', '4']>::new().serialize(writer, whitespace),
            Self::D5 => <Char!['d', '5']>::new().serialize(writer, whitespace),
            Self::D6 => <Char!['d', '6']>::new().serialize(writer, whitespace),
            Self::D7 => <Char!['d', '7']>::new().serialize(writer, whitespace),
            Self::D8 => <Char!['d', '8']>::new().serialize(writer, whitespace),
            Self::D9 => <Char!['d', '9']>::new().serialize(writer, whitespace),
            Self::D10 => <Char!['d', '1', '0']>::new().serialize(writer, whitespace),
            Self::D11 => <Char!['d', '1', '1']>::new().serialize(writer, whitespace),
            Self::D12 => <Char!['d', '1', '2']>::new().serialize(writer, whitespace),
            Self::D13 => <Char!['d', '1', '3']>::new().serialize(writer, whitespace),
        }
    }
}

impl packet::PacketDeserialize for PinId {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, _: bool)
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
            b'a' => match nb::block!(reader.read())? {
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
            b'd' => match nb::block!(reader.read())? {
                b'0' => complete!(PinId::D0),
                b'1' => match nb::block!(reader.read())? {
                    b'0' => complete!(PinId::D10),
                    b'1' => complete!(PinId::D11),
                    b'2' => complete!(PinId::D12),
                    b'3' => complete!(PinId::D13),
                    byte => {
                        *peek = Some(byte);
                        Ok(PinId::D1)
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
            byte => {
                *peek = Some(byte);
                Err(())
            },
        })
    }
}