#![no_std]
#![no_main] // tell the Rust compiler that we don’t want to use the normal entry point chain

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
