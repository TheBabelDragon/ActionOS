# Bare-metal Roadmap

## Now (v0.1)

- `no_std` core + platform traits
- Host simulation
- Documented Eclypse Z7 requirements

## Next

1. Board crate skeleton for Zynq-7000 Cortex-A9
2. Linker scripts and startup from measured vendor/FSBL flow
3. UART bring-up with lab evidence
4. QEMU where faithful; otherwise host-only until hardware lab time

## Explicitly not claimed

- Compiles for a generic `none` target ⇒ runs on Eclypse Z7
- Register-level drivers without datasheet + lab verification
- ESP32 as a substitute for the Eclypse Z7 target

See [ECLYPSE_Z7_READINESS.md](ECLYPSE_Z7_READINESS.md).
