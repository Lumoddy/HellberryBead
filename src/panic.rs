use core::panic::{Location, PanicInfo};

use crate::packet::{outgoing, surround_frame};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut buffer = heapless::Deque::new();

    let location = info.location().unwrap_or(Location::caller());

    let response = outgoing::Response::Panic {
        line: location.line(),
        column: location.column(),
        file: location.file().as_bytes(),
    };
    outgoing::serialize(&response, &mut buffer);
    surround_frame(&mut buffer);

    for byte in buffer.iter() {
        serial.write_byte(*byte);
    }

    loop { }
}
