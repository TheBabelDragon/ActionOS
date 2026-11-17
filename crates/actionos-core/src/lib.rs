//! ActionOS Core — platform types and boot lifecycle.
//!
//! Genuinely `no_std`. No application state engine, no authorization,
//! no event journal. Those belong to SELF-State Kernel and BabelForge.

#![no_std]
#![forbid(unsafe_code)]

/// Contract / crate version string.
pub const VERSION: &str = "0.1.0";

/// Platform boot phases.
///
/// This is a *platform* lifecycle, not a BabelForge action lifecycle
/// and not a SELF-State Kernel state transition graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BootPhase {
    Reset,
    EarlyInit,
    MemoryReady,
    PlatformReady,
    ServicesReady,
    Running,
    /// Unrecoverable initialization failure.
    Failed,
}

impl BootPhase {
    pub fn is_terminal(self) -> bool {
        matches!(self, BootPhase::Running | BootPhase::Failed)
    }

    pub fn is_running(self) -> bool {
        matches!(self, BootPhase::Running)
    }

    /// Legal forward transitions. Skipping phases is rejected.
    pub fn can_transition_to(self, next: BootPhase) -> bool {
        use BootPhase::*;
        match self {
            Reset => matches!(next, EarlyInit | Failed),
            EarlyInit => matches!(next, MemoryReady | Failed),
            MemoryReady => matches!(next, PlatformReady | Failed),
            PlatformReady => matches!(next, ServicesReady | Failed),
            ServicesReady => matches!(next, Running | Failed),
            Running | Failed => false,
        }
    }
}

impl core::fmt::Display for BootPhase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            BootPhase::Reset => "reset",
            BootPhase::EarlyInit => "early_init",
            BootPhase::MemoryReady => "memory_ready",
            BootPhase::PlatformReady => "platform_ready",
            BootPhase::ServicesReady => "services_ready",
            BootPhase::Running => "running",
            BootPhase::Failed => "failed",
        };
        write!(f, "{s}")
    }
}

/// Structured platform error. No heap required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformError {
    IllegalTransition { from: BootPhase, to: BootPhase },
    InitFailed { phase: BootPhase, code: ErrorCode },
    Unsupported,
    InvalidMemoryRegion,
    DeviceNotFound,
    AlreadyInitialized,
    NotReady,
}

/// Stable machine-readable error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorCode {
    Unknown = 0,
    ClockInit = 1,
    MemoryMap = 2,
    InterruptController = 3,
    Console = 4,
    ServiceStart = 5,
    HardwareProbe = 6,
}

impl PlatformError {
    pub fn code(self) -> ErrorCode {
        match self {
            PlatformError::InitFailed { code, .. } => code,
            PlatformError::Unsupported => ErrorCode::HardwareProbe,
            PlatformError::InvalidMemoryRegion => ErrorCode::MemoryMap,
            _ => ErrorCode::Unknown,
        }
    }
}

/// Opaque resource identifier (no heap).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(pub u32);

/// Device identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub u32);

/// Memory region descriptor (physical view for platform bring-up).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    pub base: u64,
    pub size: u64,
    pub flags: MemoryFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryFlags {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub device: bool,
}

impl MemoryRegion {
    pub fn validate(self) -> Result<(), PlatformError> {
        if self.size == 0 {
            return Err(PlatformError::InvalidMemoryRegion);
        }
        if self.base.checked_add(self.size).is_none() {
            return Err(PlatformError::InvalidMemoryRegion);
        }
        Ok(())
    }
}

/// Capability flags reported by the platform after init.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlatformCapabilities {
    pub has_clock: bool,
    pub has_console: bool,
    pub has_interrupt_controller: bool,
    pub has_gpio: bool,
    pub host_simulated: bool,
}

/// Boot controller: pure transition logic over [`BootPhase`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootController {
    phase: BootPhase,
    capabilities: PlatformCapabilities,
    init_count: u32,
}

impl BootController {
    pub fn new() -> Self {
        Self {
            phase: BootPhase::Reset,
            capabilities: PlatformCapabilities::default(),
            init_count: 0,
        }
    }

    pub fn phase(&self) -> BootPhase {
        self.phase
    }

    pub fn capabilities(&self) -> PlatformCapabilities {
        self.capabilities
    }

    pub fn init_count(&self) -> u32 {
        self.init_count
    }

    pub fn transition(&mut self, next: BootPhase) -> Result<(), PlatformError> {
        if !self.phase.can_transition_to(next) {
            return Err(PlatformError::IllegalTransition {
                from: self.phase,
                to: next,
            });
        }
        self.phase = next;
        Ok(())
    }

    pub fn fail(&mut self, code: ErrorCode) -> PlatformError {
        let phase = self.phase;
        let _ = self.transition(BootPhase::Failed);
        PlatformError::InitFailed { phase, code }
    }

    pub fn set_capabilities(&mut self, caps: PlatformCapabilities) {
        self.capabilities = caps;
    }

    pub fn record_init_attempt(&mut self) {
        self.init_count = self.init_count.saturating_add(1);
    }
}

impl Default for BootController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_boot_path() {
        let mut b = BootController::new();
        assert_eq!(b.phase(), BootPhase::Reset);
        b.transition(BootPhase::EarlyInit).unwrap();
        b.transition(BootPhase::MemoryReady).unwrap();
        b.transition(BootPhase::PlatformReady).unwrap();
        b.transition(BootPhase::ServicesReady).unwrap();
        b.transition(BootPhase::Running).unwrap();
        assert!(b.phase().is_running());
    }

    #[test]
    fn reject_skip() {
        let mut b = BootController::new();
        let err = b.transition(BootPhase::Running).unwrap_err();
        assert!(matches!(
            err,
            PlatformError::IllegalTransition {
                from: BootPhase::Reset,
                to: BootPhase::Running
            }
        ));
    }

    #[test]
    fn reject_from_running() {
        let mut b = BootController::new();
        for p in [
            BootPhase::EarlyInit,
            BootPhase::MemoryReady,
            BootPhase::PlatformReady,
            BootPhase::ServicesReady,
            BootPhase::Running,
        ] {
            b.transition(p).unwrap();
        }
        assert!(b.transition(BootPhase::EarlyInit).is_err());
    }

    #[test]
    fn fail_path() {
        let mut b = BootController::new();
        b.transition(BootPhase::EarlyInit).unwrap();
        let e = b.fail(ErrorCode::ClockInit);
        assert_eq!(b.phase(), BootPhase::Failed);
        assert!(matches!(
            e,
            PlatformError::InitFailed {
                phase: BootPhase::EarlyInit,
                code: ErrorCode::ClockInit
            }
        ));
    }

    #[test]
    fn memory_region_validation() {
        let ok = MemoryRegion {
            base: 0x1000,
            size: 0x1000,
            flags: MemoryFlags {
                readable: true,
                writable: true,
                executable: false,
                device: false,
            },
        };
        assert!(ok.validate().is_ok());
        let bad = MemoryRegion {
            base: 0,
            size: 0,
            flags: MemoryFlags {
                readable: true,
                writable: false,
                executable: false,
                device: false,
            },
        };
        assert!(bad.validate().is_err());
    }
}
