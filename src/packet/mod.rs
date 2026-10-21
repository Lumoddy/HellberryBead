use core::convert::Infallible;

use arduino_hal::prelude::{_embedded_hal_serial_Read as Read, _embedded_hal_serial_Write as Write};
use ufmt::{Formatter, uDisplay, uWrite};

mod char;
pub use char::*;

mod literal;
pub use literal::*;

pub mod incoming;
pub mod outgoing;

pub trait PacketDeserialize<Word = u8>: Sized {
    type Error;

    fn deserialize<R: Read<Word>>(reader: &mut R, peek: &mut Option<Word>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error>;
}

pub trait PacketSerialize<Word = u8> {
    fn serialize<W: Write<Word>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
}

impl<T: PacketSerialize<Word> + ?Sized, Word> PacketSerialize<Word> for &T {
    fn serialize<W: Write<Word>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        T::serialize(self, writer, whitespace)
    }
}

impl<T: PacketSerialize<Word> + ?Sized, Word> PacketSerialize<Word> for &mut T {
    fn serialize<W: Write<Word>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        T::serialize(self, writer, whitespace)
    }
}

impl PacketSerialize for bool {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        match *self {
            true => <Char!['t', 'r', 'u', 'e']>::new().serialize(writer, whitespace),
            false => <Char!['f', 'a', 'l', 's', 'e']>::new().serialize(writer, whitespace),
        }
    }
}

impl PacketDeserialize for bool {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        let mut first = match peek.take() {
            Some(x) => x,
            None => nb::block!(reader.read())?,
        };

        if whitespace {
            while let b' ' | b'\r' | b'\n' = first {
                first = nb::block!(reader.read())?;
            }
        }

        // Treat this like lexing an identifier, which means that an otherwise
        // valid "true" should not return a valid value if its followed by eg.
        // an "r" making "truer".
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
            b'0' => complete!(false),
            b'1' => complete!(true),
            b'T' | b't' => sequence!(
                b'r',
                b'u',
                b'e',
                => true),
            b'F' | b'f' => sequence!(
                b'a',
                b'l',
                b's',
                b'e',
                => false),
            byte => {
                *peek = Some(byte);
                Ok(false)
            },
        })
    }
}

macro_rules! impl_int {
    ($($ty:ident: $size:literal),*$(,)?) => {
        $(
            impl PacketSerialize for $ty {
                fn serialize<W: Write<u8>>(&self, writer: &mut W, _: bool) -> Result<(), W::Error> {
                    struct Writer<'a, W: Write<u8>> { writer: &'a mut W }

                    impl<W: Write<u8>> uWrite for Writer<'_, W> {
                        type Error = W::Error;

                        fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
                            for byte in s.bytes() {
                                nb::block!(self.writer.write(byte))?;
                            }
                            Ok(())
                        }
                    }

                    uDisplay::fmt(self, &mut Formatter::new(&mut Writer { writer }))
                }
            }

            impl PacketDeserialize for $ty {
                type Error = ();

                fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
                    -> Result<Result<Self, Self::Error>, R::Error> {
                    let mut first = match peek.take() {
                        Some(x) => x,
                        None => nb::block!(reader.read())?,
                    };

                    if whitespace {
                        while let b' ' | b'\r' | b'\n' = first {
                            first = nb::block!(reader.read())?;
                        }
                    }

                    let mut buffer = [0u8; $size];
                    let mut i = 0;

                    match first {
                        b'0'..=b'9' | b'-' => {
                            buffer[i] = first;
                            i += 1;
                        },
                        byte => {
                            *peek = Some(byte);
                            return Ok(Err(()));
                        },
                    }

                    loop {
                        if i >= buffer.len() {
                            return Ok(Err(()));
                        }

                        match nb::block!(reader.read())? {
                            b'_' => { },
                            b'0'..=b'9' => {
                                buffer[i] = nb::block!(reader.read())?;
                                i += 1;
                            },
                            byte => {
                                *peek = Some(byte);
                                break;
                            },
                        }
                    }

                    match core::str::from_utf8(&buffer[..i]) {
                        Ok(s) => match s.parse::<$ty>() {
                            Ok(value) => Ok(Ok(value)),
                            Err(_) => Ok(Err(())),
                        },
                        Err(_) => Ok(Err(())),
                    }
                }
            }
        )*
    };
}

