# Responsibility Boundaries

## Ownership table

| Concern | Owner | ActionOS role |
|---------|-------|---------------|
| Application / system state, invariants, integrity hash | SELF-State Kernel | Host / link adapter only |
| Capability discovery, authz, typed ops, verify, journal | BabelForge | May *host* a node; does not reimplement |
| Unicode / text codecs, format adapters | BabelCodec | Explicit adapter; no competing codec |
| Platform boot, HAL traits, timers, IRQ surface | **ActionOS** | Primary |
| Network discovery | Aurora | Platform may expose NIC traits later |
| CAN normalization | CANtheon | Transport adapter host |
| Vehicle ops | ECUgate | No physical writes in this phase |
| Lattice / field compute | MetaField / TensorGate | Consumer of platform services |
| Eclypse real-time boundary | arty-realtime | Hardware contracts / ADRs consulted |

## Rules

1. Do not import unmerged branches as hard dependencies.
2. Do not duplicate BabelForge canonical hashing or execution contracts.
3. Do not create an ActionOS-local state store that competes with SELF-State Kernel.
4. Simulated host hardware must set `host_simulated = true`.
