use core::{mem, ptr};

use arduino_hal::hal::Atmega;
use arduino_hal::port::{Pin, mode};
use arduino_hal::simple_pwm::IntoPwmPin;
use embedded_hal::digital::{self, OutputPin, PinState};

use crate::de::{Deserialize, Deserializer, DeserializeError};
use crate::packet::outgoing;
use crate::ser::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinId(pub u8);

impl From<u8> for PinId
{
    fn from(value: u8) -> Self { Self(value) }
}

impl From<PinId> for u8
{
    fn from(value: PinId) -> Self { value.0 }
}

impl Serialize for PinId
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &PinId
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(self.0) }
}

impl Deserialize for PinId
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized { Ok(Self(de.u8()?)) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PinMode
{
    DigitalInput,
    DigitalOutput,
    DigitalListening,
    AnalogInput,
    AnalogOutput,
}

impl Serialize for PinMode
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &PinMode
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized
    {
        ser.enum_tag(match self
        {
            PinMode::DigitalInput => 1,
            PinMode::DigitalOutput => 2,
            PinMode::DigitalListening => 3,
            PinMode::AnalogInput => 4,
            PinMode::AnalogOutput => 5,
        })
    }
}

impl Deserialize for PinMode
{
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer + ?Sized
    {
        match de.enum_tag()?
        {
            1 => Ok(PinMode::DigitalInput),
            2 => Ok(PinMode::DigitalOutput),
            3 => Ok(PinMode::DigitalListening),
            4 => Ok(PinMode::AnalogInput),
            5 => Ok(PinMode::AnalogOutput),
            _ => Err(D::Error::invalid_pin_mode())
        }
    }
}

pub trait InterPin
{
    fn get_power(&mut self, adc: &mut arduino_hal::adc::Adc) -> u16;

    fn set_power(&mut self, power_or_duty: u8) -> Result<(), outgoing::Packet>;

    fn get_mode(&self) -> PinMode;

    fn poll_listen(&mut self) -> Option<u16>;
}

pub trait InterPin2<TC>: InterPin
{
    fn set_mode(&mut self, mode: PinMode, timer: &TC, adc: &mut arduino_hal::adc::Adc) -> Result<(), outgoing::Packet>;
}

const SET_INPUT_PIN_POWER: outgoing::Packet<'_> = outgoing::Packet::InvalidSetInputPinPower(outgoing::InvalidSetInputPinPower);
const UNSUPPORTED_PIN_MODE: outgoing::Packet<'_> = outgoing::Packet::InvalidUnsupportedPinMode(outgoing::InvalidUnsupportedPinMode);

pub enum DiDoInterPin<PIN>
where
    PIN: arduino_hal::port::PinOps,
{
    DigitalInput { pin: Pin<mode::Input<mode::PullUp>, PIN> },
    DigitalListening { pin: Pin<mode::Input<mode::PullUp>, PIN>, last_state: bool },
    DigitalOutput { pin: Pin<mode::Output, PIN> },
}

impl<PIN> InterPin for DiDoInterPin<PIN>
where
    PIN: arduino_hal::port::PinOps,
{
    fn get_power(&mut self, _: &mut arduino_hal::adc::Adc) -> u16
    {
        match self
        {
            Self::DigitalInput { pin } => pin.is_high() as u16,
            Self::DigitalListening { pin, last_state } =>
            {
                *last_state = pin.is_high();
                *last_state as u16
            },
            Self::DigitalOutput { pin } => pin.is_set_high() as u16,
        }
    }

    fn set_power(&mut self, power_or_duty: u8) -> Result<(), outgoing::Packet>
    {
        match self
        {
            Self::DigitalOutput { pin } => pin.set_state(PinState::from(power_or_duty > 0)).map_err(|x| match x { }),
            _ => Err(SET_INPUT_PIN_POWER),
        }
    }

    fn get_mode(&self) -> PinMode
    {
        match self
        {
            Self::DigitalInput { .. } => PinMode::DigitalInput,
            Self::DigitalListening { .. } => PinMode::DigitalListening,
            Self::DigitalOutput { .. } => PinMode::DigitalOutput,
        }
    }

    fn poll_listen(&mut self) -> Option<u16>
    {
        match self
        {
            Self::DigitalListening { pin, last_state } =>
            {
                let new_state = pin.is_high();
                if new_state == *last_state { return None };
                *last_state = new_state;
                Some(new_state as u16)
            },
            _ => None,
        }
    }
}

impl<TC, PIN> InterPin2<TC> for DiDoInterPin<PIN>
where
    PIN: arduino_hal::port::PinOps,
{
    fn set_mode(&mut self, mode: PinMode, _: &TC, _: &mut arduino_hal::adc::Adc) -> Result<(), outgoing::Packet>
    {
        unsafe
        {
            Ok(ptr::write(
                self,
                match ptr::read(self)
                {
                    Self::DigitalInput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin, last_state: false },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::DigitalListening { pin, last_state } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin, last_state },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::DigitalOutput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin: pin.into_pull_up_input() },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin },
                        PinMode::DigitalListening => Self::DigitalListening { pin: pin.into_pull_up_input(), last_state: false },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                }))
        }
    }
}

