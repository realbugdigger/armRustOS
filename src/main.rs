#![allow(clippy::upper_case_acronyms)]
#![feature(asm_const)]
#![feature(const_option)]
#![feature(format_args_nl)]
#![feature(nonzero_min_max)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(unchecked_math)]

#![no_std]
#![no_main] // tell the Rust compiler that we don’t want to use the normal entry point chain

mod bsp;
mod console;
mod cpu;
mod print;
mod exception;
mod time;
mod synchronization;
mod driver;

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
unsafe fn kernel_init() -> ! {
    // Initialize the BSP driver subsystem.
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    driver::driver_manager().init_drivers();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernelMain()
}

/// The main function running after the early init.
#[no_mangle]
pub extern "C" fn kernelMain() -> ! {
    use console::console;
    use core::time::Duration;

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    let (_, privilege_level) = exception::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    exception::asynchronous::print_state();

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    info!("Timer test, spinning for 5 seconds");
    time::time_manager().spin_for(Duration::from_secs(5));

    info!("Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
