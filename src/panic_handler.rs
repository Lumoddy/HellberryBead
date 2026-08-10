use arduino_hal::pac::wdt::wdtcsr;
use arduino_hal::prelude::*;

use crate::EscapedWriter;
use crate::packet::outgoing;
use crate::ser::Serializer;

#[inline(never)]
#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> !
{
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 9600);

    let (_, mut writer) = serial.split();

    let mut writer = EscapedWriter(|x| writer.write(x));

    let Ok(()) = writer.start();
    let Ok(()) = writer.value(outgoing::Packet::Panic(outgoing::Panic));

    dp.CPU.mcusr.write(|w| w
        .wdrf().set_bit());
    dp.WDT.wdtcsr.write(|w| w
        .wde().set_bit()
        .wdph().variant(false)
        .wdpl().variant(wdtcsr::WDPL_A::CYCLES_64K));

    loop { }
}