pub enum DiDoAiInterPin<PIN>
where
    PIN: arduino_hal::port::PinOps,
    Pin<mode::Analog, PIN>: arduino_hal::adc::AdcChannel<Atmega, arduino_hal::pac::ADC>,
{
    DigitalInput { pin: Pin<mode::Input<mode::PullUp>, PIN> },
    DigitalListening { pin: Pin<mode::Input<mode::PullUp>, PIN>, last_state: bool },
    DigitalOutput { pin: Pin<mode::Output, PIN> },
    AnalogInput { pin: Pin<mode::Analog, PIN> },
}

impl<PIN> InterPin for DiDoAiInterPin<PIN>
where
    PIN: arduino_hal::port::PinOps,
    Pin<mode::Analog, PIN>: arduino_hal::adc::AdcChannel<Atmega, arduino_hal::pac::ADC>,
{
    fn get_power(&mut self, adc: &mut arduino_hal::adc::Adc) -> u16
    {
        match self
        {
            Self::DigitalInput { pin } => pin.is_high() as u16,
            Self::DigitalListening { pin, last_state } =>
            {
                *last_state = pin.is_high();
                *last_state as u16
            },
            Self::DigitalOutput { pin } => pin.is_set_high() as u16,
            Self::AnalogInput { pin } => pin.analog_read(adc),
        }
    }

    fn set_power(&mut self, power_or_duty: u8) -> Result<(), outgoing::Packet>
    {
        match self
        {
            Self::DigitalOutput { pin } => pin.set_state(PinState::from(power_or_duty > 0)).map_err(|x| match x { }),
            _ => Err(SET_INPUT_PIN_POWER),
        }
    }

    fn get_mode(&self) -> PinMode
    {
        match self
        {
            Self::DigitalInput { .. } => PinMode::DigitalInput,
            Self::DigitalListening { .. } => PinMode::DigitalListening,
            Self::DigitalOutput { .. } => PinMode::DigitalOutput,
            Self::AnalogInput { .. } => PinMode::AnalogInput,
        }
    }

    fn poll_listen(&mut self) -> Option<u16>
    {
        match self
        {
            Self::DigitalListening { pin, last_state } =>
            {
                let new_state = pin.is_high();
                if new_state == *last_state { return None };
                *last_state = new_state;
                Some(new_state as u16)
            },
            _ => None,
        }
    }
}

impl<TC, PIN> InterPin2<TC> for DiDoAiInterPin<PIN>
where
    PIN: arduino_hal::port::PinOps,
    Pin<mode::Analog, PIN>: arduino_hal::adc::AdcChannel<Atmega, arduino_hal::pac::ADC>,
{
    fn set_mode(&mut self, mode: PinMode, _: &TC, adc: &mut arduino_hal::adc::Adc) -> Result<(), outgoing::Packet>
    {
        unsafe
        {
            Ok(ptr::write(
                self,
                match ptr::read(self)
                {
                    Self::DigitalInput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin, last_state: false },
                        PinMode::AnalogInput => Self::AnalogInput { pin: pin.into_analog_input(adc) },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::DigitalListening { pin, last_state } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin, last_state },
                        PinMode::AnalogInput => Self::AnalogInput { pin: pin.into_analog_input(adc) },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::DigitalOutput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin: pin.into_pull_up_input() },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin },
                        PinMode::DigitalListening => Self::DigitalListening { pin: pin.into_pull_up_input(), last_state: false },
                        PinMode::AnalogInput => Self::AnalogInput { pin: pin.into_analog_input(adc) },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::AnalogInput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin: pin.into_digital(adc).into_pull_up_input() },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_digital(adc).into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin: pin.into_digital(adc).into_pull_up_input(), last_state: false },
                        PinMode::AnalogInput => Self::AnalogInput { pin },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                }))
        }
    }
}

pub enum DiDoAoInterPin<TC, PIN>
where
    PIN: arduino_hal::port::PinOps + arduino_hal::simple_pwm::PwmPinOps<TC, Duty: Into<u16>>,
{
    DigitalInput { pin: Pin<mode::Input<mode::PullUp>, PIN> },
    DigitalListening { pin: Pin<mode::Input<mode::PullUp>, PIN>, last_state: bool },
    DigitalOutput { pin: Pin<mode::Output, PIN> },
    AnalogOutput { pin: Pin<mode::PwmOutput<TC>, PIN> },
}

