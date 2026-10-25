use crate::pin::PinMode;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Request<'a> {
    Ping { },
    GetConfig { },
    GetPinPower { pin: &'a [u8] },
    GetPinMode { pin: &'a [u8] },
    GetPinFlags { pin: &'a [u8] },
    SetPinPower { pin: &'a [u8], power: u8 },
    SetPinMode { pin: &'a [u8], mode: PinMode },
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Error {
    UnexpectedEndOfPacket,
    UnexpectedExcessData,
    InvalidPinMode,
    InvalidPacketId,
}

pub fn deserialize(payload: &[u8]) -> Result<Request<'_>, Error> {
    match payload {
        [0, rest @ ..] => {
            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            Ok(Request::Ping { })
        },
        [1, rest @ ..] => {
            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            Ok(Request::GetConfig { })
        },
        [2, rest @ ..] => {
            let [len, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let Some((pin, rest)) = rest.split_at_checked(*len as usize) else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            Ok(Request::GetPinPower { pin })
        },
        [3, rest @ ..] => {
            let [len, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let Some((pin, rest)) = rest.split_at_checked(*len as usize) else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            Ok(Request::GetPinMode { pin })
        },
        [4, rest @ ..] => {
            let [len, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let Some((pin, rest)) = rest.split_at_checked(*len as usize) else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            Ok(Request::GetPinFlags { pin })
        },
        [5, rest @ ..] => {
            let [len, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let Some((pin, rest)) = rest.split_at_checked(*len as usize) else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let [power, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            Ok(Request::SetPinPower { pin, power: *power })
        },
        [6, rest @ ..] => {
            let [len, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let Some((pin, rest)) = rest.split_at_checked(*len as usize) else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            let [mode, rest @ ..] = rest else {
                return Err(Error::UnexpectedEndOfPacket);
            };

            if !rest.is_empty() {
                return Err(Error::UnexpectedExcessData);
            };

            let mode = PinMode::try_from(*mode).map_err(|_| Error::InvalidPinMode)?;

            Ok(Request::SetPinMode { pin, mode })
        },
        [_, ..] => Err(Error::InvalidPacketId),
        [] => Err(Error::UnexpectedEndOfPacket),
    }
}
