use arduino_hal::pac::wdt::wdtcsr;

use crate::endian::BigEndianness;

#[inline(never)]
#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> !
{
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    // let pins = arduino_hal::pins!(dp);
    // let serial = arduino_hal::default_serial!(dp, pins, 9600);

    // let (_, writer) = serial.split();

    // let mut writer = Writer(writer);

    // type Endian = BigEndianness;

    // let Ok(()) = writer.write_begin();
    // let Ok(()) = write_outgoing_packet::<_, Endian>(&mut writer, OutgoingPacket::Panic);

    dp.CPU.mcusr.write(|w| w
        .wdrf().set_bit());
    dp.WDT.wdtcsr.write(|w| w
        .wde().set_bit()
        .wdph().variant(false)
        .wdpl().variant(wdtcsr::WDPL_A::CYCLES_64K));

    loop { }
}