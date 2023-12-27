#[path = "aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
pub use arch_cpu::{nop, wait_forever};

#[cfg(feature = "bsp_rpi3")]
pub use arch_cpu::spin_for_cycles;