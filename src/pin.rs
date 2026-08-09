use embedded_hal::digital;

use crate::de::{Deserialize, Deserializer, DeserializeError};
use crate::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinId(pub u8);

impl From<u8> for PinId
{
    fn from(value: u8) -> Self { Self(value) }
}

impl From<PinId> for u8
{
    fn from(value: PinId) -> Self { value.0 }
}

impl Serialize for PinId
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &PinId
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.0) }
}

impl Deserialize for PinId
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized { Ok(Self(de.u8()?)) }
}

impl Serialize for digital::PinState
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &digital::PinState
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized
    {
        ser.enum_tag(match self
        {
            digital::PinState::Low => 0,
            digital::PinState::High => 1,
        })
    }
}

impl Deserialize for digital::PinState
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        match de.enum_tag()?
        {
            0 => Ok(digital::PinState::Low),
            1 => Ok(digital::PinState::High),
            _ => Err(D::Error::invalid_pin_state())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PinMode
{
    Ignore,
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    AnalogOutput,
}

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
        S: Serializer + ?Sized
    {
        ser.enum_tag(match self
        {
            PinMode::Ignore => 0,
            PinMode::DigitalInput => 1,
            PinMode::DigitalOutput => 2,
            PinMode::AnalogInput => 3,
            PinMode::AnalogOutput => 4,
        })
    }
}

impl Deserialize for PinMode
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        match de.enum_tag()?
        {
            0 => Ok(PinMode::Ignore),
            1 => Ok(PinMode::DigitalInput),
            2 => Ok(PinMode::DigitalOutput),
            3 => Ok(PinMode::AnalogInput),
            4 => Ok(PinMode::AnalogOutput),
            _ => Err(D::Error::invalid_pin_mode())
        }
    }
}

pub enum DrDwArAwInterPin
{
    
}