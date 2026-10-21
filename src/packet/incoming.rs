use arduino_hal::prelude::_embedded_hal_serial_Read as Read;

use crate::packet::{Field, literal::*};
use crate::pin::{PinId, PinMode};

use super::PacketDeserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Packet {
    /// Encoded as `"ping"`.
    Ping,
    /// Encoded as `"pins.{pin}.getPower()"`.
    GetPinPower { pin: PinId },
    /// Encoded as `"pins.{pin}.getMode()"`.
    GetPinMode { pin: PinId },
    /// Encoded as `"pins.{pin}.flags"`.
    PinFlags { pin: PinId },
    /// Encoded as `"pins.{pin}.supports.{mode}"`.
    PinSupports { pin: PinId, mode: PinMode },
    /// Encoded as `"pins.{pin}.setPower({power})"`.
    SetPinPower { pin: PinId, power: u16 },
    /// Encoded as `"pins.{pin}.setMode({mode})"`.
    SetPinMode { pin: PinId, mode: PinMode },
}

macro_rules! impl_construct {
    (
        @ $error:ty, $reader:ident, $peek:ident, $whitespace:ident,
        { $($pat:pat => $($val:tt)::+ $(($($round:tt)*))? $({ $($curly:tt)* })?),*$(,)? }
    ) => {
        match PacketDeserialize::deserialize($reader, $peek, $whitespace) {
            Err(x) => Err(x),
            Ok(Err(x)) => Ok(Err(Into::<$error>::into(x))),
            $(
                Ok(Ok($pat)) => impl_construct!
                {
                    @ $error, $reader, $peek, $whitespace,
                    $($val)::+ $(($($round)*))? $({ $($curly)* })?
                },
            )*
        }
    };
    (
        @ $error:ty, $reader:ident, $peek:ident, $whitespace:ident,
        $($tt:tt)*
    ) => {
        Ok($($tt)*)
    };
    (
        $(use $($path:tt)::+;)*
        for $ty:ty where Error = $error:ty $(,)? { $($tt:tt)* }
    ) => {
        impl PacketDeserialize for $ty {
            type Error = $error;

            fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
                -> Result<Result<Self, Self::Error>, R::Error> {
                $(use $($path)::+;)*
                impl_construct! { @ $error, reader, peek, whitespace, { $($tt)* } }
            }
        }
    };
}

impl_construct! {
    use LiteralPingOrPins::*;
    use LiteralPinFunctionName::*;
    use LiteralOpen::*;
    use LiteralClose::*;
    use LiteralComma::*;

    for Packet
    where
        Error = (), {
        Ping(_) => Ok(Packet::Ping),
        Pins(_) => {
            Field(pin) => {
                Field(GetPower(_)) => {
                    Open(_) => {
                        Close(_) => Ok(Self::GetPinPower { pin }),
                    },
                },
                Field(GetMode(_)) => {
                    Open(_) => {
                        Close(_) => Ok(Self::GetPinPower { pin }),
                    },
                },
                Field(Flags(_)) => Ok(Self::PinFlags { pin }),
                Field(Supports(_)) => {
                    Field(mode) => Ok(Self::PinSupports { pin, mode }),
                },
                Field(SetPower(_)) => {
                    Open(_) => {
                        power => {
                            Some(Comma(_)) | None => {
                                Close(_) => Ok(Self::SetPinPower { pin, power }),
                            }
                        },
                    },
                },
                Field(SetMode(_)) => {
                    Open(_) => {
                        mode => {
                            Some(Comma(_)) | None => {
                                Close(_) => Ok(Self::SetPinMode { pin, mode }),
                            }
                        },
                    },
                },
            },
        },
    }
}