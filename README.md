# ActionOS v0.1

**Bare-metal-first action orchestration environment** for the TheBabelDragon ecosystem.

ActionOS is the agent-facing action coordination layer. It lets intelligence, software, and physical hardware expose, discover, authorize, coordinate, and verify capabilities.

It is **not** a Linux distribution, chatbot, LLM framework, or replacement for BabelForge.

```
PROPOSED → INSPECTED → AWAITING_AUTHORIZATION → AUTHORIZED → READY
    → EXECUTING → VERIFYING → COMPLETED
```

Terminal alternatives: `DENIED` · `FAILED` · `CANCELLED` · `INDETERMINATE`

## Status

v0.1 delivers:

- Rust workspace with no_std-oriented `actionos-contracts` and `actionos-core`
- Versioned action contracts (plans, requests, authorization, receipts, events)
- Deterministic action state machine (pure transitions, no I/O)
- Explicit backend interfaces (authorization, preconditions, execution, verification)
- Host simulation harness and CLI
- Default-deny authorization demonstrated by tests
- Idempotency without automatic side-effect replay
- Adapter boundary for BabelForge (no compile-time dependency)
- Documentation of ownership boundaries and non-goals

## Quick start

```bash
cargo build
cargo test
cargo run -p actionos-cli -- version
cargo run -p actionos-cli -- simulate
cargo run -p actionos-cli -- simulate --authorize false
cargo run -p actionos-cli -- lifecycle
```

## Workspace

| Crate | Role |
|-------|------|
| actionos-contracts | Versioned action descriptors, plans, requests, receipts, errors |
| actionos-core | no_std state machine — pure deterministic transitions |
| actionos-codec | Codec abstraction; JSON host codec (no competing BabelCodec) |
| actionos-hal | Platform-independent HAL traits (clock, GPIO, storage, transport) |
| actionos-runtime | Coordinates plans through explicit backends |
| actionos-adapters | Versioned BabelForge / BabelCodec integration boundaries |
| actionos-host | std host simulation + test harness |
| actionos-cli | `actionos` binary |

## Ownership

| Concern | Owner |
|---------|-------|
| Universal execution lifecycle, PolicyEngine, event journal, identity, replay | **BabelForge** |
| Action plans, action state machine, preconditions, human-approval gating | **ActionOS** |
| Canonical Unicode / text representation | **BabelCodec** |

ActionOS **must not** reimplement BabelForge's PolicyEngine or journal.

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md)
- [SECURITY.md](SECURITY.md)
- [BAREMETAL.md](BAREMETAL.md)
- [ACTION_LIFECYCLE.md](ACTION_LIFECYCLE.md)
- [ECOSYSTEM_INTEGRATION.md](ECOSYSTEM_INTEGRATION.md)
- [docs/CONTRACTS.md](docs/CONTRACTS.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)

## License

MIT
