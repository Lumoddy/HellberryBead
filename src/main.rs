#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(abi_avr_interrupt)]

use arduino_hal::prelude::*;

mod panic_handler;
mod setup;
mod ser;
mod de;
mod endian;
mod pin;
mod packet;

#[arduino_hal::entry]
fn main() -> !
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, arduino_hal::adc::AdcSettings::default());

    // let a = serial.write();

    let a = pins.a0.into_analog_input(&mut adc);
    // let a = pins.d0.into_analog_input(&mut adc);

    todo!()
}