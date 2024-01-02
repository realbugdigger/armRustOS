//! Device driver.

#[cfg(feature = "bsp_rpi4")]
mod arm;
#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod bcm;
mod common;

#[cfg(feature = "bsp_rpi4")]
pub use arm::*;
#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use bcm::*;