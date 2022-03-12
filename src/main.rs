#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

///this function is called on panicn.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
