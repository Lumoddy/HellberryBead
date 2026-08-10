#![no_std]
#![no_main]

use core::convert::Infallible;

use arduino_hal::prelude::*;
use de::{DeserializeError, Deserializer};
use packet::{incoming, outgoing};
use pin::{DiDoAiInterPin, DiDoAoInterPin, DiDoInterPin, InterPin, InterPin2, inter_from_config};
use ser::{Serializer};
use setup::{BOARD_NAME, with_pins, with_pwm_timers};

mod panic_handler;
mod setup;
mod ser;
mod de;
mod endian;
mod pin;
mod packet;

const CONTROL: u8 = 16;
const START: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EscapedReaderError<T>
{
    EncounteredNewBeginning,
    Other(T),
}

impl DeserializeError for outgoing::Packet<'_>
{
    fn invalid_packet_id() -> Self { Self::InvalidPacketId(outgoing::InvalidPacketId) }

    fn invalid_pin_mode() -> Self { Self::InvalidPinMode(outgoing::InvalidPinMode) }
}

impl<T: DeserializeError> DeserializeError for EscapedReaderError<T>
{
    fn invalid_packet_id() -> Self { Self::Other(T::invalid_packet_id()) }

    fn invalid_pin_mode() -> Self { Self::Other(T::invalid_pin_mode()) }
}

struct EscapedReader<F: FnMut() -> nb::Result<u8, Infallible>>(F);

impl<F> Deserializer for EscapedReader<F>
where
    F: FnMut() -> nb::Result<u8, Infallible>,
{
    type Error = EscapedReaderError<outgoing::Packet<'static>>;

    fn u8(&mut self) -> Result<u8, Self::Error>
    {
        match nb::block!(self.0())
        {
            Ok(CONTROL) => match nb::block!(self.0())
            {
                Ok(CONTROL) => Ok(CONTROL),
                Ok(START) => Err(EscapedReaderError::EncounteredNewBeginning),
                _ => Err(EscapedReaderError::Other(outgoing::Packet::InvalidEscape(outgoing::InvalidEscape))),
            },
            Ok(x) => Ok(x),
        }
    }

    fn u16(&mut self) -> Result<u16, Self::Error>
    {
        Ok(u16::from_be_bytes([self.u8()?, self.u8()?]))
    }
}

struct EscapedWriter<F: FnMut(u8) -> nb::Result<(), Infallible>>(F);

impl<F> EscapedWriter<F>
where
    F: FnMut(u8) -> nb::Result<(), Infallible>,
{
    fn start(&mut self) -> Result<(), Infallible>
    {
        nb::block!(self.0(CONTROL))?;
        nb::block!(self.0(START))
    }
}

impl<F> Serializer for EscapedWriter<F>
where
    F: FnMut(u8) -> nb::Result<(), Infallible>,
{
    type Error = Infallible;

    fn u8(&mut self, value: u8) -> Result<(), Self::Error>
    {
        match value
        {
            CONTROL =>
            {
                nb::block!(self.0(CONTROL))?;
                nb::block!(self.0(CONTROL))
            },
            x => nb::block!(self.0(x)),
        }
    }

    fn u16(&mut self, value: u16) -> Result<(), Self::Error>
    {
        let bytes = value.to_be_bytes();
        self.u8(bytes[0])?;
        self.u8(bytes[1])?;
        Ok(())
    }
}

