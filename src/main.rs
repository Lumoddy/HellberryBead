#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read as Read;
use packet::{Delimited, Deserialize, incoming, is_whitespace, outgoing};
use pin::InteractableCtx;
use ufmt::uwrite;

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

    uwrite!(serial, "{}", Delimited(outgoing::Packet::Ready));

    loop {
        match serial.read() {
            Ok(first) => {
                if is_whitespace(first) {
                    continue;
                }

                let mut peek = Some(first);
                let mut reader = CountingReader { inner: &mut serial, count: 0 };
                let response = match <_>::deserialize(&mut reader, &mut peek, true) {
                    Ok(Err(())) => {
                        outgoing::Packet::MalformedPacket { pos: reader.count }
                    },
                    Ok(Ok(Delimited(None))) => {
                        continue;
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::Ping)))) => {
                        outgoing::Packet::Pong
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::Config)))) => {
                        outgoing::Packet::Config { collection: &interactive }
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::GetPinPower { pin })))) => {
                        match interactive[pin].power(ctx) {
                            Ok(power) => {
                                outgoing::Packet::PinPowerResponse { pin, power }
                            },
                            Err(()) => {
                                outgoing::Packet::PinGetPowerDenied { pin }
                            },
                        }
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::GetPinMode { pin })))) => {
                        match interactive[pin].mode(ctx) {
                            Ok(mode) => {
                                outgoing::Packet::PinModeResponse { pin, mode }
                            },
                            Err(()) => {
                                outgoing::Packet::PinGetPowerDenied { pin }
                            },
                        }
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::PinFlags { pin })))) => {
                        let flags =  interactive[pin].flags();
                        outgoing::Packet::PinFlagsResponse { pin, flags }
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::SetPinPower { pin, power })))) => {
                        let interactive_pin = &mut interactive[pin];
                        match interactive_pin.set_power(power, ctx) {
                            Ok(power) => {
                                outgoing::Packet::PinPowerResponse { pin, power }
                            },
                            Err(()) => {
                                outgoing::Packet::PinSetPowerDenied { pin }
                            },
                        }
                    },
                    Ok(Ok(Delimited(Some(incoming::Packet::SetPinMode { pin, mode })))) => {
                        match interactive[pin].set_mode(mode, ctx) {
                            Ok(()) => {
                                outgoing::Packet::PinModeResponse { pin, mode }
                            },
                            Err(()) => {
                                outgoing::Packet::UnsupportedPinMode { pin, mode }
                            },
                        }
                    },
                };

                uwrite!(serial, "{}", Delimited(response));
            },
            Err(nb::Error::WouldBlock) => {
                for pin in &mut interactive {
                    if let Some(power) = pin.poll_change(ctx) {
                        let response = outgoing::Packet::PinPowerResponse { pin: pin.id(), power };
                        uwrite!(serial, "{}", Delimited(response));
                    }
                }
            },
        }
    }
}

pub struct CountingReader<'a, R> {
    pub inner: &'a mut R,
    pub count: usize,
}

impl<R: Read<T>, T> Read<T> for CountingReader<'_, R> {
    type Error = R::Error;

    fn read(&mut self) -> nb::Result<T, Self::Error> {
        let word = self.inner.read()?;
        self.count += 1;
        Ok(word)
    }
}