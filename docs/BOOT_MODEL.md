# Boot Model

## Phases

| Phase | Meaning |
|-------|---------|
| RESET | Cold controller state |
| EARLY_INIT | Clocks / minimal CPU bring-up hooks |
| MEMORY_READY | Memory map validated |
| PLATFORM_READY | IRQ / device surface available |
| SERVICES_READY | Console and platform services started |
| RUNNING | Accepts higher-level service start |
| FAILED | Structured unrecoverable init failure |

## Rules

- Transitions are single-step only (`can_transition_to`).
- Skipping phases is an error (`IllegalTransition`).
- `RUNNING` and `FAILED` are terminal for the boot controller.
- Failure records `{ phase, ErrorCode }`.
- Host simulation is deterministic given the same `fail_at` configuration.

## Non-goals

- Bootloader / FSBL / U-Boot replacement
- Claiming physical board boot
- Automatic recovery to RUNNING after FAILED without a new controller instance
