use arduino_hal::prelude::_embedded_hal_serial_Read as Read;

use crate::pin::{PinId, PinMode};

use super::PacketDeserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Packet {
    Ping,
    GetPinPower { pin: PinId },
    GetPinMode { pin: PinId },
    SetPinPower { pin: PinId, power: u16 },
    SetPinMode { pin: PinId, mode: PinMode },
}

impl PacketDeserialize for Packet {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Ok(match LiteralPingOrPins::deserialize(reader, peek)? {
            Err(()) => Err(()),
            Ok(LiteralPingOrPins::Ping) => {
                Ok(Packet::Ping)
            },
            Ok(LiteralPingOrPins::Pins) => match PinId::deserialize(reader, peek)? {
                Err(()) => Err(()),
                Ok(pin) => match LiteralPinFunctionName::deserialize(reader, peek)? {
                    Err(()) => Err(()),
                    Ok(LiteralPinFunctionName::GetPower) => match LiteralOpen::deserialize(reader, peek)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match LiteralClose::deserialize(reader, peek)? {
                            Err(()) => Err(()),
                            Ok(LiteralClose) => {
                                Ok(Self::GetPinPower { pin })
                            },
                        },
                    },
                    Ok(LiteralPinFunctionName::GetMode) => match LiteralOpen::deserialize(reader, peek)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match LiteralClose::deserialize(reader, peek)? {
                            Err(()) => Err(()),
                            Ok(LiteralClose) => {
                                Ok(Self::GetPinPower { pin })
                            },
                        },
                    },
                    Ok(LiteralPinFunctionName::SetPower) => match LiteralOpen::deserialize(reader, peek)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match u16::deserialize(reader, peek)? {
                            Err(()) => Err(()),
                            Ok(power) => match LiteralClose::deserialize(reader, peek)? {
                                Err(()) => Err(()),
                                Ok(LiteralClose) => {
                                    Ok(Self::SetPinPower { pin, power })
                                },
                            },
                        },
                    },
                    Ok(LiteralPinFunctionName::SetMode) => match LiteralOpen::deserialize(reader, peek)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match PinMode::deserialize(reader, peek)? {
                            Err(()) => Err(()),
                            Ok(mode) => match LiteralClose::deserialize(reader, peek)? {
                                Err(()) => Err(()),
                                Ok(LiteralClose) => {
                                    Ok(Self::SetPinMode { pin, mode })
                                },
                            },
                        },
                    },
                },
            },
        })
    }
}

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