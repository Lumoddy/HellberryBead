use crate::de::{Deserialize, Deserializer, DeserializeError};
use crate::pin::{PinId, PinMode};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Packet
{
    Ping(Ping),
    WholeConfig(WholeConfig),
    GetPinPower(GetPinPower),
    GetPinMode(GetPinMode),
    SetPinPower(SetPinPower),
    SetPinMode(SetPinMode),
}

impl Deserialize for Packet
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        match de.enum_tag()?
        {
            0 => Ok(Self::Ping(de.value()?)),
            1 => Ok(Self::WholeConfig(de.value()?)),
            2 => Ok(Self::GetPinPower(de.value()?)),
            3 => Ok(Self::GetPinMode(de.value()?)),
            4 => Ok(Self::SetPinPower(de.value()?)),
            5 => Ok(Self::SetPinMode(de.value()?)),
            _ => Err(D::Error::invalid_packet_id()),
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

            impl Deserialize for $name
            {
                fn deserialize<D>(_: &mut D) -> Result<Self, D::Error>
                where
                    D: Deserializer + ?Sized { Ok(Self) }
            }
        )*
    };
}

unit_impl!
{
    Ping,
    WholeConfig,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GetPinPower { pub pin: PinId }

impl Deserialize for GetPinPower
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        Ok(Self { pin: de.value()? })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GetPinMode { pub pin: PinId }

impl Deserialize for GetPinMode
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        Ok(Self { pin: de.value()? })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SetPinPower { pub pin: PinId, pub power: u8 }

impl Deserialize for SetPinPower
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        Ok(Self { pin: de.value()?, power: de.value()? })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SetPinMode { pub pin: PinId, pub mode: PinMode }

impl Deserialize for SetPinMode
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        Ok(Self { pin: de.value()?, mode: de.value()? })
    }
}