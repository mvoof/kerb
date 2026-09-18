# iracing_type_gen

Internal code generator for the `kerb` crate. **For crate maintainers only — end users do not need this.**

## When to run

Run the generator only when iRacing telemetry variables change:

- a new iRacing update adds new variables
- an existing variable is renamed, removed, or its type changes
- a car exposes variables the catalogue has never seen (hybrid energy, DRS, push-to-pass)

## What it does

Generates `src/iracing/types.rs` — the `IracingOffsets` table and the `IracingFrame` struct built from it — out of `iracing_vars.toml`.

When iRacing is running it first reads the session's variable list out of `irsdk_varHeader[numVars]` in shared memory and merges it into that catalogue.

**Do not edit `src/iracing/types.rs` by hand** — changes will be overwritten on the next codegen run. Edit the catalogue instead.

> [!IMPORTANT]
> iRacing only needs to be running when you want to *extend* the catalogue.
> Generating `types.rs` from the existing catalogue works without the sim.

## The catalogue

`iracing_vars.toml` beside this README is the source of truth. It is the
**union over every car ever seen**, and `types.rs` is generated from it.

That union is necessary because iRacing declares in each session only the
variables the current car has. A Ferrari 499P publishes 354 of them including
the hybrid set (`EnergyERSBatteryPct`, `PowerMGU_K`, `dcMGUKDeployMode` …) but
no `dcThrottleShape`; a car without a hybrid publishes the mirror image. Before
the catalogue existed this tool wrote `types.rs` straight from the session, so
each run silently deleted whatever the current car happened to lack.

A merge therefore only ever **adds**. Where a variable is known to both sides
the session wins on type and element count — the sim is authoritative if an
iRacing update reshapes something — and on description and unit whenever it
actually carries them, since shared memory leaves plenty of them empty.

Entries are never removed automatically. If iRacing genuinely retires a
variable, delete its `[[var]]` block by hand.

## Usage

```bash
cargo run --manifest-path tools/iracing_type_gen/Cargo.toml -- \
  tools/iracing_type_gen/iracing_vars.toml \
  src/iracing/types.rs
```

Run from the workspace root, then `cargo fmt` — the generator emits unformatted
Rust. Commit both the updated catalogue and `types.rs`.

With iRacing running, the session is merged into the catalogue first and the
tool prints what it added, what it reshaped, and how many entries this car does
not expose. Without iRacing, it generates from the catalogue alone, which is
what CI and anyone without the sim installed can do.
