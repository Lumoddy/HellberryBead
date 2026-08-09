#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(abi_avr_interrupt)]

use core::convert::Infallible;

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::IntoPwmPin;
use de::{DeserializeError, Deserializer};
use packet::outgoing;
use pin::{DiDoAiInterPin, DiDoAoInterPin, DiDoInterPin, inter_from_config};
use ser::Serializer;
use setup::{with_pins, with_pwm_timers};

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
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
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

    let mut pwm_timers = with_pwm_timers!(init_pwn_timers);

    let (mut read, mut write) = serial.split();

    let mut read = EscapedReader(|| read.read());
    let mut write = EscapedWriter(|x| write.write(x));

    macro_rules! program_from_pins
    {
        (
            $(
                $name:ident : $port:ident =
                {
                    digital_in: $digital_in:literal ,
                    digital_out: $digital_out:literal ,
                    analog_in: $analog_in:literal ,
                    analog_out: $analog_out:literal ,
                    $(analog_timer: $analog_timer_index:literal , )?
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

                    
                }
            }
        };
    }

    with_pins!(program_from_pins);

    todo!()
}