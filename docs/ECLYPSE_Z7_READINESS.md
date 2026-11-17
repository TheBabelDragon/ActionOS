# Eclypse Z7 Readiness

Source of truth: [arty-realtime](https://github.com/TheBabelDragon/arty-realtime) ADR-001 / ADR-002.

**Board:** Digilent Eclypse Z7 (Zynq-7000 XC7Z020). Not an ESP32 stand-in.
**Host link (ADR):** Gigabit Ethernet (TCP control / UDP data); FT601 deferred.

## CPU / Rust target

| Item | Notes |
|------|-------|
| Application cores | Dual Cortex-A9 (ARMv7-A) |
| Likely Rust target | `armv7a-none-eabi` or vendor-supported equivalent |
| Status | **Not configured in ActionOS v0.1 CI** |

## Bring-up checklist (not implemented here)

| Area | Requirement | Status |
|------|-------------|--------|
| Boot firmware | FSBL / vendor startup | Missing ActionOS artifacts |
| Memory map | PS DDR + OCM from official docs | Not coded as MMIO |
| GIC | Interrupt controller init | Trait only |
| Timers | TTC / global timer | Trait only |
| UART | Console bring-up | Trait + host sim |
| Ethernet | PS GEM per ADR-002 | Out of scope v0.1 |
| PL interface | Via arty-realtime | Owned by arty-realtime |

## What host simulation covers

Boot phase machine, deterministic clock, fake memory map, console buffer, IRQ enable bitmap.

## What requires the physical board

Real DDR/OCM init, UART electrical bring-up, Ethernet MAC/PHY, PL bitstream, Zmod path.

**Do not treat a generic `none` target compile as Eclypse support.**
