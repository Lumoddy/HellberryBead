use arduino_hal::prelude::{_embedded_hal_serial_Read as Read};
use ufmt::{Formatter, uDisplay, uWrite, uwrite};

mod literal;

pub mod incoming;
pub mod outgoing;

pub const fn is_literal_char(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_')
}

pub const fn is_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t')
}

pub trait Deserialize: Sized {
    type Error;

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error>;
}

macro_rules! impl_int {
    ($($ty:ident: $size:literal),*$(,)?) => {
        $(
            impl Deserialize for $ty {
                type Error = ();

                fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
                    -> Result<Result<Self, Self::Error>, R::Error> {
                    let mut first = match peek.take() {
                        Some(x) => x,
                        None => nb::block!(reader.read())?,
                    };

                    if whitespace {
                        while is_whitespace(first) {
                            first = nb::block!(reader.read())?;
                        }
                    }

                    let mut buffer = [0u8; $size];
                    let mut i = 0;

                    match first {
                        b'_' => { },
                        byte @ (b'0'..=b'9' | b'-') => {
                            buffer[i] = byte;
                            i += 1;
                        },
                        byte => {
                            *peek = Some(byte);
                            return Ok(Err(()));
                        },
                    }

                    loop {
                        match nb::block!(reader.read())? {
                            b'_' => { },
                            byte @ b'0'..=b'9' => {
                                if i >= buffer.len() {
                                    return Ok(Err(()));
                                }
                                buffer[i] = byte;
                                i += 1;
                            },
                            byte => {
                                *peek = Some(byte);
                                break;
                            },
                        }
                    }

                    match core::str::from_utf8(&buffer[..i]).unwrap().parse::<$ty>() {
                        Ok(value) => Ok(Ok(value)),
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

impl<T: Deserialize<Error: Default>> Deserialize for Delimited<T> {
    type Error = T::Error;

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Ok(match T::deserialize(reader, peek, whitespace)? {
            Err(x) => Err(x),
            Ok(x) => {
                let mut first = match peek.take() {
                    Some(x) => x,
                    None => nb::block!(reader.read())?,
                };

                if whitespace {
                    while is_whitespace(first) {
                        first = nb::block!(reader.read())?;
                    }
                }

                match first {
                    b'\n' => Ok(Self(x)),
                    byte => {
                        *peek = Some(byte);
                        Err(<_>::default())
                    },
                }
            },
        })
    }
}

impl<T: uDisplay> uDisplay for Delimited<T> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized { uwrite!(f, "{}\n", self.0) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field<T>(pub T);

impl<T> From<T> for Field<T> {
    fn from(value: T) -> Self { Self(value) }
}

impl<T: Deserialize<Error: Default>> Deserialize for Field<T> {
    type Error = T::Error;

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or_else(<_>::default)))
    }
}

impl<T: Deserialize<Error: Default>> Deserialize for Option<Field<T>> {
    type Error = T::Error;

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        let mut first = match peek.take() {
            Some(x) => x,
            None => nb::block!(reader.read())?,
        };

        if whitespace {
            while is_whitespace(first) {
                first = nb::block!(reader.read())?;
            }
        }

        Ok(match first {
            b'.' => {
                match T::deserialize(reader, peek, whitespace)? {
                    Err(x) => Err(x),
                    Ok(x) => Ok(Some(Field(x))),
                }
            },
            b'[' => {
                let mut first = match peek.take() {
                    Some(x) => x,
                    None => nb::block!(reader.read())?,
                };

                if whitespace {
                    while is_whitespace(first) {
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
                                            while is_whitespace(first) {
                                                byte = nb::block!(reader.read())?;
                                            }
                                        }

                                        match byte {
                                            b']' => {
                                                Ok(Some(Field(x)))
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

impl<T: uDisplay> uDisplay for Field<T> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized { uwrite!(f, ".{}", self.0) }
}