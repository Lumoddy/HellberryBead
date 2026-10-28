use crate::pin::{Collection, PinFlags, PinMode};

#[derive(Clone, Copy)]
pub enum Response<'a> {
    Pong { },
    Config { model: &'a [u8], pins: &'a Collection },
    GetPinPowerResponse { pin: &'a [u8], power: u16 },
    GetPinModeResponse { pin: &'a [u8], mode: PinMode },
    GetPinFlagsResponse { pin: &'a [u8], flags: PinFlags },
    SetPinPowerResponse { pin: &'a [u8], power: u8 },
    SetPinModeResponse { pin: &'a [u8], mode: PinMode },
    PinChanged { pin: &'a [u8], power: u16 },
    PacketError { message: &'a [u8] },
    DenyError { message: &'a [u8] },
    Panic { line: u32, column: u32, file: &'a [u8] },
}

pub fn serialize(response: &Response<'_>, buffer: &mut impl Extend<u8>) {
    match response {
        Response::Pong { } => {
            buffer.extend([0]);
        },
        Response::Config { model, pins } => {
            buffer.extend([1]);
            buffer.extend([1]); // version
            buffer.extend([model.len() as u8]);
            buffer.extend(model.iter().copied());
            buffer.extend([pins.len() as u8]);
            for pin in pins.iter() {
                buffer.extend([pin.id().name().len() as u8]);
                buffer.extend(pin.id().name().bytes());
                buffer.extend([pin.flags().bits()]);
            }
        },
        Response::GetPinPowerResponse { pin, power } => {
            buffer.extend([2]);
            buffer.extend([pin.len() as u8]);
            buffer.extend(pin.iter().copied());
            buffer.extend(power.to_be_bytes());
        },
        Response::GetPinModeResponse { pin, mode } => {
            buffer.extend([3]);
            buffer.extend([pin.len() as u8]);
            buffer.extend(pin.iter().copied());
            buffer.extend([u8::from(*mode)]);
        },
        Response::GetPinFlagsResponse { pin, flags } => {
            buffer.extend([4]);
            buffer.extend([pin.len() as u8]);
            buffer.extend(pin.iter().copied());
            buffer.extend([flags.bits()]);
        },
        Response::SetPinPowerResponse { pin, power } => {
            buffer.extend([5]);
            buffer.extend([pin.len() as u8]);
            buffer.extend(pin.iter().copied());
            buffer.extend(power.to_be_bytes());
        },
        Response::SetPinModeResponse { pin, mode } => {
            buffer.extend([6]);
            buffer.extend([pin.len() as u8]);
            buffer.extend(pin.iter().copied());
            buffer.extend([u8::from(*mode)]);
        },
        Response::PinChanged { pin, power } => {
            buffer.extend([7]);
            buffer.extend([pin.len() as u8]);
            buffer.extend(pin.iter().copied());
            buffer.extend(power.to_be_bytes());
        },
        Response::PacketError { message } => {
            buffer.extend([201]);
            buffer.extend([message.len() as u8]);
            buffer.extend(message.iter().copied());
        },
        Response::DenyError { message } => {
            buffer.extend([202]);
            buffer.extend([message.len() as u8]);
            buffer.extend(message.iter().copied());
        },
        Response::Panic { line, column, file } => {
            buffer.extend([255]);
            buffer.extend((*line as u64).to_be_bytes());
            buffer.extend((*column as u64).to_be_bytes());
            buffer.extend([file.len() as u8]);
            buffer.extend(file.iter().copied());
        },
    }
}
