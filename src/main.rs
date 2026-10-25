#![no_std]
#![no_main]

use packet::{FrameError, incoming, outgoing, read_frame, surround_frame};
use pin::{InteractableCtx, PinId};

mod panic;
mod packet;
mod pin;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, <_>::default());
    let mut interactive = pin::collect_pins!(pins);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut ctx = &mut InteractableCtx { adc: &mut adc };

    let mut buffer = heapless::Deque::new();

    loop {
        let response = match read_frame(&mut serial, &mut buffer) {
            Err(FrameError::NotMagic) => {
                _ = buffer.pop_front();
                continue;
            },
            Err(FrameError::ChecksumMismatch) => {
                buffer.clear();
                outgoing::Response::NACK { message: b"Packet corrupted" }
            },
            Ok(payload) => {
                match incoming::deserialize(payload) {
                    Err(incoming::Error::UnexpectedEndOfPacket) => {
                        outgoing::Response::NACK { message: b"Unexpected end of packet" }
                    },
                    Err(incoming::Error::UnexpectedExcessData) => {
                        outgoing::Response::NACK { message: b"Unexpected excess data" }
                    },
                    Err(incoming::Error::InvalidPinMode) => {
                        outgoing::Response::NACK { message: b"Invalid pin mode" }
                    },
                    Err(incoming::Error::InvalidPacketId) => {
                        outgoing::Response::NACK { message: b"Invalid packet id" }
                    },
                    Ok(incoming::Request::Ping { }) => {
                        outgoing::Response::Empty()
                    },
                    Ok(incoming::Request::GetConfig { }) => {
                        outgoing::Response::Config { model: b"Arduino Uno", pins: &interactive }
                    },
                    Ok(incoming::Request::GetPinPower { pin }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::NACK { message: b"Invalid pin id" },
                            Ok(pin) => {
                                match interactive[pin].power(&mut ctx) {
                                    Ok(power) => outgoing::Response::Power(power),
                                    Err(()) => outgoing::Response::NACK { message: b"Denied get pin power" },
                                }
                            }
                        }
                    },
                    Ok(incoming::Request::GetPinMode { pin }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::NACK { message: b"Invalid pin id" },
                            Ok(pin) => {
                                match interactive[pin].mode(&mut ctx) {
                                    Ok(mode) => outgoing::Response::Mode(mode),
                                    Err(()) => outgoing::Response::NACK { message: b"Denied get pin power" },
                                }
                            }
                        }
                    },
                    Ok(incoming::Request::GetPinFlags { pin }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::NACK { message: b"Invalid pin id" },
                            Ok(pin) => {
                                outgoing::Response::Flags(interactive[pin].flags())
                            }
                        }
                    },
                    Ok(incoming::Request::SetPinPower { pin, power }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::NACK { message: b"Invalid pin id" },
                            Ok(pin) => {
                                match interactive[pin].set_power(power, &mut ctx) {
                                    Ok(power) => outgoing::Response::Power(power),
                                    Err(()) => outgoing::Response::NACK { message: b"Denied set pin power" },
                                }
                            }
                        }
                    },
                    Ok(incoming::Request::SetPinMode { pin, mode }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::NACK { message: b"Invalid pin id" },
                            Ok(pin) => {
                                match interactive[pin].set_mode(mode, &mut ctx) {
                                    Ok(()) => outgoing::Response::Mode(mode),
                                    Err(()) => outgoing::Response::NACK { message: b"Denied set pin mode" },
                                }
                            }
                        }
                    },
                }
            },
        };

        buffer.clear();
        outgoing::serialize(&response, &mut buffer);
        surround_frame(&mut buffer);

        for byte in buffer.iter() {
            serial.write_byte(*byte);
        }
    }
}
