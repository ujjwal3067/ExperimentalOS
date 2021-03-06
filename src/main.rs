#![allow(dead_code)]
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"[  Hello World! ] ";
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    loop {}
}
