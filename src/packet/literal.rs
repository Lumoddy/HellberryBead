use arduino_hal::prelude::_embedded_hal_serial_Read as Read;

use super::PacketDeserialize;

macro_rules! impl_literal {
    (
        @ $reader:ident, $peek:ident, read(),
        { $($pat:pat => $($val:tt)::+ $(($($round:tt)*))? $({ $($curly:tt)* })?),*$(,)? }
    ) => {
        {
            let byte = nb::block!($reader.read())?;
            impl_literal!
            {
                @ $reader, $peek, byte,
                { $($pat => $($val)::+ $(($($round)*))? $({ $($curly)* })?,)* }
            }
        }
    };
    (
        @ $reader:ident, $peek:ident, $byte:ident,
        { $($pat:pat => $($val:tt)::+ $(($($round:tt)*))? $({ $($curly:tt)* })?),*$(,)? }
    ) => {
        match $byte {
            $(
                $pat => impl_literal!
                {
                    @ $reader, $peek, read(),
                    $($val)::+ $(($($round)*))? $({ $($curly)* })?
                },
            )*
            #[allow(unreachable_patterns)]
            byte => {
                *$peek = Some(byte);
                Err(())
            },
        }
    };
    (
        @ $reader:ident, $peek:ident, $byte:ident $(())?,
        $($tt:tt)*
    ) => {
        Ok($($tt)*)
    };
    (
        for $ty:ty { $($tt:tt)* }
    ) => {
        impl PacketDeserialize for $ty {
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

                Ok(impl_literal! { @ reader, peek, first, { $($tt)* } })
            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiteralDot;

impl_literal! {
    for LiteralDot {
        b'.' => Self,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiteralOpen;

impl_literal! {
    for LiteralOpen {
        b'(' => Self,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiteralClose;

impl_literal! {
    for LiteralClose {
        b')' => Self,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiteralComma;

impl_literal! {
    for LiteralComma {
        b',' => Self,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiteralSemiColon;

impl_literal! {
    for LiteralSemiColon {
        b';' => Self,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralPingOrPins {
    Ping,
    Pins,
}

impl_literal! {
    for LiteralPingOrPins {
        b'P' | b'p' => {
            b'i' => {
                b'n' => {
                    b'g' => Self::Ping,
                    b's' => Self::Pins,
                },
            },
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralPinFunctionName {
    GetPower,
    GetMode,
    Flags,
    SetPower,
    SetMode,
}

impl_literal! {
    for LiteralPinFunctionName {
        b'G' | b'g' => {
            b'e' => {
                b't' => {
                    b'P' | b'p' => {
                        b'o' => {
                            b'w' => {
                                b'e' => {
                                    b'r' => Self::GetPower,
                                },
                            },
                        },
                    },
                    b'M' | b'm' => {
                        b'o' => {
                            b'd' => {
                                b'e' => Self::GetMode,
                            },
                        },
                    },
                },
            },
        },
        b'F' | b'f' => {
            b'l' => {
                b'a' => {
                    b'g' => {
                        b's' => Self::Flags,
                    },
                },
            },
        },
        b'S' | b's' => {
            b'e' => {
                b't' => {
                    b'P' | b'p' => {
                        b'o' => {
                            b'w' => {
                                b'e' => {
                                    b'r' => Self::SetPower,
                                },
                            },
                        },
                    },
                    b'M' | b'm' => {
                        b'o' => {
                            b'd' => {
                                b'e' => Self::SetMode,
                            },
                        },
                    },
                },
            },
        },
    }
}

impl PacketDeserialize for Option<LiteralComma> {
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

        match first {
            b',' => {
                *peek = None;
                Ok(Ok(Some(LiteralComma)))
            },
            byte => {
                *peek = Some(byte);
                Ok(Ok(None))
            },
        }
    }
}