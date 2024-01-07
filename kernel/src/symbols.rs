//! Debug symbol support.

use crate::memory::{Address, Virtual};
use core::{cell::UnsafeCell, slice};
use debug_symbol_types::Symbol;

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

// Symbol from the linker script.
extern "Rust" {
    static __kernel_symbols_start: UnsafeCell<()>;
}

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

/// This will be patched to the correct value by the "kernel symbols tool" after linking. This given
/// value here is just a (safe) dummy.
#[no_mangle]
static NUM_KERNEL_SYMBOLS: u64 = 0;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

fn kernel_symbol_section_virt_start_addr() -> Address<Virtual> {
    Address::new(unsafe { __kernel_symbols_start.get() as usize })
}

fn num_kernel_symbols() -> usize {
    unsafe {
        // Read volatile is needed here to prevent the compiler from optimizing NUM_KERNEL_SYMBOLS
        // away.
        core::ptr::read_volatile(&NUM_KERNEL_SYMBOLS as *const u64) as usize
    }
}

fn kernel_symbols_slice() -> &'static [Symbol] {
    let ptr = kernel_symbol_section_virt_start_addr().as_usize() as *const Symbol;

    unsafe { slice::from_raw_parts(ptr, num_kernel_symbols()) }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Retrieve the symbol corresponding to a virtual address, if any.
pub fn lookup_symbol(addr: Address<Virtual>) -> Option<&'static Symbol> {
    kernel_symbols_slice()
        .iter()
        .find(|&i| i.contains(addr.as_usize()))
}
