# Ecosystem Integration Matrix

| Repository | Owns | ActionOS responsibility | Adapter | Readiness | Missing evidence |
|------------|------|-------------------------|---------|-----------|------------------|
| self-state-kernel | State authority (Python ref) | Do not fork state store | future | Reference only | Stable Rust API |
| BabelForge | Execution lifecycle | Host/init node only | future | v0.1 public | Wire protocol |
| BabelCodec | Codecs / Unicode | No competing Unicode core | future | Public | Shared schema versioning |
| metafield / engine | Field / physics | Platform services consumer | out of tree | Active | — |
| Aurora family | Coordination / sensing | Optional net traits | out of tree | Partial | Public API surface |
| CANtheon | CAN normalize | Transport host | out of tree | Unknown | Repo evidence |
| ECUgate | Vehicle ops | No ECU writes in v0.1 | out of tree | Unknown | Safety policy |
| TensorGate | Compute experiments | None in core | — | Unknown | — |
| arty-realtime | Eclypse RT boundary | Board target requirements | docs + future board | ADRs published | PS bare-metal path |

## Integration rules

- No hard dependency on unmerged work.
- Missing components fail explicitly — never fake success.
- MetaField domain logic stays outside ActionOS core.