impl_int!(u8: 3, u16: 5, u32: 10, u64: 20, u128: 39, i8: 4, i16: 6, i32: 11, i64: 20, i128: 40);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Delimited<T>(pub T);

impl<T> From<T> for Delimited<T> {
    fn from(value: T) -> Self { Self(value) }
}

impl<T: PacketDeserialize<Error: Default>> PacketDeserialize for Delimited<T> {
    type Error = T::Error;

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Ok(match T::deserialize(reader, peek, whitespace)? {
            Err(x) => Err(x),
            Ok(x) => match LiteralSemiColon::deserialize(reader, peek, whitespace)? {
                Err(()) => Err(T::Error::default()),
                Ok(LiteralSemiColon::SemiColon(_)) => {
                    Ok(Self(x))
                },
            },
        })
    }
}

impl<T: PacketSerialize> PacketSerialize for Delimited<T> {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        self.0.serialize(writer, whitespace)?;
        <Char![';']>::new().serialize(writer, whitespace)?;
        Ok(())
    }
}

impl PacketDeserialize for () {
    type Error = Infallible;

    fn deserialize<R: Read<u8>>(_: &mut R, _: &mut Option<u8>, _: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Ok(Ok(()))
    }
}

impl PacketSerialize for () {
    fn serialize<W: Write<u8>>(&self, _: &mut W, _: bool) -> Result<(), W::Error> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field<T>(pub T);

impl<T> From<T> for Field<T> {
    fn from(value: T) -> Self { Self(value) }
}

impl<T: PacketDeserialize<Error: Default>> PacketDeserialize for Field<T> {
    type Error = T::Error;

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        let mut first = match peek.take() {
            Some(x) => x,
            None => nb::block!(reader.read())?,
        };

        if whitespace {
            while let b' ' | b'\r' | b'\n' = first {
                first = nb::block!(reader.read())?;
            }
        }

        Ok(match first {
            b'.' => {
                match T::deserialize(reader, peek, whitespace)? {
                    Err(x) => Err(x),
                    Ok(x) => Ok(Self(x)),
                }
            },
            b'[' => {
                let mut first = match peek.take() {
                    Some(x) => x,
                    None => nb::block!(reader.read())?,
                };

                if whitespace {
                    while let b' ' | b'\r' | b'\n' = first {
                        first = nb::block!(reader.read())?;
                    }
                }

                match first {
                    b'\"' => {
                        match T::deserialize(reader, peek, false)? {
                            Err(x) => Err(x),
                            Ok(x) => {
                                let byte = match peek.take() {
                                    Some(x) => x,
                                    None => nb::block!(reader.read())?,
                                };

                                match byte {
                                    b'\"' => {
                                        let mut byte = match peek.take() {
                                            Some(x) => x,
                                            None => nb::block!(reader.read())?,
                                        };

                                        if whitespace {
                                            while let b' ' | b'\r' | b'\n' = first {
                                                byte = nb::block!(reader.read())?;
                                            }
                                        }

                                        match byte {
                                            b']' => {
                                                Ok(Self(x))
                                            },
                                            byte => {
                                                *peek = Some(byte);
                                                Err(<_>::default())
                                            },
                                        }
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Err(<_>::default())
                                    },
                                }
                            },
                        }
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(<_>::default())
                    },
                }
            },
            byte => {
                *peek = Some(byte);
                Err(<_>::default())
            },
        })
    }
}

impl<T: PacketSerialize> PacketSerialize for Field<T> {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        <Char!['.']>::new().serialize(writer, whitespace)?;
        self.0.serialize(writer, whitespace)?;
        Ok(())
    }
}