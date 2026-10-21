use arduino_hal::prelude::_embedded_hal_serial_Write as Write;

use crate::pin::{PinFlags, PinId, PinMode};

use super::{Field, PacketSerialize};
use super::char::Char;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Packet {
    /// Encoded as `"pong"`.
    Pong,
    /// Encoded as `"pins.{pin}.power = {power}"`.
    PinPowerResponse { pin: PinId, power: u16 },
    /// Encoded as `"pins.{pin}.mode = {mode}"`.
    PinModeResponse { pin: PinId, mode: PinMode },
    /// Encoded as `"pins.{pin}.flags = {flags}"`.
    PinFlagsResponse { pin: PinId, flags: PinFlags },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Encoded as `"pins.{pin}.error(\"Unsupported mode: {mode}\")"`.
    UnsupportedPinMode { pin: PinId, mode: PinMode },
}

impl PacketSerialize for Packet {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        match self {
            Self::Pong => {
                <Char!['p', 'o', 'n', 'g']>::new().serialize(writer, whitespace)?;
                Ok(())
            },
            Self::PinPowerResponse { pin, power } => {
                <Char!['p', 'i', 'n', 's']>::new().serialize(writer, whitespace)?;
                Field(pin).serialize(writer, whitespace)?;
                Field(<Char!['p', 'o', 'w', 'e', 'r']>::new()).serialize(writer, whitespace)?;
                <Char![' ']>::new().serialize(writer, whitespace)?;
                <Char!['=']>::new().serialize(writer, whitespace)?;
                <Char![' ']>::new().serialize(writer, whitespace)?;
                power.serialize(writer, whitespace)?;
                Ok(())
            },
            Self::PinModeResponse { pin, mode } => {
                <Char!['p', 'i', 'n', 's']>::new().serialize(writer, whitespace)?;
                Field(pin).serialize(writer, whitespace)?;
                Field(<Char!['m', 'o', 'd', 'e']>::new()).serialize(writer, whitespace)?;
                <Char![' ']>::new().serialize(writer, whitespace)?;
                <Char!['=']>::new().serialize(writer, whitespace)?;
                <Char![' ']>::new().serialize(writer, whitespace)?;
                mode.serialize(writer, whitespace)?;
                Ok(())
            },
            Self::PinFlagsResponse { pin, flags } => {
                <Char!['p', 'i', 'n', 's']>::new().serialize(writer, whitespace)?;
                Field(pin).serialize(writer, whitespace)?;
                Field(<Char!['f', 'l', 'a', 'g', 's']>::new()).serialize(writer, whitespace)?;
                <Char![' ']>::new().serialize(writer, whitespace)?;
                <Char!['=']>::new().serialize(writer, whitespace)?;
                <Char![' ']>::new().serialize(writer, whitespace)?;
                flags.serialize(writer, whitespace)?;
                Ok(())
            },
        }
    }
}

impl PacketSerialize for Error {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        match self {
            Self::UnsupportedPinMode { pin, mode } => {
                <Char!['p', 'i', 'n', 's']>::new().serialize(writer, whitespace)?;
                Field(pin).serialize(writer, whitespace)?;
                Field(<Char!['e', 'r', 'r', 'o', 'r']>::new()).serialize(writer, whitespace)?;
                <Char!['(']>::new().serialize(writer, whitespace)?;
                <Char!['"']>::new().serialize(writer, whitespace)?;
                <Char!['U', 'n', 's', 'u', 'p', 'p', 'o', 'r', 't', 'e', 'd', ' ', 'm', 'o', 'd', 'e', ':', ' ']>::new().serialize(writer, whitespace)?;
                mode.serialize(writer, whitespace)?;
                <Char!['"']>::new().serialize(writer, whitespace)?;
                <Char![')']>::new().serialize(writer, whitespace)?;
                Ok(())
            },
        }
    }
}