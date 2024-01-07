//! Generation of kernel symbols.

#![no_std]
#![no_main]

#[cfg(feature = "generated_symbols_available")]
include!(env!("KERNEL_SYMBOLS_DEMANGLED_RS"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unimplemented!()
}