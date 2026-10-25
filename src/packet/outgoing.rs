use crate::pin::{Collection, PinFlags, PinMode};

#[derive(Clone, Copy)]
pub enum Response<'a> {
    Empty(),
    Config { model: &'a [u8], pins: &'a Collection },
    Power(u16),
    Mode(PinMode),
    Flags(PinFlags),
    NACK { message: &'a [u8] },
    Panic { line: u32, column: u32, file: &'a [u8] },
}

pub fn serialize(response: &Response<'_>, buffer: &mut impl Extend<u8>) {
    match response {
        Response::Empty() => {
            buffer.extend([0]);
        },
        Response::Config { model: name, pins } => {
            buffer.extend([1]);
            buffer.extend([1]); // version
            buffer.extend([name.len() as u8]);
            buffer.extend(name.iter().copied());
            buffer.extend([pins.len() as u8]);
            for pin in pins.iter() {
                buffer.extend([pin.id().name().len() as u8]);
                buffer.extend(pin.id().name().bytes());
                buffer.extend([pin.flags().bits()]);
            }
        },
        Response::Power(power) => {
            buffer.extend([2]);
            buffer.extend(power.to_be_bytes());
        },
        Response::Mode(mode) => {
            buffer.extend([3]);
            buffer.extend([u8::from(*mode)]);
        },
        Response::Flags(flags) => {
            buffer.extend([4]);
            buffer.extend([flags.bits()]);
        },
        Response::NACK { message } => {
            buffer.extend([200]);
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
