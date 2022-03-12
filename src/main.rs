// we are compiling for bare metal system :  thumbv7em-none-eabihf
// because we don't need to use host operating system ( or library for our own operating system )
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point , since the linkeer looks for a function
    // named '_start' by default
    loop {}
}

///this function is called on panicn.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
