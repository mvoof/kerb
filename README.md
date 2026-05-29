# kerb

Rust crate for reading real-time telemetry from racing simulators via Windows Shared Memory.
Add a single dependency, enable feature flags for the simulators you want, and call `SimConnection::connect()`.

## Supported Simulators

| Simulator           | Feature flag | Notes                                   |
| ------------------- | ------------ | --------------------------------------- |
| iRacing             | `iracing`    | Event-based sync, 0% CPU idle           |
| Assetto Corsa       | `ac`         | Auto-detected via Shared Memory         |
| Assetto Corsa Rally | `ac`         | Same SHM layout as AC                   |
| Assetto Corsa Evo   | `ac`         | Auto-detected — same feature flag as AC |
| Le Mans Ultimate    | `lmu`        | Requires rF2 plugin DLL                 |

## Quick Start

All simulators are enabled by default. Add to `Cargo.toml`:

```toml
[dependencies]
kerb = "0.1"
```

To include only specific simulators, disable defaults:

```toml
[dependencies]
kerb = { version = "0.1", default-features = false, features = ["iracing"] }
```

Call `SimConnection::connect()` — it auto-detects whichever sim is running and returns it wrapped in a `Connection` enum. Match on the variant to access each sim's full API:

```rust
use kerb::{Connection, SimConnection, SimError};

fn main() -> Result<(), SimError> {
    let conn = SimConnection::connect()?;

    match conn {
        Connection::IRacing(c) => {
            c.wait_for_data(16);
            let frame = c.frame();
            println!("rpm={:.0}  gear={}", frame.rpm, frame.gear);
        }
        Connection::Ac(c) => {
            let frame = c.frame();
            // Common methods work for both AC and AC Evo
            println!("rpm={:.0}  gear={}", frame.rpms(), frame.gear());
        }
        Connection::Lmu(c) => {
            let frame = c.frame();
            let player = frame.player_telemetry();
            let rpm = player.engine_rpm;
            let gear = player.gear;
            println!("rpm={:.0}  gear={}", rpm, gear);
        }
    }

    Ok(())
}
```

> [!IMPORTANT]
> The variants present in `Connection` depend on which features are enabled in your `Cargo.toml`. With `default-features = false, features = ["iracing"]` only `Connection::IRacing` exists — add `_ => {}` to handle any variants you don't care about.

> [!IMPORTANT]
> AC, AC Evo, and LMU frames use `#[repr(C, packed)]` structs mapped directly from shared memory. Rust forbids taking a reference to unaligned packed fields, so **always copy fields to local variables before using them** (e.g. in `println!`, arithmetic, or function calls). Accessing them directly will be a compile error.

## Connection Loop

For overlays that need to reconnect automatically:

```rust
use kerb::{Connection, SimConnection};
use std::io::{self, Write};

fn main() {
    loop {
        match SimConnection::connect() {
            Ok(Connection::IRacing(conn)) => {
                println!("Connected to iRacing");

                if let Some(session) = conn.session_info() {
                    let track = session.get_value("WeekendInfo.TrackDisplayName")
                        .unwrap_or_default();
                    println!("Track: {}", track);
                }

                while conn.is_connected() {
                    conn.wait_for_data(100);
                    let f = conn.frame();
                    print!("\r[{}] {:.0} rpm  {:.1} km/h",
                        f.gear, f.rpm, f.speed * 3.6);
                    let _ = io::stdout().flush();
                }
                println!("\nDisconnected.");
            }
            Ok(_) => {
                // different sim connected
            }
            Err(e) => {
                eprint!("\r{e}");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
}
```

## Multi-Sim Support

If multiple sims are running simultaneously and `connect()` picks the wrong one, use `connect_to()`:

```rust
use kerb::{SimConnection, SimType};

let conn = SimConnection::connect_to(SimType::Lmu)?;
```

## Feature Flags

All features are enabled by default. Use `default-features = false` to opt in selectively.

