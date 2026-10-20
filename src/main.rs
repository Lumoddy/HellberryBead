#![no_std]
#![no_main]

mod panic;
mod packet;
mod pin;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut interactive = pin::collect_pins!(pins);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    todo!();
}