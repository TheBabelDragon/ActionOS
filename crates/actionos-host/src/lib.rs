//! ActionOS Host — simulated platform for development and tests.
//!
//! Simulated hardware is **not** physical hardware. Capabilities are
//! reported with `host_simulated = true`.

use actionos_core::{
    BootController, BootPhase, DeviceId, ErrorCode, MemoryFlags, MemoryRegion, PlatformCapabilities,
    PlatformError, ResourceId,
};
use actionos_platform::{
    Clock, Console, DeviceProbe, InterruptController, MemoryMap, PlatformServices, Timer,
};

/// Deterministic simulated clock.
#[derive(Debug)]
pub struct SimClock {
    ms: u64,
}

impl SimClock {
    pub fn new(start_ms: u64) -> Self {
        Self { ms: start_ms }
    }

    pub fn advance(&mut self, delta: u64) {
        self.ms = self.ms.saturating_add(delta);
    }

    pub fn set(&mut self, ms: u64) {
        self.ms = ms;
    }
}

impl Clock for SimClock {
    fn now_ms(&self) -> u64 {
        self.ms
    }
}

/// In-memory console buffer.
#[derive(Debug, Default)]
pub struct SimConsole {
    pub buffer: Vec<u8>,
}

impl Console for SimConsole {
    fn write_bytes(&mut self, data: &[u8]) -> Result<usize, PlatformError> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }
}

/// Simulated interrupt controller.
#[derive(Debug, Default)]
pub struct SimIrq {
    enabled: [bool; 32],
}

impl InterruptController for SimIrq {
    fn enable(&mut self, irq: u32) -> Result<(), PlatformError> {
        if irq as usize >= self.enabled.len() {
            return Err(PlatformError::Unsupported);
        }
        self.enabled[irq as usize] = true;
        Ok(())
    }

    fn disable(&mut self, irq: u32) -> Result<(), PlatformError> {
        if irq as usize >= self.enabled.len() {
            return Err(PlatformError::Unsupported);
        }
        self.enabled[irq as usize] = false;
        Ok(())
    }
}

impl SimIrq {
    pub fn is_enabled(&self, irq: u32) -> bool {
        self.enabled.get(irq as usize).copied().unwrap_or(false)
    }
}

/// Fixed memory map for host simulation.
#[derive(Debug)]
pub struct SimMemoryMap {
    regions: Vec<MemoryRegion>,
}

impl SimMemoryMap {
    pub fn default_map() -> Self {
        Self {
            regions: vec![
                MemoryRegion {
                    base: 0x0000_0000,
                    size: 0x0001_0000,
                    flags: MemoryFlags {
                        readable: true,
                        writable: false,
                        executable: true,
                        device: false,
                    },
                },
                MemoryRegion {
                    base: 0x2000_0000,
                    size: 0x0010_0000,
                    flags: MemoryFlags {
                        readable: true,
                        writable: true,
                        executable: false,
                        device: false,
                    },
                },
            ],
        }
    }
}

impl MemoryMap for SimMemoryMap {
    fn region_count(&self) -> usize {
        self.regions.len()
    }

    fn region_at(&self, index: usize) -> Result<MemoryRegion, PlatformError> {
        self.regions
            .get(index)
            .copied()
            .ok_or(PlatformError::InvalidMemoryRegion)
    }
}

/// Device probe: only id 1 exists in the sim.
#[derive(Debug, Default)]
pub struct SimDevices;

impl DeviceProbe for SimDevices {
    fn probe(&self, id: DeviceId) -> Result<ResourceId, PlatformError> {
        if id.0 == 1 {
            Ok(ResourceId(1))
        } else {
            Err(PlatformError::DeviceNotFound)
        }
    }
}

/// Unsupported timer placeholder (explicit failure).
#[derive(Debug, Default)]
pub struct UnsupportedTimer;

impl Timer for UnsupportedTimer {
    fn schedule_ms(&mut self, _delay_ms: u64) -> Result<(), PlatformError> {
        Err(PlatformError::Unsupported)
    }

    fn cancel(&mut self) -> Result<(), PlatformError> {
        Err(PlatformError::Unsupported)
    }
}

/// Full host platform aggregate.
pub struct HostPlatform {
    pub boot: BootController,
    pub clock: SimClock,
    pub console: SimConsole,
    pub irq: SimIrq,
    pub memory: SimMemoryMap,
    pub devices: SimDevices,
    /// When true, force failure at a specific phase during `boot_to_running`.
    pub fail_at: Option<BootPhase>,
}

impl HostPlatform {
    pub fn new() -> Self {
        Self {
            boot: BootController::new(),
            clock: SimClock::new(0),
            console: SimConsole::default(),
            irq: SimIrq::default(),
            memory: SimMemoryMap::default_map(),
            devices: SimDevices,
            fail_at: None,
        }
    }

