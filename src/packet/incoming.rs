use embedded_hal::digital;

use crate::de::{Deserialize, Deserializer};
use crate::pin::{self, PinId};
use crate::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Packet
{
    Ping(Ping),
}

impl Serialize for Packet
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &Packet
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized
    {
        match self
        {
            Packet::Ok(x) =>
            {
                ser.enum_tag(0)?;
                ser.value(x)
            },
            Packet::ConfigPin(x) =>
            {
                ser.enum_tag(1)?;
                ser.value(x)
            },
            Packet::Config(x) =>
            {
                ser.enum_tag(2)?;
                ser.value(x)
            },
            Packet::PinState(x) =>
            {
                ser.enum_tag(3)?;
                ser.value(x)
            },
            Packet::PinMode(x) =>
            {
                ser.enum_tag(4)?;
                ser.value(x)
            },
            Packet::Panic(x) =>
            {
                ser.enum_tag(100)?;
                ser.value(x)
            },
            Packet::InvalidPacketId(x) =>
            {
                ser.enum_tag(101)?;
                ser.value(x)
            },
            Packet::InvalidPinId(x) =>
            {
                ser.enum_tag(102)?;
                ser.value(x)
            },
            Packet::InvalidPinState(x) =>
            {
                ser.enum_tag(103)?;
                ser.value(x)
            },
            Packet::InvalidPinMode(x) =>
            {
                ser.enum_tag(104)?;
                ser.value(x)
            },
        }
    }
}

macro_rules! unit_impl
{
    ( $( $name:ident $(,)? )* ) =>
    {
        $(
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct $name;

            impl Serialize for $name
            {
                fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
                where
                    S: Serializer + ?Sized { ser.value(&self) }
            }

            impl Serialize for &$name
            {
                fn serialize<S>(self, _: &mut S) -> Result<(), S::Error>
                where
                    S: Serializer + ?Sized { Result::Ok(()) }
            }
        )*
    };
}

unit_impl!
{
    Ok,
    Panic,
    InvalidPacketId,
    InvalidPinId,
    InvalidPinState,
    InvalidPinMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GetPinState { pin: PinId }

impl Deserialize for GetPinState
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        Self { pin: de.u8() }
    }
}

impl Serialize for &GetPinState
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.state) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinMode { mode: pin::PinMode }

impl Serialize for PinMode
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &PinMode
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.mode) }
}