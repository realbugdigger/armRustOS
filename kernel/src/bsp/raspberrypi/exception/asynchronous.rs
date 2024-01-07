//! BSP asynchronous exception handling.

use crate::bsp;

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Export for reuse in generic asynchronous.rs.
pub use bsp::device_driver::IRQNumber;

#[cfg(feature = "bsp_rpi3")]
pub(in crate::bsp) mod irq_map {
    use super::bsp::device_driver::{IRQNumber, PeripheralIRQ};

    pub const PL011_UART: IRQNumber = IRQNumber::Peripheral(PeripheralIRQ::new(57));
}
