use arduino_hal::prelude::{_embedded_hal_serial_Read as Read};
use ufmt::{uDisplay, uWrite, uwrite};

use super::{Deserialize, is_literal_char, is_whitespace};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct LiteralOpen;

impl Deserialize for LiteralOpen {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<LiteralOpen> {
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

        Ok(match first {
            b'(' => Ok(Some(LiteralOpen)),
            byte => {
                *peek = Some(byte);
                Ok(None)
            },
        })
    }
}

impl uDisplay for LiteralOpen {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized { uwrite!(f, "(") }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct LiteralClose;

impl Deserialize for LiteralClose {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<LiteralClose> {
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

        Ok(match first {
            b')' => Ok(Some(LiteralClose)),
            byte => {
                *peek = Some(byte);
                Ok(None)
            },
        })
    }
}

impl uDisplay for LiteralClose {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized { uwrite!(f, ")") }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct LiteralComma;

impl Deserialize for LiteralComma {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<LiteralComma> {
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

        Ok(match first {
            b',' => Ok(Some(LiteralComma)),
            byte => {
                *peek = Some(byte);
                Ok(None)
            },
        })
    }
}

impl uDisplay for LiteralComma {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized { uwrite!(f, ",") }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LiteralField {
    Ping,
    Config,
    Pins,
}

impl Deserialize for LiteralField {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<LiteralField> {
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

