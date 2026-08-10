use crate::pin;
use crate::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Packet<'a>
{
    Pong(Pong),
    Config(Config<'a>),
    GetPinPowerResponse(GetPinPowerResponse),
    GetPinModeResponse(GetPinModeResponse),
    SetPinPowerResponse(SetPinPowerResponse),
    SetPinModeResponse(SetPinModeResponse),
    ListenPinPower(ListenPinPower),
    Panic(Panic),
    InvalidPacketId(InvalidPacketId),
    InvalidPinId(InvalidPinId),
    InvalidPinMode(InvalidPinMode),
    InvalidEscape(InvalidEscape),
    InvalidMissingStart(InvalidMissingStart),
    InvalidSetInputPinPower(InvalidSetInputPinPower),
    InvalidUnsupportedPinMode(InvalidUnsupportedPinMode),
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
        macro_rules! value
        {
            ( $tag:expr , $x:expr $(,)? ) =>
            {
                match ser.enum_tag($tag)
                {
                    Result::Ok(()) => ser.value($x),
                    Err(x) => Err(x),
                }
            };
        }

        match self
        {
            Packet::Pong(x) => value!(0, x),
            Packet::Config(x) => value!(1, x),
            Packet::GetPinPowerResponse(x) => value!(2, x),
            Packet::GetPinModeResponse(x) => value!(3, x),
            Packet::SetPinPowerResponse(x) => value!(4, x),
            Packet::SetPinModeResponse(x) => value!(5, x),
            Packet::ListenPinPower(x) => value!(6, x),
            Packet::Panic(x) => value!(100, x),
            Packet::InvalidPacketId(x) => value!(101, x),
            Packet::InvalidPinId(x) => value!(102, x),
            Packet::InvalidPinMode(x) => value!(103, x),
            Packet::InvalidEscape(x) => value!(104, x),
            Packet::InvalidMissingStart(x) => value!(105, x),
            Packet::InvalidSetInputPinPower(x) => value!(106, x),
            Packet::InvalidUnsupportedPinMode(x) => value!(107, x),
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
    Pong,
    SetPinPowerResponse,
    SetPinModeResponse,
    Panic,
    InvalidPacketId,
    InvalidPinId,
    InvalidPinPower,
    InvalidPinMode,
    InvalidEscape,
    InvalidMissingStart,
    InvalidSetInputPinPower,
    InvalidUnsupportedPinMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ConfigPinFlags(pub u8);

impl ConfigPinFlags
{
    pub const fn new() -> Self { Self(0) }

    pub const fn can_digital_read(&self) -> bool { self.0 & 0x1 != 0 }

    pub const fn set_can_digital_input(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x1 }
        else { self.0 = self.0 & !0x1 }
        self
    }

    pub const fn can_digital_write(&self) -> bool { self.0 & 0x2 != 0 }

    pub const fn set_can_digital_output(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x2 }
        else { self.0 = self.0 & !0x2 }
        self
    }

    pub const fn can_analog_read(&self) -> bool { self.0 & 0x4 != 0 }

    pub const fn set_can_analog_input(&mut self, flag: bool) -> &mut Self
    {
        if flag { self.0 = self.0 | 0x4 }
        else { self.0 = self.0 & !0x4 }
        self
    }

    pub const fn can_analog_write(&self) -> bool { self.0 & 0x8 != 0 }

    pub const fn set_can_analog_output(&mut self, flag: bool) -> &mut Self
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
pub struct ConfigPin<'a> { pub name: &'a str, pub flags: ConfigPinFlags }

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
pub struct Config<'a> { pub name: &'a str, pub pins: &'a [ConfigPin<'a>] }

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
pub struct GetPinPowerResponse { pub power: u16 }

impl Serialize for GetPinPowerResponse
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &GetPinPowerResponse
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.power) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GetPinModeResponse { pub mode: pin::PinMode }

impl Serialize for GetPinModeResponse
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &GetPinModeResponse
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.mode) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ListenPinPower { pub power: u16 }

impl Serialize for ListenPinPower
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &ListenPinPower
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.power) }
}