    /// Drive the platform through the full boot sequence.
    pub fn boot_to_running(&mut self) -> Result<(), PlatformError> {
        self.boot.record_init_attempt();

        self.step(BootPhase::EarlyInit)?;
        if self.should_fail(BootPhase::EarlyInit) {
            return Err(self.boot.fail(ErrorCode::ClockInit));
        }
        let _ = self.clock.now_ms();

        self.step(BootPhase::MemoryReady)?;
        if self.should_fail(BootPhase::MemoryReady) {
            return Err(self.boot.fail(ErrorCode::MemoryMap));
        }
        for i in 0..self.memory.region_count() {
            let r = self.memory.region_at(i)?;
            r.validate()?;
        }

        self.step(BootPhase::PlatformReady)?;
        if self.should_fail(BootPhase::PlatformReady) {
            return Err(self.boot.fail(ErrorCode::InterruptController));
        }
        self.irq.enable(0)?;

        self.step(BootPhase::ServicesReady)?;
        if self.should_fail(BootPhase::ServicesReady) {
            return Err(self.boot.fail(ErrorCode::ServiceStart));
        }
        let _ = self.console.write_bytes(b"services ready\n")?;

        self.step(BootPhase::Running)?;
        self.boot.set_capabilities(PlatformCapabilities {
            has_clock: true,
            has_console: true,
            has_interrupt_controller: true,
            has_gpio: false,
            host_simulated: true,
        });
        Ok(())
    }

    fn step(&mut self, next: BootPhase) -> Result<(), PlatformError> {
        self.boot.transition(next)
    }

    fn should_fail(&self, phase: BootPhase) -> bool {
        self.fail_at == Some(phase)
    }
}

impl Default for HostPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for HostPlatform {
    fn now_ms(&self) -> u64 {
        self.clock.now_ms()
    }
}

impl PlatformServices for HostPlatform {
    fn console(&mut self) -> Option<&mut dyn Console> {
        Some(&mut self.console)
    }

    fn interrupt_controller(&mut self) -> Option<&mut dyn InterruptController> {
        Some(&mut self.irq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_boot_sequence() {
        let mut p = HostPlatform::new();
        p.boot_to_running().unwrap();
        assert_eq!(p.boot.phase(), BootPhase::Running);
        assert!(p.boot.capabilities().host_simulated);
        assert!(p.boot.capabilities().has_clock);
    }

    #[test]
    fn reject_skip_init() {
        let mut p = HostPlatform::new();
        assert!(p.boot.transition(BootPhase::PlatformReady).is_err());
        assert_eq!(p.boot.phase(), BootPhase::Reset);
    }

    #[test]
    fn init_failure_at_memory() {
        let mut p = HostPlatform::new();
        p.fail_at = Some(BootPhase::MemoryReady);
        let err = p.boot_to_running().unwrap_err();
        assert_eq!(p.boot.phase(), BootPhase::Failed);
        assert!(matches!(
            err,
            PlatformError::InitFailed {
                phase: BootPhase::MemoryReady,
                code: ErrorCode::MemoryMap
            }
        ));
    }

    #[test]
    fn repeated_init_from_fresh_controller() {
        let mut p = HostPlatform::new();
        p.boot_to_running().unwrap();
        assert_eq!(p.boot.init_count(), 1);
        let mut p2 = HostPlatform::new();
        p2.boot_to_running().unwrap();
        assert_eq!(p2.boot.init_count(), 1);
    }

    #[test]
    fn cannot_reenter_running() {
        let mut p = HostPlatform::new();
        p.boot_to_running().unwrap();
        assert!(p.boot.transition(BootPhase::ServicesReady).is_err());
    }

    #[test]
    fn deterministic_clock() {
        let mut c = SimClock::new(100);
        assert_eq!(c.now_ms(), 100);
        c.advance(50);
        assert_eq!(c.now_ms(), 150);
        c.set(0);
        assert_eq!(c.now_ms(), 0);
    }

    #[test]
    fn unsupported_timer() {
        let mut t = UnsupportedTimer;
        assert_eq!(t.schedule_ms(10), Err(PlatformError::Unsupported));
    }

    #[test]
    fn device_identity() {
        let d = SimDevices;
        assert_eq!(d.probe(DeviceId(1)).unwrap(), ResourceId(1));
        assert_eq!(d.probe(DeviceId(99)), Err(PlatformError::DeviceNotFound));
    }

    #[test]
    fn memory_regions_valid() {
        let m = SimMemoryMap::default_map();
        assert!(m.region_count() >= 2);
        for i in 0..m.region_count() {
            m.region_at(i).unwrap().validate().unwrap();
        }
        assert!(m.region_at(99).is_err());
    }

    #[test]
    fn irq_bounds() {
        let mut irq = SimIrq::default();
        irq.enable(0).unwrap();
        assert!(irq.is_enabled(0));
        assert_eq!(irq.enable(100), Err(PlatformError::Unsupported));
    }
}