        Ok(match first {
            b'p' => match nb::block!(reader.read())? {
                b'i' => match nb::block!(reader.read())? {
                    b'n' => match nb::block!(reader.read())? {
                        b'g' => match nb::block!(reader.read())? {
                            byte if is_literal_char(byte) => {
                                *peek = Some(byte);
                                Err(())
                            },
                            byte => {
                                *peek = Some(byte);
                                Ok(Some(LiteralField::Ping))
                            },
                        },
                        b's' => match nb::block!(reader.read())? {
                            byte if is_literal_char(byte) => {
                                *peek = Some(byte);
                                Err(())
                            },
                            byte => {
                                *peek = Some(byte);
                                Ok(Some(LiteralField::Pins))
                            },
                        },
                        byte => {
                            *peek = Some(byte);
                            Err(())
                        },
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(())
                    },
                },
                byte => {
                    *peek = Some(byte);
                    Err(())
                },
            },
            b'c' => match nb::block!(reader.read())? {
                b'o' => match nb::block!(reader.read())? {
                    b'n' => match nb::block!(reader.read())? {
                        b'f' => match nb::block!(reader.read())? {
                            b'i' => match nb::block!(reader.read())? {
                                b'g' => match nb::block!(reader.read())? {
                                    byte if is_literal_char(byte) => {
                                        *peek = Some(byte);
                                        Err(())
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Ok(Some(LiteralField::Config))
                                    },
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        byte => {
                            *peek = Some(byte);
                            Err(())
                        },
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(())
                    },
                },
                byte => {
                    *peek = Some(byte);
                    Err(())
                },
            },
            byte => {
                *peek = Some(byte);
                Ok(None)
            },
        })
    }
}

impl uDisplay for LiteralField {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized {
        match self {
            Self::Ping => uwrite!(f, "ping"),
            Self::Config => uwrite!(f, "config"),
            Self::Pins => uwrite!(f, "pins"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LiteralPinsField {
    GetPower,
    GetMode,
    Flags,
    Supports,
    SetPower,
    SetMode,
}

impl Deserialize for LiteralPinsField {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<LiteralPinsField> {
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

        Ok(match first {
            b'g' => match nb::block!(reader.read())? {
                b'e' => match nb::block!(reader.read())? {
                    b't' => match nb::block!(reader.read())? {
                        b'P' => match nb::block!(reader.read())? {
                            b'o' => match nb::block!(reader.read())? {
                                b'w' => match nb::block!(reader.read())? {
                                    b'e' => match nb::block!(reader.read())? {
                                        b'r' => match nb::block!(reader.read())? {
                                            byte if is_literal_char(byte) => {
                                                *peek = Some(byte);
                                                Err(())
                                            },
                                            byte => {
                                                *peek = Some(byte);
                                                Ok(Some(LiteralPinsField::GetPower))
                                            },
                                        },
                                        byte => {
                                            *peek = Some(byte);
                                            Err(())
                                        },
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Err(())
                                    },
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        b'M' => match nb::block!(reader.read())? {
                            b'o' => match nb::block!(reader.read())? {
                                b'd' => match nb::block!(reader.read())? {
                                    b'e' => match nb::block!(reader.read())? {
                                        byte if is_literal_char(byte) => {
                                            *peek = Some(byte);
                                            Err(())
                                        },
                                        byte => {
                                            *peek = Some(byte);
                                            Ok(Some(LiteralPinsField::GetMode))
                                        },
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Err(())
                                    },
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        byte => {
                            *peek = Some(byte);
                            Err(())
                        },
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(())
                    },
                },
                byte => {
                    *peek = Some(byte);
                    Err(())
                },
            },
            b'f' => match nb::block!(reader.read())? {
                b'l' => match nb::block!(reader.read())? {
                    b'a' => match nb::block!(reader.read())? {
                        b'g' => match nb::block!(reader.read())? {
                            b's' => match nb::block!(reader.read())? {
                                byte if is_literal_char(byte) => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Ok(Some(LiteralPinsField::Flags))
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        byte => {
                            *peek = Some(byte);
                            Err(())
                        },
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(())
                    },
                },
                byte => {
                    *peek = Some(byte);
                    Err(())
                },
            },
            b's' => match nb::block!(reader.read())? {
                b'e' => match nb::block!(reader.read())? {
                    b't' => match nb::block!(reader.read())? {
                        b'P' => match nb::block!(reader.read())? {
                            b'o' => match nb::block!(reader.read())? {
                                b'w' => match nb::block!(reader.read())? {
                                    b'e' => match nb::block!(reader.read())? {
                                        b'r' => match nb::block!(reader.read())? {
                                            byte if is_literal_char(byte) => {
                                                *peek = Some(byte);
                                                Err(())
                                            },
                                            byte => {
                                                *peek = Some(byte);
                                                Ok(Some(LiteralPinsField::SetPower))
                                            },
                                        },
                                        byte => {
                                            *peek = Some(byte);
                                            Err(())
                                        },
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Err(())
                                    },
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        b'M' => match nb::block!(reader.read())? {
                            b'o' => match nb::block!(reader.read())? {
                                b'd' => match nb::block!(reader.read())? {
                                    b'e' => match nb::block!(reader.read())? {
                                        byte if is_literal_char(byte) => {
                                            *peek = Some(byte);
                                            Err(())
                                        },
                                        byte => {
                                            *peek = Some(byte);
                                            Ok(Some(LiteralPinsField::SetMode))
                                        },
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Err(())
                                    },
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        byte => {
                            *peek = Some(byte);
                            Err(())
                        },
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(())
                    },
                },
                b'u' => match nb::block!(reader.read())? {
                    b'p' => match nb::block!(reader.read())? {
                        b'p' => match nb::block!(reader.read())? {
                            b'o' => match nb::block!(reader.read())? {
                                b'r' => match nb::block!(reader.read())? {
                                    b't' => match nb::block!(reader.read())? {
                                        b's' => match nb::block!(reader.read())? {
                                            byte if is_literal_char(byte) => {
                                                *peek = Some(byte);
                                                Err(())
                                            },
                                            byte => {
                                                *peek = Some(byte);
                                                Ok(Some(LiteralPinsField::Supports))
                                            },
                                        },
                                        byte => {
                                            *peek = Some(byte);
                                            Err(())
                                        },
                                    },
                                    byte => {
                                        *peek = Some(byte);
                                        Err(())
                                    },
                                },
                                byte => {
                                    *peek = Some(byte);
                                    Err(())
                                },
                            },
                            byte => {
                                *peek = Some(byte);
                                Err(())
                            },
                        },
                        byte => {
                            *peek = Some(byte);
                            Err(())
                        },
                    },
                    byte => {
                        *peek = Some(byte);
                        Err(())
                    },
                },
                byte => {
                    *peek = Some(byte);
                    Err(())
                },
            },
            byte => {
                *peek = Some(byte);
                Ok(None)
            },
        })
    }
}