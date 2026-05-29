# kerb-codegen

Internal code generator for the `kerb` crate. **For crate maintainers only — end users do not need this.**

## When to run

Run the generator only when iRacing telemetry variables change:

- a new iRacing update adds new variables
- an existing variable is renamed, removed, or its type changes

## What it does

Connects to a **running iRacing session** via shared memory, reads the live variable list, and regenerates `src/iracing/vars.rs`, which contains the typed accessor methods on `IracingFrame<'a>` (e.g. `rpm()`, `speed()`, `gear()`).

**Do not edit `src/iracing/vars.rs` by hand** — changes will be overwritten on the next codegen run.

> [!IMPORTANT]
> iRacing must be running (in-session) when you execute the generator.

## Usage

```bash
cargo run --manifest-path tools/codegen/Cargo.toml -- src/iracing/vars.rs
```

Run from the workspace root. Commit the regenerated `vars.rs`.
