# iracing_type_gen

Internal code generator for the `kerb` crate. **For crate maintainers only — end users do not need this.**

## When to run

Run the generator only when iRacing telemetry variables change:

- a new iRacing update adds new variables
- an existing variable is renamed, removed, or its type changes

## What it does

Connects to a **running iRacing session** via shared memory, reads the live variable list, and regenerates `src/iracing/types.rs`, which contains the typed accessor methods on `IracingFrame` (e.g. `rpm()`, `speed()`, `gear()`).

**Do not edit `src/iracing/types.rs` by hand** — changes will be overwritten on the next codegen run.

> [!IMPORTANT]
> iRacing must be running (in-session) when you execute the generator.

## Usage

```bash
cargo run --manifest-path tools/iracing_type_gen/Cargo.toml -- src/iracing/types.rs
```

Run from the workspace root. Commit the regenerated `types.rs`.