#[arduino_hal::entry]
fn main() -> !
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, arduino_hal::adc::AdcSettings::default());

    macro_rules! init_pwn_timers
    {
        ( $( $index:tt , )* ) =>
        {
            paste::paste!
            {
                ($(
                    arduino_hal::simple_pwm:: [<Timer $index Pwm>]
                        ::new(dp. [<TC $index >], arduino_hal::simple_pwm::Prescaler::Prescale64),
                )*)
            }
        };
    }

    let pwm_timers = with_pwm_timers!(init_pwn_timers);

    let (mut reader, mut writer) = serial.split();

    let mut reader = EscapedReader(|| reader.read());
    let mut writer = EscapedWriter(|x| writer.write(x));

    macro_rules! program_from_pins
    {
        (
            $(
                $name:ident : $port:ident =
                {
                    display: $display_name:literal ,
                    digital_in: $digital_in:literal ,
                    digital_out: $digital_out:literal ,
                    analog_in: $analog_in:literal ,
                    analog_out: $analog_out:literal ,
                    $(
                        analog_timer: $analog_timer_index:literal ,
                        $( $analog_timer_marker:lifetime )?
                    )?
                } ,
            )*
        )
        =>
        {
            {
                paste::paste!
                {
                    $(
                        let mut [<inter_ $name >] =
                        {
                            inter_from_config!
                            {
                                {
                                    digital_in: $digital_in ,
                                    digital_out: $digital_out ,
                                    analog_in: $analog_in ,
                                    analog_out: $analog_out ,
                                    $(analog_timer: $analog_timer_index , )?
                                }
                                [::DigitalInput { pin: pins. $name .into_pull_up_input() }]
                            }
                        };
                    )*

                    loop
                    {
                        $(
                            if let Some(power) = [<inter_ $name >] .poll_listen()
                            {
                                let Ok(()) = writer.start();
                                let Ok(()) = writer.value(
                                    outgoing::Packet::ListenPinPower(outgoing::ListenPinPower { power }));
                            }
                        )*

                        match (reader.0)()
                        {
                            Ok(CONTROL) if nb::block!((reader.0)()) == Ok(START) => 'packet: loop
                            {
                                match reader.value()
                                {
                                    Ok(incoming::Packet::Ping(incoming::Ping)) =>
                                    {
                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(
                                            outgoing::Packet::Pong(outgoing::Pong));
                                    },
                                    Ok(incoming::Packet::WholeConfig(incoming::WholeConfig)) =>
                                    {
                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(
                                            outgoing::Packet::Config(outgoing::Config
                                            {
                                                name: BOARD_NAME,
                                                pins:
                                                &[
                                                    $(
                                                        outgoing::ConfigPin
                                                        {
                                                            name: $display_name,
                                                            flags: *outgoing::ConfigPinFlags::new()
                                                                .set_can_digital_input($digital_in)
                                                                .set_can_digital_output($digital_out)
                                                                .set_can_analog_input($analog_in)
                                                                .set_can_analog_output($analog_out),
                                                        },
                                                    )*
                                                ],
                                            }));
                                    },
                                    Ok(incoming::Packet::GetPinPower(incoming::GetPinPower { pin })) =>
                                    {
                                        let power = 'power:
                                        {
                                            let i = 0;

                                            $(
                                                if pin.0 == i
                                                {
                                                    break 'power [<inter_ $name >] .get_power(&mut adc);
                                                }

                                                #[allow(unused)]
                                                let i = i + 1;
                                            )*

                                            let Ok(()) = writer.start();
                                            let Ok(()) = writer.value(
                                                outgoing::Packet::InvalidPinId(outgoing::InvalidPinId));

                                            break 'packet;
                                        };

                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(
                                            outgoing::Packet::GetPinPowerResponse(outgoing::GetPinPowerResponse { power }));
                                    },
                                    Ok(incoming::Packet::GetPinMode(incoming::GetPinMode { pin })) =>
                                    {
                                        let mode = 'mode:
                                        {
                                            let i = 0;

                                            $(
                                                if pin.0 == i
                                                {
                                                    break 'mode [<inter_ $name >] .get_mode();
                                                }

                                                #[allow(unused)]
                                                let i = i + 1;
                                            )*

                                            let Ok(()) = writer.start();
                                            let Ok(()) = writer.value(
                                                outgoing::Packet::InvalidPinId(outgoing::InvalidPinId));

                                            break 'packet;
                                        };

                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(
                                            outgoing::Packet::GetPinModeResponse(outgoing::GetPinModeResponse { mode }));
                                    },
                                    Ok(incoming::Packet::SetPinPower(incoming::SetPinPower { pin, power })) =>
                                    {
                                        let response = 'power:
                                        {
                                            let i = 0;

                                            $(
                                                if pin.0 == i
                                                {
                                                    break 'power [<inter_ $name >] .set_power(power);
                                                }

                                                #[allow(unused)]
                                                let i = i + 1;
                                            )*

                                            let Ok(()) = writer.start();
                                            let Ok(()) = writer.value(
                                                outgoing::Packet::InvalidPinId(outgoing::InvalidPinId));

                                            break 'packet;
                                        }
                                            .err().unwrap_or(outgoing::Packet::SetPinPowerResponse(outgoing::SetPinPowerResponse));

                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(response);
                                    },
                                    Ok(incoming::Packet::SetPinMode(incoming::SetPinMode { pin, mode })) =>
                                    {
                                        let response = 'mode:
                                        {
                                            let i = 0;

                                            $(
                                                if pin.0 == i
                                                {
                                                    #[cfg(all(true $( $( $analog_timer_marker:lifetime )? , false )?))]
                                                    break 'mode [<inter_ $name >]
                                                        .set_mode(mode, &(), &mut adc);
                                                    $(
                                                        break 'mode [<inter_ $name >]
                                                            .set_mode(mode, &pwm_timers. $analog_timer_index, &mut adc);
                                                    )?
                                                }

                                                #[allow(unused)]
                                                let i = i + 1;
                                            )*

                                            let Ok(()) = writer.start();
                                            let Ok(()) = writer.value(
                                                outgoing::Packet::InvalidPinId(outgoing::InvalidPinId));

                                            break 'packet;
                                        }
                                            .err().unwrap_or(outgoing::Packet::SetPinModeResponse(outgoing::SetPinModeResponse));

                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(response);
                                    },
                                    Err(EscapedReaderError::EncounteredNewBeginning) => continue,
                                    Err(EscapedReaderError::Other(packet)) =>
                                    {
                                        let Ok(()) = writer.start();
                                        let Ok(()) = writer.value(packet);
                                    },
                                }

                                break 'packet;
                            },
                            Ok(_) =>
                            {
                                let Ok(()) = writer.start();
                                let Ok(()) = writer.value(
                                    outgoing::Packet::InvalidMissingStart(outgoing::InvalidMissingStart));
                            },
                            Err(nb::Error::WouldBlock) => continue,
                        }
                    }
                }
            }
        };
    }

    with_pins!(program_from_pins);
}