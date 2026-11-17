# ActionOS v0.1 — Platform Foundation

**Bare-metal-first operating environment and system integration layer** for the TheBabelDragon ecosystem.

ActionOS owns platform initialization, hardware abstraction boundaries, boot lifecycle, and composition of ecosystem services on a host or future board target.

It is **not**:

- A second BabelForge (execution / authorization / journal)
- A second SELF-State Kernel (application state authority)
- A BabelCodec replacement
- An AI agent framework
- A Linux distribution
- A claim of tested physical hardware support

```
RESET → EARLY_INIT → MEMORY_READY → PLATFORM_READY → SERVICES_READY → RUNNING
                                                                    ↓ FAILED
```

## Status

v0.1 delivers a **corrected, testable platform foundation**:

- `no_std` core (`actionos-core`) with explicit boot phases and structured errors
- Platform trait layer (`actionos-platform`)
- Host simulation (`actionos-host`) with deterministic clock and simulated devices
- Documented responsibility boundaries vs BabelForge, SELF-State Kernel, BabelCodec
- Eclypse Z7 readiness notes (requirements only — no fabricated drivers)
- CI: fmt, test, no_std check

This is **not** yet a functioning bare-metal OS.

## Quick start

```bash
cargo test --workspace
cargo run -p actionos-host
cargo check -p actionos-core --target aarch64-unknown-none
```

## Workspace

| Crate | Role |
|-------|------|
| `actionos-core` | `no_std` boot phases, errors, memory descriptors, resource IDs |
| `actionos-platform` | Hardware-facing traits (clock, console, IRQ, memory map, GPIO) |
| `actionos-host` | Host simulation + binary `actionos-host` |

## Ownership (summary)

| System | Owns |
|--------|------|
| **SELF-State Kernel** | Application/system state, invariants, integrity |
| **BabelForge** | Discover → authorize → execute → verify → journal |
| **BabelCodec** | Encoding/decoding, Unicode canonical text |
| **ActionOS** | Boot, HAL boundaries, platform lifecycle, integration adapters |

See [docs/RESPONSIBILITY_BOUNDARIES.md](docs/RESPONSIBILITY_BOUNDARIES.md).

## Documentation

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/RESPONSIBILITY_BOUNDARIES.md](docs/RESPONSIBILITY_BOUNDARIES.md)
- [docs/BOOT_MODEL.md](docs/BOOT_MODEL.md)
- [docs/BAREMETAL_ROADMAP.md](docs/BAREMETAL_ROADMAP.md)
- [docs/ECOSYSTEM_INTEGRATION.md](docs/ECOSYSTEM_INTEGRATION.md)
- [docs/ECLYPSE_Z7_READINESS.md](docs/ECLYPSE_Z7_READINESS.md)

## Prior branch note

`feat/actionos-no-std-foundation-v0.1` explored an action-orchestration shape. This branch **course-corrects** toward platform/OS responsibility so ActionOS does not duplicate BabelForge or the SELF-State Kernel.

## License

MIT
