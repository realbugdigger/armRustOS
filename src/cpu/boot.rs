//! Boot code.

#[cfg(target_arch = "aarch64")]
#[path = "../aarch64/cpu/boot.rs"]
mod arch_boot;