#![no_std]
#![no_main] // tell the Rust compiler that we don’t want to use the normal entry point chain

#![feature(global_asm)]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Startup assembly
// global_asm!(include_str!("boot.s"));

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    const UART0: *mut u8 = 0x09000000 as *mut u8;

    for byte in HELLO.iter() {
        unsafe {
            while read_volatile(UART0.offset(0x18)) & 0x20 != 0 {}
            write_volatile(UART0, *byte);
        }
    }

    loop {}
}
