#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read;
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
    let ctx = &mut InteractableCtx { adc: &mut adc };

    let mut buffer = heapless::Deque::new();
    outgoing::serialize(&outgoing::Response::Pong { }, &mut buffer);
    surround_frame(&mut buffer);

    for byte in buffer.iter() {
        serial.write_byte(*byte);
    }

    loop {
        if buffer.is_empty() {
            match serial.read() {
                Ok(byte) => {
                    buffer.push_back(byte).map_err(drop).unwrap();
                },
                Err(nb::Error::WouldBlock) => {
                    for pin in interactive.iter_mut() {
                        if let Some(power) = pin.poll_change(ctx) {
                            buffer.clear();
                            let response = outgoing::Response::PinChanged {
                                pin: pin.id().name().as_bytes(),
                                power,
                            };
                            outgoing::serialize(&response, &mut buffer);
                            surround_frame(&mut buffer);

                            for byte in buffer.iter() {
                                serial.write_byte(*byte);
                            }
                        }
                    }
                },
            }
        }

        let response = match read_frame(&mut serial, &mut buffer) {
            Err(FrameError::NotMagic) => {
                _ = buffer.pop_front();
                continue;
            },
            Err(FrameError::ChecksumMismatch) => {
                buffer.clear();
                outgoing::Response::PacketError {
                    message: b"Packet corrupted",
                }
            },
            Ok(payload) => {
                match incoming::deserialize(payload) {
                    Err(incoming::Error::UnexpectedEndOfPacket) => {
                        outgoing::Response::PacketError {
                            message: b"Unexpected end of packet",
                        }
                    },
                    Err(incoming::Error::UnexpectedExcessData) => {
                        outgoing::Response::PacketError {
                            message: b"Unexpected excess data",
                        }
                    },
                    Err(incoming::Error::InvalidPinMode) => {
                        outgoing::Response::PacketError {
                            message: b"Invalid pin mode",
                        }
                    },
                    Err(incoming::Error::InvalidPacketId) => {
                        outgoing::Response::PacketError {
                            message: b"Invalid packet id",
                        }
                    },
                    Ok(incoming::Request::Ping { }) => {
                        outgoing::Response::Pong { }
                    },
                    Ok(incoming::Request::GetConfig { }) => {
                        outgoing::Response::Config {
                            model: b"Arduino Uno",
                            pins: &interactive,
                        }
                    },
                    Ok(incoming::Request::GetPinPower { pin }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::DenyError {
                                message: b"Invalid pin id",
                            },
                            Ok(pin) => {
                                match interactive[pin].power(ctx) {
                                    Ok(power) => outgoing::Response::GetPinPowerResponse {
                                        pin: pin.name().as_bytes(),
                                        power,
                                    },
                                    Err(()) => outgoing::Response::DenyError {
                                        message: b"Denied get pin power",
                                    },
                                }
                            },
                        }
                    },
                    Ok(incoming::Request::GetPinMode { pin }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::DenyError {
                                message: b"Invalid pin id",
                            },
                            Ok(pin) => {
                                match interactive[pin].mode(ctx) {
                                    Ok(mode) => outgoing::Response::GetPinModeResponse {
                                        pin: pin.name().as_bytes(),
                                        mode,
                                    },
                                    Err(()) => outgoing::Response::DenyError {
                                        message: b"Denied get pin mode",
                                    },
                                }
                            },
                        }
                    },
                    Ok(incoming::Request::GetPinFlags { pin }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::DenyError {
                                message: b"Invalid pin id",
                            },
                            Ok(pin) => {
                                outgoing::Response::GetPinFlagsResponse {
                                    pin: pin.name().as_bytes(),
                                    flags: interactive[pin].flags(),
                                }
                            },
                        }
                    },
                    Ok(incoming::Request::SetPinPower { pin, power }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::DenyError {
                                message: b"Invalid pin id",
                            },
                            Ok(pin) => {
                                match interactive[pin].set_power(power, ctx) {
                                    Ok(power) => outgoing::Response::SetPinPowerResponse {
                                        pin: pin.name().as_bytes(),
                                        power,
                                    },
                                    Err(()) => outgoing::Response::DenyError {
                                        message: b"Denied set pin power",
                                    },
                                }
                            },
                        }
                    },
                    Ok(incoming::Request::SetPinMode { pin, mode }) => {
                        match PinId::try_from(pin) {
                            Err(()) => outgoing::Response::DenyError {
                                message: b"Invalid pin id",
                            },
                            Ok(pin) => {
                                match interactive[pin].set_mode(mode, ctx) {
                                    Ok(()) => outgoing::Response::SetPinModeResponse {
                                        pin: pin.name().as_bytes(),
                                        mode,
                                    },
                                    Err(()) => outgoing::Response::DenyError {
                                        message: b"Denied set pin mode",
                                    },
                                }
                            },
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
