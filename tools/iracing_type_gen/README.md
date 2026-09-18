# iracing_type_gen

Internal code generator for the `kerb` crate. **For crate maintainers only — end users do not need this.**

## When to run

Run the generator only when iRacing telemetry variables change:

- a new iRacing update adds new variables
- an existing variable is renamed, removed, or its type changes
- a car exposes variables the catalogue has never seen (hybrid energy, DRS, push-to-pass)

## What it does

Connects to a **running iRacing session** via shared memory, reads the live variable list out of `irsdk_varHeader[numVars]`, and regenerates `src/iracing/types.rs` — the `IracingOffsets` catalogue and the `IracingFrame` struct built from it.

**Do not edit `src/iracing/types.rs` by hand** — changes will be overwritten on the next codegen run.

> [!IMPORTANT]
> iRacing must be running (in-session) when you execute the generator.

> [!WARNING]
> **The variable list is per car, and this tool overwrites rather than merges.**
> iRacing declares in each session only the variables the current car has, so a
> regeneration keeps exactly that set and silently drops everything else the
> checked-in file knew about.
>
> Measured example: regenerating in a Ferrari 499P (354 variables) adds the
> hybrid set — `EnergyERSBatteryPct`, `EnergyERSBattery`, `EnergyBatteryToMGU_KLap`,
> `PowerMGU_K`, `PowerMGU_H`, `TorqueMGU_K`, `dcMGUKDeployMode` — along with
> `dcTractionControl2`, `dcAntiRollFront`, `dcAntiRollRear` and the `HFshock*`
> channels, and removes `dcThrottleShape`, which that car does not have.
>
> So: pick a car that exposes as much as possible, then **read the diff** and put
> back anything the run deleted. A dropped field compiles fine and fails only for
> the users driving the car that had it.
>
> There is nowhere to fetch a complete list from — the official SDK headers
> contain no variable names, only the runtime layout that describes them. The
> real fix is for this tool to keep a curated catalogue and merge each session
> into it; until then the diff is the safety net.

## Usage

```bash
cargo run --manifest-path tools/iracing_type_gen/Cargo.toml -- src/iracing/types.rs
```

Run from the workspace root. Commit the regenerated `types.rs`.
