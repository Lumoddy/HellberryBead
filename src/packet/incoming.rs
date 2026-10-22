use arduino_hal::prelude::_embedded_hal_serial_Read as Read;

use crate::packet::{Field, literal::*};
use crate::pin::{PinId, PinMode};

use super::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Packet {
    /// Encoded as `"ping"`.
    Ping,
    /// Encoded as `"config"`.
    Config,
    /// Encoded as `"pins.{pin}.getPower()"`.
    GetPinPower { pin: PinId },
    /// Encoded as `"pins.{pin}.getMode()"`.
    GetPinMode { pin: PinId },
    /// Encoded as `"pins.{pin}.flags"`.
    PinFlags { pin: PinId },
    /// Encoded as `"pins.{pin}.setPower({power})"`.
    SetPinPower { pin: PinId, power: u8 },
    /// Encoded as `"pins.{pin}.setMode({mode})"`.
    SetPinMode { pin: PinId, mode: PinMode },
}

impl Deserialize for Packet {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Option::deserialize(reader, peek, whitespace).map(|x| x.and_then(|x| x.ok_or(())))
    }
}

impl Deserialize for Option<Packet> {
    type Error = ();

    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Ok(match <_>::deserialize(reader, peek, whitespace)? {
            Err(()) => Err(()),
            Ok(None) => {
                Ok(None)
            },
            Ok(Some(LiteralField::Ping)) => {
                Ok(Some(Packet::Ping))
            },
            Ok(Some(LiteralField::Config)) => {
                Ok(Some(Packet::Config))
            },
            Ok(Some(LiteralField::Pins)) => match <_>::deserialize(reader, peek, whitespace)? {
                Err(()) => Err(()),
                Ok(Field(pin)) => match <_>::deserialize(reader, peek, whitespace)? {
                    Err(()) => Err(()),
                    Ok(Field(LiteralPinsField::GetPower)) => match <_>::deserialize(reader, peek, whitespace)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match <_>::deserialize(reader, peek, whitespace)? {
                            Err(()) => Err(()),
                            Ok(LiteralClose) => {
                                Ok(Some(Packet::GetPinPower { pin }))
                            },
                        },
                    },
                    Ok(Field(LiteralPinsField::GetMode)) => match <_>::deserialize(reader, peek, whitespace)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match <_>::deserialize(reader, peek, whitespace)? {
                            Err(()) => Err(()),
                            Ok(LiteralClose) => {
                                Ok(Some(Packet::GetPinMode { pin }))
                            },
                        },
                    },
                    Ok(Field(LiteralPinsField::Flags)) => {
                        Ok(Some(Packet::PinFlags { pin }))
                    },
                    Ok(Field(LiteralPinsField::SetPower)) => match <_>::deserialize(reader, peek, whitespace)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match <_>::deserialize(reader, peek, whitespace)? {
                            Err(()) => Err(()),
                            Ok(power) => match <_>::deserialize(reader, peek, whitespace)? {
                                Err(()) => Err(()),
                                Ok(Some(LiteralComma) | None) => match <_>::deserialize(reader, peek, whitespace)? {
                                    Err(()) => Err(()),
                                    Ok(LiteralClose) => {
                                        Ok(Some(Packet::SetPinPower { pin, power }))
                                    },
                                },
                            },
                        },
                    },
                    Ok(Field(LiteralPinsField::SetMode)) => match <_>::deserialize(reader, peek, whitespace)? {
                        Err(()) => Err(()),
                        Ok(LiteralOpen) => match <_>::deserialize(reader, peek, whitespace)? {
                            Err(()) => Err(()),
                            Ok(mode) => match <_>::deserialize(reader, peek, whitespace)? {
                                Err(()) => Err(()),
                                Ok(Some(LiteralComma) | None) => match <_>::deserialize(reader, peek, whitespace)? {
                                    Err(()) => Err(()),
                                    Ok(LiteralClose) => {
                                        Ok(Some(Packet::SetPinMode { pin, mode }))
                                    },
                                },
                            },
                        },
                    },
                },
            },
        })
    }
}