#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

#![no_std]
#![no_main] // tell the Rust compiler that we don’t want to use the normal entry point chain

mod bsp;
mod console;
mod cpu;
mod print;

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn kernelMain() -> ! {
    println!("Hello from ArmRustOs!");

    loop {}
}