| Feature   | Module          | `Connection` variant  | Default |
| --------- | --------------- | --------------------- | ------- |
| `iracing` | `kerb::iracing` | `Connection::IRacing` | yes     |
| `ac`      | `kerb::ac`      | `Connection::Ac`      | yes     |
| `lmu`     | `kerb::lmu`     | `Connection::Lmu`     | yes     |

The `ac` feature covers both classic Assetto Corsa and AC Evo — no separate flag needed. `AcConnection::connect()` tries AC Evo SHM first, then falls back to classic AC.

## Per-Simulator API

### iRacing

`IracingFrame` is a typed struct with one pub field per variable — your IDE autocomplete shows all ~90 available fields directly. Fields use snake_case (`SteeringWheelAngle` → `steering_wheel_angle`). To see all fields with their types and units, save a snapshot:

```rust
kerb::save_telemetry_snapshot(&conn, "iracing_snapshot.txt")?;
```

```rust
Connection::IRacing(conn) => {
    conn.wait_for_data(16);
    let f = conn.frame();

    // Typed fields — autocomplete, no string lookups
    println!("rpm={:.0}  speed={:.1} km/h  gear={}  throttle={:.0}%",
        f.rpm, f.speed * 3.6, f.gear, f.throttle * 100.0);

    // iRacing session YAML — parsed, cached
    if let Some(session) = conn.session_info() {
        let driver = session.get_value("DriverInfo.Drivers.0.UserName");
        println!("driver: {:?}", driver);
    }
}
```

#### iRacing session YAML

`session_info()` returns `Option<IracingSession>` with cached YAML parsing. Use `get_value("Path.To.Key")` to look up any field.

To save the raw YAML to disk for exploration:

```rust
kerb::save_session(&conn, "session.yaml")?;
```

### Assetto Corsa / AC Evo

Both games use a single `Connection::Ac` variant. `connect()` auto-detects which is running.

`AcFrame` is an enum with two variants — `Classic` and `Evo`. Use the common accessor methods for fields shared by both games, and match on the variant for Evo-specific fields:

```rust
use kerb::ac::connection::AcFrame;

Connection::Ac(conn) => {
    let frame = conn.frame();

    // Common methods — work for both AC and AC Evo
    println!("{:.0} rpm  gear {}  {:.1} km/h",
        frame.rpms(), frame.gear(), frame.speed_kmh());

    // Evo-specific fields
    if let AcFrame::Evo(f) = &frame {
        let pad_fl = f.physics.pad_life[0];
        println!("pad life FL: {:.0}%", pad_fl);
    }
}
```

To discover all available fields, save a snapshot:

```rust
kerb::save_telemetry_snapshot(&conn, "ac_snapshot.txt")?;
```

Then look up the field name in `SPageFilePhysics` / `SPageFilePhysicsEvo` (and their graphics/static counterparts) in `kerb::ac::structs`.

### Le Mans Ultimate

Three shared memory regions — `telemetry`, `scoring`, `extended`. `player_telemetry()` cross-references scoring and telemetry arrays to find the player's entry.

To discover all available fields, save a snapshot to a text file and open it:

```rust
kerb::save_telemetry_snapshot(&conn, "lmu_snapshot.txt")?;
```

Then look up the field name in the structs `rF2VehicleTelemetry` and `rF2VehicleScoring`.

```rust
Connection::Lmu(conn) => {
    let frame = conn.frame();
    let player = frame.player_telemetry(); // &rF2VehicleTelemetry

    // Must copy packed fields to locals before use
    let rpm = player.engine_rpm;
    let gear = player.gear;
    println!("{:.0} rpm  gear {}", rpm, gear);

    // Player's scoring entry (place, lap count, flags, headlights, etc.)
    if let Some(idx) = frame.player_scoring_idx() {
        let headlights = frame.scoring.vehicles[idx].headlights;
        println!("headlights={}", headlights);
    }

    // All cars in race
    let n = frame.scoring.header.num_vehicles as usize;
    for v in &frame.scoring.vehicles[..n] {
        let place = v.place;
        println!("  place {}", place);
    }
}
```

