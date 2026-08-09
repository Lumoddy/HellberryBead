use embedded_hal::digital;

use crate::pin;
use crate::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Packet<'a>
{
    Ok(Ok),
    ConfigPin(ConfigPin<'a>),
    Config(Config<'a>),
    PinState(PinState),
    PinMode(PinMode),
    Panic(Panic),
    InvalidPacketId(InvalidPacketId),
    InvalidPinId(InvalidPinId),
    InvalidPinState(InvalidPinState),
    InvalidPinMode(InvalidPinMode),
}

impl Serialize for Packet<'_>
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &Packet<'_>
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
pub struct ConfigPinFlags(pub u8);

impl ConfigPinFlags
{
    pub const fn new() -> Self { Self(0) }

    pub const fn can_digital_read(&self) -> bool { self.0 & 0x1 != 0 }

    pub const fn set_can_digital_read(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x1 }
        else { self.0 = self.0 & !0x1 }
        self
    }

    pub const fn can_digital_write(&self) -> bool { self.0 & 0x2 != 0 }

    pub const fn set_can_digital_write(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x2 }
        else { self.0 = self.0 & !0x2 }
        self
    }

    pub const fn can_analog_read(&self) -> bool { self.0 & 0x4 != 0 }

    pub const fn set_can_analog_read(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x4 }
        else { self.0 = self.0 & !0x4 }
        self
    }

    pub const fn can_analog_write(&self) -> bool { self.0 & 0x8 != 0 }

    pub const fn set_can_analog_write(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x8 }
        else { self.0 = self.0 & !0x8 }
        self
    }
}

impl Serialize for ConfigPinFlags
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &ConfigPinFlags
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.0) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ConfigPin<'a> { name: &'a str, flags: ConfigPinFlags }

impl Serialize for ConfigPin<'_>
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &ConfigPin<'_>
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value((self.name, self.flags)) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Config<'a> { name: &'a str, pins: &'a [ConfigPin<'a>] }

impl Serialize for Config<'_>
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &Config<'_>
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value((self.name, self.pins)) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinState { state: digital::PinState }

impl Serialize for PinState
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &PinState
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