impl<TC, PIN> InterPin for DiDoAoInterPin<TC, PIN>
where
    PIN: arduino_hal::port::PinOps + arduino_hal::simple_pwm::PwmPinOps<TC, Duty: Into<u16>>,
{
    fn get_power(&mut self, _: &mut arduino_hal::adc::Adc) -> u16
    {
        match self
        {
            Self::DigitalInput { pin } => pin.is_high() as u16,
            Self::DigitalListening { pin, last_state } =>
            {
                *last_state = pin.is_high();
                *last_state as u16
            },
            Self::DigitalOutput { pin } => pin.is_set_high() as u16,
            Self::AnalogOutput { pin } => pin.get_duty().into() * 4,
        }
    }

    fn set_power(&mut self, power_or_duty: u8) -> Result<(), outgoing::Packet>
    {
        match self
        {
            Self::DigitalOutput { pin } => pin.set_state(PinState::from(power_or_duty > 0)).map_err(|x| match x { }),
            Self::AnalogOutput { pin } => Ok(pin.set_duty(power_or_duty)),
            _ => Err(SET_INPUT_PIN_POWER),
        }
    }

    fn get_mode(&self) -> PinMode
    {
        match self
        {
            Self::DigitalInput { .. } => PinMode::DigitalInput,
            Self::DigitalListening { .. } => PinMode::DigitalListening,
            Self::DigitalOutput { .. } => PinMode::DigitalOutput,
            Self::AnalogOutput { .. } => PinMode::AnalogOutput,
        }
    }

    fn poll_listen(&mut self) -> Option<u16>
    {
        match self
        {
            Self::DigitalListening { pin, last_state } =>
            {
                let new_state = pin.is_high();
                if new_state == *last_state { return None };
                *last_state = new_state;
                Some(new_state as u16)
            },
            _ => None,
        }
    }
}

impl<TC, PIN> InterPin2<TC> for DiDoAoInterPin<TC, PIN>
where
    PIN: arduino_hal::port::PinOps + arduino_hal::simple_pwm::PwmPinOps<TC, Duty: Into<u16>>,
{
    fn set_mode(&mut self, mode: PinMode, timer: &TC, _: &mut arduino_hal::adc::Adc) -> Result<(), outgoing::Packet>
    {
        unsafe
        {
            Ok(ptr::write(
                self,
                match ptr::read(self)
                {
                    Self::DigitalInput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin, last_state: false },
                        PinMode::AnalogOutput => Self::AnalogOutput { pin: to_pwm(pin.into_output(), timer) },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::DigitalListening { pin, last_state } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: pin.into_output() },
                        PinMode::DigitalListening => Self::DigitalListening { pin, last_state },
                        PinMode::AnalogOutput => Self::AnalogOutput { pin: to_pwm(pin.into_output(), timer) },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::DigitalOutput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin: pin.into_pull_up_input() },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin },
                        PinMode::DigitalListening => Self::DigitalListening { pin: pin.into_pull_up_input(), last_state: false },
                        PinMode::AnalogOutput => Self::AnalogOutput { pin: to_pwm(pin, timer) },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                    Self::AnalogOutput { pin } => match mode
                    {
                        PinMode::DigitalInput => Self::DigitalInput { pin: to_digital(pin, timer).into_pull_up_input() },
                        PinMode::DigitalOutput => Self::DigitalOutput { pin: to_digital(pin, timer) },
                        PinMode::DigitalListening => Self::DigitalListening { pin: to_digital(pin, timer).into_pull_up_input(), last_state: false },
                        PinMode::AnalogOutput => Self::AnalogOutput { pin },
                        _ => return Err(UNSUPPORTED_PIN_MODE),
                    },
                }))
        }
    }
}

fn to_pwm<TC, PIN>(pin: Pin<mode::Output, PIN>, timer: &TC) -> Pin<mode::PwmOutput<TC>, PIN>
where
    PIN: arduino_hal::port::PinOps + arduino_hal::simple_pwm::PwmPinOps<TC, Duty: Into<u16>>,
{
    let mut pin = pin.into_pwm(timer);
    pin.enable();
    pin
}

/// For some reason [`arduino_hal`] does not provide a way to convert pwm pins
/// back into digital ones.
unsafe fn to_digital<TC, PIN>(mut pin: Pin<mode::PwmOutput<TC>, PIN>, _timer: &TC) -> Pin<mode::Output, PIN>
where
    PIN: arduino_hal::port::PinOps + arduino_hal::simple_pwm::PwmPinOps<TC, Duty: Into<u16>>,
{
    pin.disable();
    drop(pin);

    unsafe { mem::zeroed::<Pin<mode::Output, PIN>>() }.into_output()
}

macro_rules! inter_from_config
{
    (
        $( [ $( $before:tt )* ] )?
        {
            digital_in: true,
            digital_out: true,
            analog_in: false,
            analog_out: false,
        }
        $( [ $( $after:tt )* ] )?
    )
    => { $($( $before )*)? DiDoInterPin $($( $after )*)? };

    (
        $( [ $( $before:tt )* ] )?
        {
            digital_in: true,
            digital_out: true,
            analog_in: true,
            analog_out: false,
        }
        $( [ $( $after:tt )* ] )?
    )
    => { $($( $before )*)? DiDoAiInterPin $($( $after )*)? };

    (
        $( [ $( $before:tt )* ] )?
        {
            digital_in: true,
            digital_out: true,
            analog_in: false,
            analog_out: true,
            $(analog_timer: $analog_timer_index:literal , )?
        }
        $( [ $( $after:tt )* ] )?
    )
    => { $($( $before )*)? DiDoAoInterPin $($( $after )*)? };
}

pub(crate) use inter_from_config;