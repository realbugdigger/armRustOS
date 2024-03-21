//! Timer primitives.

#[path = "aarch64/time.rs"]
mod arch_time;

use core::time::Duration;


/// Provides time management functions.
pub struct TimeManager;


static TIME_MANAGER: TimeManager = TimeManager::new();


/// Return a reference to the global TimeManager.
pub fn time_manager() -> &'static TimeManager {
    &TIME_MANAGER
}

impl TimeManager {
    /// Create an instance.
    pub const fn new() -> Self {
        Self
    }

    /// The timer's resolution.
    pub fn resolution(&self) -> Duration {
        arch_time::resolution()
    }

    /// The uptime since power-on of the device.
    ///
    /// This includes time consumed by firmware and bootloaders.
    pub fn uptime(&self) -> Duration {
        arch_time::uptime()
    }

    /// Spin for a given duration.
    pub fn spin_for(&self, duration: Duration) {
        arch_time::spin_for(duration)
    }
}