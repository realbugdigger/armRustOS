#![allow(clippy::upper_case_acronyms)]
#![feature(alloc_error_handler)]
#![feature(asm_const)]
#![feature(const_option)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(nonzero_min_max)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(unchecked_math)]
#![feature(step_trait)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(int_roundings)]
#![feature(is_sorted)]
#![feature(linkage)]

#![no_std]
#![no_main]

extern crate alloc;

mod bsp;
mod console;
mod cpu;
mod print;
mod exception;
mod time;
mod synchronization;
mod driver;
mod memory;
mod common;
mod state;
mod symbols;
mod backtrace;

use alloc::boxed::Box;
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use core::time::Duration;

/// Stop immediately if called a second time.
///
/// # Note
///
/// Using atomics here relieves us from needing to use `unsafe` for the static variable.
///
/// On `AArch64`, which is the only implemented architecture at the time of writing this,
/// [`AtomicBool::load`] and [`AtomicBool::store`] are lowered to ordinary load and store
/// instructions. They are therefore safe to use even with MMU + caching deactivated.
///
/// [`AtomicBool::load`]: core::sync::atomic::AtomicBool::load
/// [`AtomicBool::store`]: core::sync::atomic::AtomicBool::store
fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);

        return;
    }

    cpu::wait_forever()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Protect against panic infinite loops if any of the following code panics itself.
    panic_prevent_reenter();

    let timestamp = time::time_manager().uptime();
    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    println!(
        "[  {:>3}.{:06}] Kernel panic!\n\n\
    Panic location:\n      File '{}', line {}, column {}\n\n\
    {}\n\n\
    {}",
        timestamp.as_secs(),
        timestamp.subsec_micros(),
        location,
        line,
        column,
        info.message().unwrap_or(&format_args!("")),
        backtrace::Backtrace
    );

    cpu::wait_forever()
}

/// Early init code.
///
/// # why is following function unsafe???
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order:
///     - MMU + Data caching must be activated at the earliest. Without it, any atomic operations,
///       e.g. the yet-to-be-introduced spinlocks in the device drivers (which currently employ
///       IRQSafeNullLocks instead of spinlocks), will fail to work (properly) on the RPi SoCs.
#[no_mangle]
unsafe fn kernel_init() -> ! {
    use memory::mmu::interface::MMU;

    exception::handling_init();
    memory::init();

    // Initialize the BSP driver subsystem.
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    driver::driver_manager().init_drivers_and_irqs();

    bsp::memory::mmu::kernel_add_mapping_records_for_precomputed();

    // Unmask interrupts on the boot CPU core.
    exception::asynchronous::local_irq_unmask();

    // Announce conclusion of the kernel_init() phase.
    state::state_manager().transition_to_single_core_main();

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    info!("Booting on: {}", bsp::board_name());

    info!("MMU online:");
    memory::mmu::kernel_print_mappings();

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

    info!("Registered IRQ handlers:");
    exception::asynchronous::irq_manager().print_handler();

    // info!("Timer test, spinning for 5 seconds");
    // time::time_manager().spin_for(Duration::from_secs(5));

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    info!("************************************************");
    info!("Whoa! We recovered from a synchronous exception!");
    info!("************************************************");
    info!("");
    info!("Let's try again");

    // Cause an exception by accessing a virtual address for which no translation was set up.
    // This code accesses the address 8 GiB, which is outside the mapped address space.
    //
    // For demo purposes, the exception handler will catch the faulting 8 GiB address and allow
    // execution to continue.
    info!("");
    info!("Trying to read from address 8 GiB...");
    let mut big_addr: u64 = 8 * 1024 * 1024 * 1024;
    unsafe { read_volatile(big_addr as *mut u64) };

    // invoke a breakpoint exception
    unsafe {
        asm!("brk #0")
    }

    info!("Kernel heap:");
    memory::heap_alloc::kernel_heap_allocator().print_usage();

    // allocate a number on the heap
    let heap_value = Box::new(41);
    info!("heap_value at {:p}", heap_value);

    info!("Kernel heap:");
    memory::heap_alloc::kernel_heap_allocator().print_usage();

    info!("Echoing input now");
    cpu::wait_forever();
}
