//! Top-level BSP file for the Raspberry Pi 3 and 4.

pub mod driver;
pub mod cpu;
pub mod memory;
pub mod exception;

/// Board identification.
pub fn board_name() -> &'static str {
    #[cfg(feature = "bsp_rpi3")]
    {
        "Raspberry Pi 3"
    }
}