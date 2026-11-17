//! ActionOS Platform — hardware-facing traits.
//!
//! Implementations live in host simulation or future board crates.
//! Unsupported operations must fail with [`PlatformError::Unsupported`].

#![no_std]
#![forbid(unsafe_code)]

use actionos_core::{DeviceId, MemoryRegion, PlatformError, ResourceId};

/// Monotonic clock in milliseconds since an arbitrary epoch.
pub trait Clock {
    fn now_ms(&self) -> u64;
}

/// One-shot or periodic timer.
pub trait Timer {
    fn schedule_ms(&mut self, delay_ms: u64) -> Result<(), PlatformError>;
    fn cancel(&mut self) -> Result<(), PlatformError>;
}

/// Console / serial output (bring-up).
pub trait Console {
    fn write_bytes(&mut self, data: &[u8]) -> Result<usize, PlatformError>;
}

/// Interrupt controller surface (minimal).
pub trait InterruptController {
    fn enable(&mut self, irq: u32) -> Result<(), PlatformError>;
    fn disable(&mut self, irq: u32) -> Result<(), PlatformError>;
}

/// GPIO pin.
pub trait GpioPin {
    fn set_high(&mut self) -> Result<(), PlatformError>;
    fn set_low(&mut self) -> Result<(), PlatformError>;
    fn is_high(&self) -> Result<bool, PlatformError>;
}

/// Memory map provider for early bring-up.
pub trait MemoryMap {
    fn region_count(&self) -> usize;
    fn region_at(&self, index: usize) -> Result<MemoryRegion, PlatformError>;
}

/// Device registry probe.
pub trait DeviceProbe {
    fn probe(&self, id: DeviceId) -> Result<ResourceId, PlatformError>;
}

/// Aggregate platform services available after `PlatformReady`.
pub trait PlatformServices: Clock {
    fn console(&mut self) -> Option<&mut dyn Console>;
    fn interrupt_controller(&mut self) -> Option<&mut dyn InterruptController>;
}