> [!IMPORTANT]
> **Plugin required.** Install `rFactor2SharedMemoryMapPlugin64.dll` — see [LMU Plugin Setup](#le-mans-ultimate--plugin-setup).

## Save Utilities

```rust
use kerb::{save_telemetry_snapshot, save_var_list, save_session};

// All sims — accepts &Connection
save_telemetry_snapshot(&conn, "snapshot.txt")?;
save_var_list(&conn, "vars.txt")?;

// iRacing only — accepts &IRsdkConnection
save_session(&iracing_conn, "session.yaml")?;
```

## Using from GitHub

```toml
# All simulators (default)
kerb = { git = "https://github.com/mvoof/kerb" }

# iRacing only
kerb = { git = "https://github.com/mvoof/kerb", default-features = false, features = ["iracing"] }
```

## Character Encoding

iRacing uses Windows-1252 for all strings. The crate decodes them automatically. Use `decode_cp1252(bytes)` if you need to decode raw bytes yourself.

## Le Mans Ultimate — Plugin Setup

LMU does not expose telemetry by default. Install the
[rF2SharedMemoryMapPlugin](https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin):

1. Download `rFactor2SharedMemoryMapPlugin64.dll` from the [releases page](https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin/releases)
2. Copy to `<Steam>\steamapps\common\Le Mans Ultimate\Plugins\`
3. In-game: Settings → Gameplay → Enable Plugins: **ON**
4. Restart LMU

If the plugin is missing, `SimConnection::connect()` skips LMU and tries the next enabled sim.

## Codegen — iRacing Typed Frame (for crate developers only)

> [!IMPORTANT]
> **End users of the crate do not need this.** `src/iracing/vars.rs` is already committed to the repository with all current iRacing variables. Re-run codegen only if iRacing adds or changes variables after an SDK update.

`src/iracing/vars.rs` is generated — do not edit by hand. It's just formatted with `cargo fmt`

`IracingFrame` is a struct with one pub field per iRacing variable. Field names are snake_case of the iRacing variable name (`SteeringWheelAngle` → `steering_wheel_angle`, `CarIdxLapDistPct` → `car_idx_lap_dist_pct`).

### How to regenerate

1. Start iRacing and enter a session (practice, qualifying, or race)
2. Run codegen — it connects to the live session, reads all variables, and writes `vars.rs`:

```bash
cargo run --manifest-path tools/codegen/Cargo.toml -- src/iracing/vars.rs
```

3. Commit the updated `src/iracing/vars.rs`

### When to regenerate

- After an iRacing update that mentions SDK or telemetry changes
- If you notice a new variable in iRacing that is missing from `IracingFrame`

The codegen binary will fail with a clear error if iRacing is not running.

## Benchmarks

```bash
cargo bench --all-features
```

Covers CP-1252 decoding, frame copies, snapshot HashMap allocation, and iRacing session cache behavior.

## Examples

```bash
cargo run -p kerb-examples --example facade_iracing
cargo run -p kerb-examples --example facade_ac
cargo run -p kerb-examples --example facade_ac_evo
cargo run -p kerb-examples --example facade_lmu
```

## Simulator SDK References

| Simulator                  | Documentation                                                                                                                                                            |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| iRacing                    | [iRacing SDK forum](https://forums.iracing.com/discussion/62/iracing-sdk/p1) (login required). Community reference: [irsdkdocs](https://sajax.github.io/irsdkdocs/)      |
| Assetto Corsa              | [Shared Memory Reference](https://assettocorsamods.net/threads/doc-shared-memory-reference.58/) — official Kunos thread                                                  |
| Assetto Corsa Competizione | [ACC Shared Memory Documentation](https://www.assettocorsa.net/forum/index.php?threads/acc-shared-memory-documentation.59965/) — official KS Dev Team post, includes PDF |
| Assetto Corsa Evo          | [Shared Memory Documentation](https://steamcommunity.com/sharedfiles/filedetails/?id=3707421508) — Steam guide                                                           |
| Le Mans Ultimate           | Uses [rF2SharedMemoryMapPlugin](https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin) — community plugin built on ISI/S397 internals sample                    |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
