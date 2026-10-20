use arduino_hal::prelude::_embedded_hal_serial_Read as Read;

use crate::packet::literal::*;
use crate::pin::{PinId, PinMode};

use super::PacketDeserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Packet {
    /// Encoded as `"ping"`.
    Ping,
    /// Encoded as `"pins.<pin>.getPower()"`.
    GetPinPower { pin: PinId },
    /// Encoded as `"pins.<pin>.getMode()"`.
    GetPinMode { pin: PinId },
    /// Encoded as `"pins.<pin>.flags"`.
    PinFlags { pin: PinId },
    /// Encoded as `"pins.<pin>.setPinPower(<power>)"`.
    SetPinPower { pin: PinId, power: u16 },
    /// Encoded as `"pins.<pin>.setPinMode(<mode>)"`.
    SetPinMode { pin: PinId, mode: PinMode },
}

macro_rules! impl_construct {
    (
        @ $reader:ident, $peek:ident,
        { $($pat:pat => $($val:tt)::+ $(($($round:tt)*))? $({ $($curly:tt)* })?),*$(,)? }
    ) => {
        match PacketDeserialize::deserialize($reader, $peek) {
            Err(x) => Err(x),
            $(
                Ok($pat) => impl_construct!
                {
                    @ $reader, $peek,
                    $($val)::+ $(($($round)*))? $({ $($curly)* })?
                },
            )*
        }
    };
    (
        @ $reader:ident, $peek:ident,
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
                impl_construct! { @ reader, peek, { $($tt)* } }
            }
        }
    };
}

impl_construct! {
    for Packet {
        Err(()) => Err(()),
        Ok(LiteralPingOrPins::Ping) => Ok(Packet::Ping),
        Ok(LiteralPingOrPins::Pins) => {
            Err(()) => Err(()),
            Ok(pin) => {
                Err(()) => Err(()),
                Ok(LiteralPinFunctionName::GetPower) => {
                    Err(()) => Err(()),
                    Ok(LiteralOpen) => {
                        Err(()) => Err(()),
                        Ok(LiteralClose) => Ok(Self::GetPinPower { pin }),
                    },
                },
                Ok(LiteralPinFunctionName::GetMode) => {
                    Err(()) => Err(()),
                    Ok(LiteralOpen) => {
                        Err(()) => Err(()),
                        Ok(LiteralClose) => Ok(Self::GetPinPower { pin }),
                    },
                },
                Ok(LiteralPinFunctionName::Flags) => {
                    Ok(Self::PinFlags { pin })
                },
                Ok(LiteralPinFunctionName::SetPower) => {
                    Err(()) => Err(()),
                    Ok(LiteralOpen) => {
                        Err(()) => Err(()),
                        Ok(power) => {
                            Err(()) => Err(()),
                            Ok(Some(LiteralComma) | None) => {
                                Err(()) => Err(()),
                                Ok(LiteralClose) => Ok(Self::SetPinPower { pin, power }),
                            }
                        },
                    },
                },
                Ok(LiteralPinFunctionName::SetMode) => {
                    Err(()) => Err(()),
                    Ok(LiteralOpen) => {
                        Err(()) => Err(()),
                        Ok(mode) => {
                            Err(()) => Err(()),
                            Ok(Some(LiteralComma) | None) => {
                                Err(()) => Err(()),
                                Ok(LiteralClose) => Ok(Self::SetPinMode { pin, mode }),
                            }
                        },
                    },
                },
            },
        },
    }
}