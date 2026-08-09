
pub trait DeserializeError
{
    fn invalid_packet_id() -> Self;

    fn invalid_pin_state() -> Self;

    fn invalid_pin_mode() -> Self;
}

pub trait Deserializer
{
    type Error: DeserializeError;

    fn u8(&mut self) -> Result<u8, Self::Error>;

    fn u16(&mut self) -> Result<u16, Self::Error>;

    fn len(&mut self) -> Result<u16, Self::Error> { self.u16() }

    fn enum_tag(&mut self) -> Result<u8, Self::Error> { self.u8() }

    fn value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize { T::deserialize(self) }
}

pub trait Deserialize: Sized
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized;
}

impl Deserialize for u8
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized { de.u8() }
}

impl Deserialize for u16
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized { de.u16() }
}