//! Architectural asynchronous exception handling.
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::exception::asynchronous::arch_asynchronous

use aarch64_cpu::registers::*;
use core::arch::asm;
use tock_registers::interfaces::{Readable, Writeable};


// Note that modifying DAIF bits should be performed with care.
// Unmasking interrupt types like FIQ without a corresponding handling function defined could lead to system instability.
mod daif_bits {
    pub const D: u8 = 0b1000;   // For Debug.
    pub const A: u8 = 0b0100;   // For SError (System error).
    pub const I: u8 = 0b0010;   // For IRQ (Interrupt Request).
    pub const F: u8 = 0b0001;   // For FIQ (Fast Interrupt Request).
}

trait DaifField {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register>;
}

struct Debug;
struct SError;
struct IRQ;
struct FIQ;


impl DaifField for Debug {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::D
    }
}

impl DaifField for SError {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::A
    }
}

impl DaifField for IRQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::I
    }
}

impl DaifField for FIQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::F
    }
}

fn is_masked<T>() -> bool
    where
        T: DaifField,
{
    DAIF.is_set(T::daif_field())
}


/// Returns whether IRQs are masked on the executing core.
pub fn is_local_irq_masked() -> bool {
    !is_masked::<IRQ>()
}

/// Unmask IRQs on the executing core.
///
/// It is not needed to place an explicit instruction synchronization barrier after the `msr`.
/// Quoting the Architecture Reference Manual for ARMv8-A, section C5.1.3:
///
/// "Writes to PSTATE.{PAN, D, A, I, F} occur in program order without the need for additional
/// synchronization."
#[inline(always)]
pub fn local_irq_unmask() {
    unsafe {
        asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::I,
        options(nomem, nostack, preserves_flags)
        );
    }
}

/// Mask IRQs on the executing core.
#[inline(always)]
pub fn local_irq_mask() {
    unsafe {
        asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::I,
        options(nomem, nostack, preserves_flags)
        );
    }
}

/// Mask IRQs on the executing core and return the previously saved interrupt mask bits (DAIF).
#[inline(always)]
pub fn local_irq_mask_save() -> u64 {
    let saved = DAIF.get();
    local_irq_mask();

    saved
}

/// Restore the interrupt mask bits (DAIF) using the callee's argument.
///
/// # Invariant
///
/// - No sanity checks on the input.
#[inline(always)]
pub fn local_irq_restore(saved: u64) {
    DAIF.set(saved);
}

/// Print the AArch64 exceptions status.
#[rustfmt::skip]
pub fn print_state() {
    use crate::info;

    let to_mask_str = |x| -> _ {
        if x { "Masked" } else { "Unmasked" }
    };

    info!("      Debug:  {}", to_mask_str(is_masked::<Debug>()));
    info!("      SError: {}", to_mask_str(is_masked::<SError>()));
    info!("      IRQ:    {}", to_mask_str(is_masked::<IRQ>()));
    info!("      FIQ:    {}", to_mask_str(is_masked::<FIQ>()));
}