use ufmt::{uDisplay, uWrite, uwrite};

use crate::pin::{self, PinFlags, PinId, PinMode};

#[derive(Clone, Copy)]
pub enum Packet<'a> {
    /// Encoded as `"ready"`.
    Ready,
    /// Encoded as `"pong"`.
    Pong,
    /// Encoded as `"pins.{pin}.power = {power}"`.
    PinPowerResponse { pin: PinId, power: u16 },
    /// Encoded as `"pins.{pin}.mode = {mode}"`.
    PinModeResponse { pin: PinId, mode: PinMode },
    /// Encoded as `"pins.{pin}.flags = {flags}"`.
    PinFlagsResponse { pin: PinId, flags: PinFlags },
    /// Encoded as `"error.malformedPacketAt()"`.
    MalformedPacket { pos: usize },
    /// Encoded as `"pins.{pin}.error.unsupportedMode({mode})"`.
    UnsupportedPinMode { pin: PinId, mode: PinMode },
    /// Encoded as `"pins.{pin}.error.getPowerDenied()"`.
    PinGetPowerDenied { pin: PinId },
    /// Encoded as `"pins.{pin}.error.setPowerDenied()"`.
    PinSetPowerDenied { pin: PinId },
    /// Encoded as `"config = { model: <model>, [new Pin(<name>, <flags>), ...] }`.
    Config { collection: &'a pin::Collection },
}

impl uDisplay for Packet<'_> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized {
        match self {
            Self::Ready => {
                uwrite!(f, "ready")
            },
            Self::Pong => {
                uwrite!(f, "pong")
            },
            Self::PinPowerResponse { pin, power } => {
                uwrite!(f, "pins.{}.power = {}", pin, power)
            },
            Self::PinModeResponse { pin, mode } => {
                uwrite!(f, "pins.{}.mode = {}", pin, mode)
            },
            Self::PinFlagsResponse { pin, flags } => {
                uwrite!(f, "pins.{}.flags = {}", pin, flags)
            },
            Self::MalformedPacket { pos } => {
                uwrite!(f, "error.malformedPacketAt({})", pos)
            },
            Self::UnsupportedPinMode { pin, mode } => {
                uwrite!(f, "pins.{}.error.unsupportedMode({})", pin, mode)
            },
            Self::PinGetPowerDenied { pin } => {
                uwrite!(f, "pins.{}.error.getPowerDenied()", pin)
            },
            Self::PinSetPowerDenied { pin } => {
                uwrite!(f, "pins.{}.error.setPowerDenied()", pin)
            },
            Self::Config { collection } => {
                uwrite!(f, "config = {}", collection)
            },
        }
    }
}