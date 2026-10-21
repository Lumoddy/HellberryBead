use core::marker::PhantomData;
use core::ptr;

use arduino_hal::hal::port;
use arduino_hal::{adc, hal};
use arduino_hal::port::Pin;
use arduino_hal::port::mode::{Analog, Floating, Input, Output, PullUp};

mod id;
pub use id::*;

mod mode;
pub use mode::*;

mod flags;
pub use flags::*;

mod collection;
pub use collection::*;

pub struct InteractableCtx<'a, ADC> {
    pub adc: &'a mut ADC,
}

pub trait Interactable<ADC> {
    fn id(&self) -> PinId;

    fn flags(&self) -> PinFlags;

    fn power(&mut self, ctx: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()>;

    fn poll_change(&mut self, ctx: &mut InteractableCtx<'_, ADC>) -> Option<u16>;

    fn mode(&mut self, ctx: &mut InteractableCtx<'_, ADC>) -> Result<PinMode, ()>;

    fn set_power(&mut self, power: u8, ctx: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()>;

    fn set_mode(&mut self, mode: PinMode, ctx: &mut InteractableCtx<'_, ADC>) -> Result<(), ()>;
}

impl<T: Interactable<ADC> + ?Sized, ADC> Interactable<ADC> for &mut T {
    fn id(&self) -> PinId { T::id(self) }

    fn flags(&self) -> PinFlags { T::flags(self) }

    fn power(&mut self, ctx: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()> { T::power(self, ctx) }

    fn poll_change(&mut self, ctx: &mut InteractableCtx<'_, ADC>) -> Option<u16> { T::poll_change(self, ctx) }

    fn mode(&mut self, ctx: &mut InteractableCtx<'_, ADC>) -> Result<PinMode, ()> { T::mode(self, ctx) }

    fn set_power(&mut self, power: u8, ctx: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()> { T::set_power(self, power, ctx) }

    fn set_mode(&mut self, mode: PinMode, ctx: &mut InteractableCtx<'_, ADC>) -> Result<(), ()> { T::set_mode(self, mode, ctx) }
}

pub struct ReservedPin<P> {
    _marker: PhantomData<fn(&P)>,
}

impl<P> ReservedPin<P> {
    pub const fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

impl<P: ConstPinId, ADC> Interactable<ADC> for ReservedPin<P> {
    fn id(&self) -> PinId {
        P::ID
    }

    fn flags(&self) -> PinFlags {
        PinFlags::RESERVED
    }

    fn power(&mut self, _: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()> {
        Err(())
    }

    fn poll_change(&mut self, _: &mut InteractableCtx<'_, ADC>) -> Option<u16> {
        None
    }

    fn mode(&mut self, _: &mut InteractableCtx<'_, ADC>) -> Result<PinMode, ()> {
        Err(())
    }

    fn set_power(&mut self, _: u8, _: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()> {
        Err(())
    }

    fn set_mode(&mut self, _: PinMode, _: &mut InteractableCtx<'_, ADC>) -> Result<(), ()> {
        todo!()
    }
}

pub enum DiDoPin<P> {
    Ignored { pin: Pin<Input<Floating>, P> },
    DigitalInput { pin: Pin<Input<PullUp>, P>, last_power: bool },
    DigitalOutput { pin: Pin<Output, P> },
}

impl<P: ConstPinId + port::PinOps> From<Pin<Input<Floating>, P>> for DiDoPin<P> {
    fn from(value: Pin<Input<Floating>, P>) -> Self {
        Self::Ignored { pin: value.into_floating_input() }
    }
}

impl<P: ConstPinId + port::PinOps> From<Pin<Input<PullUp>, P>> for DiDoPin<P> {
    fn from(value: Pin<Input<PullUp>, P>) -> Self {
        Self::Ignored { pin: value.into_floating_input() }
    }
}

impl<P: ConstPinId + port::PinOps> From<Pin<Output, P>> for DiDoPin<P> {
    fn from(value: Pin<Output, P>) -> Self {
        Self::Ignored { pin: value.into_floating_input() }
    }
}

impl<P: ConstPinId + port::PinOps, ADC> Interactable<ADC> for DiDoPin<P>
{
    fn id(&self) -> PinId {
        P::ID
    }

    fn flags(&self) -> PinFlags {
        PinFlags::DIGITAL_INPUT | PinFlags::DIGITAL_OUTPUT
    }

    fn power(&mut self, _: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()> {
        match self {
            Self::Ignored { pin } => {
                Ok(pin.is_high() as u16 * 1023)
            },
            Self::DigitalInput { pin, last_power } => {
                *last_power = pin.is_high();
                Ok(*last_power as u16 * 1023)
            },
            Self::DigitalOutput { pin } => {
                Ok(pin.is_set_high() as u16 * 1023)
            },
        }
    }

    fn poll_change(&mut self, _: &mut InteractableCtx<'_, ADC>) -> Option<u16> {
        match self {
            Self::Ignored { .. } => {
                None
            },
            Self::DigitalInput { pin, last_power } => {
                let power = pin.is_high();
                if power != *last_power {
                    *last_power = pin.is_high();
                    Some(power as u16 * 1023)
                } else {
                    None
                }
            },
            Self::DigitalOutput { .. } => {
                None
            },
        }
    }

    fn mode(&mut self, _: &mut InteractableCtx<'_, ADC>) -> Result<PinMode, ()> {
        match self {
            Self::Ignored { .. } => {
                Ok(PinMode::Ignored)
            },
            Self::DigitalInput { .. } => {
                Ok(PinMode::DigitalInput)
            },
            Self::DigitalOutput { .. } => {
                Ok(PinMode::DigitalOutput)
            },
        }
    }

    fn set_power(&mut self, power: u8, _: &mut InteractableCtx<'_, ADC>) -> Result<u16, ()> {
        match self {
            Self::Ignored { .. } => {
                Err(())
            },
            Self::DigitalInput { .. } => {
                Err(())
            },
            Self::DigitalOutput { pin } => {
                if power > 0 {
                    pin.set_high();
                    Ok(1023)
                } else {
                    pin.set_low();
                    Ok(0)
                }
            },
        }
    }

    fn set_mode(&mut self, mode: PinMode, _: &mut InteractableCtx<'_, ADC>) -> Result<(), ()> {
        match (&mut *self, mode) {
            (Self::Ignored { .. }, PinMode::Ignored) => Ok(()),
            (Self::DigitalInput { .. }, PinMode::DigitalInput) => Ok(()),
            (Self::DigitalOutput { .. }, PinMode::DigitalOutput) => Ok(()),

            (Self::Ignored { pin }, PinMode::DigitalInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_pull_up_input();
                    ptr::write(self, Self::DigitalInput { last_power: pin.is_high(), pin });
                }
                Ok(())
            },
            (Self::Ignored { pin }, PinMode::DigitalOutput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_output();
                    ptr::write(self, Self::DigitalOutput { pin });
                }
                Ok(())
            },

            (Self::DigitalInput { pin, .. }, PinMode::Ignored) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_floating_input();
                    ptr::write(self, Self::Ignored { pin });
                }
                Ok(())
            },
            (Self::DigitalInput { pin, .. }, PinMode::DigitalOutput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_output();
                    ptr::write(self, Self::DigitalOutput { pin });
                }
                Ok(())
            },

            (Self::DigitalOutput { pin, .. }, PinMode::Ignored) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_floating_input();
                    ptr::write(self, Self::Ignored { pin });
                }
                Ok(())
            },
            (Self::DigitalOutput { pin, .. }, PinMode::DigitalInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_pull_up_input();
                    ptr::write(self, Self::DigitalInput { last_power: pin.is_high(), pin });
                }
                Ok(())
            },

            _ => Err(()),
        }
    }
}

pub enum DiDoAiPin<P> {
    Ignored { pin: Pin<Input<Floating>, P> },
    DigitalInput { pin: Pin<Input<PullUp>, P>, last_power: bool },
    DigitalOutput { pin: Pin<Output, P> },
    AnalogInput { pin: Pin<Analog, P>, last_power: u16 },
}

impl<P: ConstPinId + port::PinOps> From<Pin<Input<Floating>, P>> for DiDoAiPin<P>
where
    Pin<Analog, P>: adc::AdcChannel<hal::Atmega, arduino_hal::pac::ADC>,
{
    fn from(value: Pin<Input<Floating>, P>) -> Self {
        Self::Ignored { pin: value.into_floating_input() }
    }
}

impl<P: ConstPinId + port::PinOps> From<Pin<Input<PullUp>, P>> for DiDoAiPin<P>
where
    Pin<Analog, P>: adc::AdcChannel<hal::Atmega, arduino_hal::pac::ADC>,
{
    fn from(value: Pin<Input<PullUp>, P>) -> Self {
        Self::Ignored { pin: value.into_floating_input() }
    }
}

impl<P: ConstPinId + port::PinOps> From<Pin<Output, P>> for DiDoAiPin<P>
where
    Pin<Analog, P>: adc::AdcChannel<hal::Atmega, arduino_hal::pac::ADC>,
{
    fn from(value: Pin<Output, P>) -> Self {
        Self::Ignored { pin: value.into_floating_input() }
    }
}

impl<P: ConstPinId + port::PinOps> Interactable<adc::Adc> for DiDoAiPin<P>
where
    Pin<Analog, P>: adc::AdcChannel<hal::Atmega, arduino_hal::pac::ADC>,
{
    fn id(&self) -> PinId {
        P::ID
    }

    fn flags(&self) -> PinFlags {
        PinFlags::DIGITAL_INPUT | PinFlags::DIGITAL_OUTPUT | PinFlags::ANALOG_INPUT
    }

    fn power(&mut self, ctx: &mut InteractableCtx<'_, adc::Adc>) -> Result<u16, ()> {
        match self {
            Self::Ignored { pin } => {
                Ok(pin.is_high() as u16 * 1023)
            },
            Self::DigitalInput { pin, last_power } => {
                *last_power = pin.is_high();
                Ok(*last_power as u16 * 1023)
            },
            Self::DigitalOutput { pin } => {
                Ok(pin.is_set_high() as u16 * 1023)
            },
            Self::AnalogInput { pin, last_power } => {
                *last_power = pin.analog_read(ctx.adc);
                Ok(*last_power)
            },
        }
    }

    fn poll_change(&mut self, ctx: &mut InteractableCtx<'_, adc::Adc>) -> Option<u16> {
        match self {
            Self::Ignored { .. } => {
                None
            },
            Self::DigitalInput { pin, last_power } => {
                let power = pin.is_high();
                if power != *last_power {
                    *last_power = pin.is_high();
                    Some(power as u16 * 1023)
                } else {
                    None
                }
            },
            Self::DigitalOutput { .. } => {
                None
            },
            Self::AnalogInput { pin, last_power } => {
                let power = pin.analog_read(ctx.adc);
                if power != *last_power {
                    *last_power = power;
                    Some(power)
                } else {
                    None
                }
            },
        }
    }

    fn mode(&mut self, _: &mut InteractableCtx<'_, adc::Adc>) -> Result<PinMode, ()> {
        match self {
            Self::Ignored { .. } => {
                Ok(PinMode::Ignored)
            },
            Self::DigitalInput { .. } => {
                Ok(PinMode::DigitalInput)
            },
            Self::DigitalOutput { .. } => {
                Ok(PinMode::DigitalOutput)
            },
            Self::AnalogInput { .. } => {
                Ok(PinMode::AnalogInput)
            },
        }
    }

    fn set_power(&mut self, power: u8, _: &mut InteractableCtx<'_, adc::Adc>) -> Result<u16, ()> {
        match self {
            Self::Ignored { .. } => {
                Err(())
            },
            Self::DigitalInput { .. } => {
                Err(())
            },
            Self::DigitalOutput { pin } => {
                if power > 0 {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
                if power == 255 {
                    Ok(1023)
                } else {
                    Ok(power as u16 * 4)
                }
            },
            Self::AnalogInput { .. } => {
                Err(())
            },
        }
    }

    fn set_mode(&mut self, mode: PinMode, ctx: &mut InteractableCtx<'_, adc::Adc>) -> Result<(), ()> {
        match (&mut *self, mode) {
            (Self::Ignored { .. }, PinMode::Ignored) => Ok(()),
            (Self::DigitalInput { .. }, PinMode::DigitalInput) => Ok(()),
            (Self::DigitalOutput { .. }, PinMode::DigitalOutput) => Ok(()),
            (Self::AnalogInput { .. }, PinMode::AnalogInput) => Ok(()),

            (Self::Ignored { pin }, PinMode::DigitalInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_pull_up_input();
                    ptr::write(self, Self::DigitalInput { last_power: pin.is_high(), pin });
                }
                Ok(())
            },
            (Self::Ignored { pin }, PinMode::DigitalOutput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_output();
                    ptr::write(self, Self::DigitalOutput { pin });
                }
                Ok(())
            },
            (Self::Ignored { pin }, PinMode::AnalogInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_analog_input(ctx.adc);
                    ptr::write(self, Self::AnalogInput { last_power: pin.analog_read(ctx.adc), pin });
                }
                Ok(())
            },

            (Self::DigitalInput { pin, .. }, PinMode::Ignored) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_floating_input();
                    ptr::write(self, Self::Ignored { pin });
                }
                Ok(())
            },
            (Self::DigitalInput { pin, .. }, PinMode::DigitalOutput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_output();
                    ptr::write(self, Self::DigitalOutput { pin });
                }
                Ok(())
            },
            (Self::DigitalInput { pin, .. }, PinMode::AnalogInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_analog_input(ctx.adc);
                    ptr::write(self, Self::AnalogInput { last_power: pin.analog_read(ctx.adc), pin });
                }
                Ok(())
            },

            (Self::DigitalOutput { pin, .. }, PinMode::Ignored) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_floating_input();
                    ptr::write(self, Self::Ignored { pin });
                }
                Ok(())
            },
            (Self::DigitalOutput { pin, .. }, PinMode::DigitalInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_pull_up_input();
                    ptr::write(self, Self::DigitalInput { last_power: pin.is_high(), pin });
                }
                Ok(())
            },
            (Self::DigitalOutput { pin, .. }, PinMode::AnalogInput) => {
                // SAFETY: Self is uniquely borrowed.
                unsafe {
                    let pin = ptr::read(pin).into_analog_input(ctx.adc);
                    ptr::write(self, Self::AnalogInput { last_power: pin.analog_read(ctx.adc), pin });
                }
                Ok(())
            },

            _ => Err(()),
        }
    }
}