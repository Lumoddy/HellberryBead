use arduino_hal::prelude::{_embedded_hal_serial_Read as Read};
use ufmt::{uDisplay, uWrite, uwrite};

use crate::packet::{Deserialize, is_literal_char, is_whitespace};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinMode {
    Ignored,
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    AnalogOutput,
}

impl Deserialize for PinMode {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<PinMode> {
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
            b'0' => match nb::block!(reader.read())? {
                byte if is_literal_char(byte) => {
                    *peek = Some(byte);
                    Err(())
                },
                byte => {
                    *peek = Some(byte);
                    Ok(Some(PinMode::Ignored))
                },
            },
            b'I' => match nb::block!(reader.read())? {
                b'G' => match nb::block!(reader.read())? {
                    b'N' => match nb::block!(reader.read())? {
                        b'O' => match nb::block!(reader.read())? {
                            b'R' => match nb::block!(reader.read())? {
                                b'E' => match nb::block!(reader.read())? {
                                    b'D' => match nb::block!(reader.read())? {
                                        byte if is_literal_char(byte) => {
                                            *peek = Some(byte);
                                            Err(())
                                        },
                                        byte => {
                                            *peek = Some(byte);
                                            Ok(Some(PinMode::Ignored))
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
            b'D' => match nb::block!(reader.read())? {
                b'I' => match nb::block!(reader.read())? {
                    b'G' => match nb::block!(reader.read())? {
                        b'I' => match nb::block!(reader.read())? {
                            b'T' => match nb::block!(reader.read())? {
                                b'A' => match nb::block!(reader.read())? {
                                    b'L' => match nb::block!(reader.read())? {
                                        b'_' => match nb::block!(reader.read())? {
                                            b'I' => match nb::block!(reader.read())? {
                                                b'N' => match nb::block!(reader.read())? {
                                                    b'P' => match nb::block!(reader.read())? {
                                                        b'U' => match nb::block!(reader.read())? {
                                                            b'T' => match nb::block!(reader.read())? {
                                                                byte if is_literal_char(byte) => {
                                                                    *peek = Some(byte);
                                                                    Err(())
                                                                },
                                                                byte => {
                                                                    *peek = Some(byte);
                                                                    Ok(Some(PinMode::DigitalInput))
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
                                            b'O' => match nb::block!(reader.read())? {
                                                b'U' => match nb::block!(reader.read())? {
                                                    b'T' => match nb::block!(reader.read())? {
                                                        b'P' => match nb::block!(reader.read())? {
                                                            b'U' => match nb::block!(reader.read())? {
                                                                b'T' => match nb::block!(reader.read())? {
                                                                    byte if is_literal_char(byte) => {
                                                                        *peek = Some(byte);
                                                                        Err(())
                                                                    },
                                                                    byte => {
                                                                        *peek = Some(byte);
                                                                        Ok(Some(PinMode::DigitalOutput))
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
            b'A' => match nb::block!(reader.read())? {
                b'N' => match nb::block!(reader.read())? {
                    b'A' => match nb::block!(reader.read())? {
                        b'L' => match nb::block!(reader.read())? {
                            b'O' => match nb::block!(reader.read())? {
                                b'G' => match nb::block!(reader.read())? {
                                    b'_' => match nb::block!(reader.read())? {
                                        b'I' => match nb::block!(reader.read())? {
                                            b'N' => match nb::block!(reader.read())? {
                                                b'P' => match nb::block!(reader.read())? {
                                                    b'U' => match nb::block!(reader.read())? {
                                                        b'T' => match nb::block!(reader.read())? {
                                                            byte if is_literal_char(byte) => {
                                                                *peek = Some(byte);
                                                                Err(())
                                                            },
                                                            byte => {
                                                                *peek = Some(byte);
                                                                Ok(Some(PinMode::DigitalInput))
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
                                        b'O' => match nb::block!(reader.read())? {
                                            b'U' => match nb::block!(reader.read())? {
                                                b'T' => match nb::block!(reader.read())? {
                                                    b'P' => match nb::block!(reader.read())? {
                                                        b'U' => match nb::block!(reader.read())? {
                                                            b'T' => match nb::block!(reader.read())? {
                                                                byte if is_literal_char(byte) => {
                                                                    *peek = Some(byte);
                                                                    Err(())
                                                                },
                                                                byte => {
                                                                    *peek = Some(byte);
                                                                    Ok(Some(PinMode::DigitalOutput))
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

impl uDisplay for PinMode {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized {
        match self {
            Self::Ignored => uwrite!(f, "IGNORED"),
            Self::DigitalInput => uwrite!(f, "DIGITAL_INPUT"),
            Self::DigitalOutput => uwrite!(f, "DIGITAL_OUTPUT"),
            Self::AnalogInput => uwrite!(f, "ANALOG_INPUT"),
            Self::AnalogOutput => uwrite!(f, "ANALOG_OUTPUT"),
        }
    }
}