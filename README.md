# kerb

Rust crate for reading real-time telemetry from racing simulators via Windows Shared Memory.
Add a single dependency, enable feature flags for the simulators you want, and call `SimConnection::connect()`.

## Supported Simulators

| Simulator         | Feature flag | Notes                         |
| ----------------- | ------------ | ----------------------------- |
| iRacing           | `iracing`    | Event-based sync, 0% CPU idle |
| Assetto Corsa Evo | `ac`         | Windows Shared Memory         |
| Le Mans Ultimate  | `lmu`        | Requires rF2 plugin DLL       |

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
            let frame = c.frame()?;
            println!("rpm={}  gear={}", frame.physics.rpms, frame.physics.gear);
        }
        Connection::Lmu(c) => {
            let frame = c.frame()?;
            if let Some(player) = frame.player_telemetry() {
                let rpm = player.engine_rpm;
                let gear = player.gear;
                println!("rpm={:.0}  gear={}", rpm, gear);
            }
        }
        _ => {}
    }

    Ok(())
}
```

> [!IMPORTANT]
> The variants present in `Connection` depend on which features are enabled in your `Cargo.toml`. With `default-features = false, features = ["iracing"]` only `Connection::IRacing` exists — add `_ => {}` to handle any variants you don't care about.

> [!IMPORTANT]
> AC Evo and LMU frames use `#[repr(C, packed)]` structs mapped directly from shared memory. Rust forbids taking a reference to unaligned packed fields, so **always copy fields to local variables before using them** (e.g. in `println!`, arithmetic, or function calls). Accessing them directly will be a compile error.

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

## Per-Simulator API

### iRacing

**Connection:** `IRsdkConnection` (via `Connection::IRacing`)

| Method                 | Returns                           | Scope         | Notes                                                       |
| ---------------------- | --------------------------------- | ------------- | ----------------------------------------------------------- |
| `frame()`              | `IracingFrame`                    | player's car  | ~90 typed fields; IDE autocomplete works                    |
| `session_info()`       | `Option<IracingSession>`          | whole session | Parsed YAML; cached until iRacing reports a change          |
| `session_yaml()`       | `Option<String>`                  | whole session | Raw YAML string for manual parsing                          |
| `telemetry_snapshot()` | `HashMap<String, TelemetryValue>` | player's car  | Dynamic access by iRacing variable name                     |
| `var_list_snapshot()`  | `Vec<VarMeta>`                    | —             | All variable names, types, units, and descriptions          |
| `wait_for_data(ms)`    | `bool`                            | —             | Blocks until new data or timeout; uses Win32 event (0% CPU) |
| `is_connected()`       | `bool`                            | —             | `true` when iRacing is broadcasting telemetry               |

`IracingFrame` is a typed struct with one pub field per variable — your IDE autocomplete shows all ~90 available fields directly. Fields use snake_case (`SteeringWheelAngle` → `steering_wheel_angle`).

```rust
Connection::IRacing(conn) => {
    conn.wait_for_data(16);
    let f = conn.frame();

    println!("rpm={:.0}  speed={:.1} km/h  gear={}  throttle={:.0}%",
        f.rpm, f.speed * 3.6, f.gear, f.throttle * 100.0);

    if let Some(session) = conn.session_info() {
        let driver = session.get_value("DriverInfo.Drivers.0.UserName");
        println!("driver: {:?}", driver);
    }
}
```

To save the raw session YAML to disk:

```rust
kerb::save_session(&conn, "session.yaml")?;
```

### Assetto Corsa Evo

**Connection:** `AcConnection` (via `Connection::Ac`)

| Method                 | Returns                           | Scope        | Notes                                                           |
| ---------------------- | --------------------------------- | ------------ | --------------------------------------------------------------- |
| `frame()`              | `Result<AcFrame>`                 | player's car | Plain struct with `physics`, `graphics`, `static_data`          |
| `telemetry_snapshot()` | `HashMap<String, TelemetryValue>` | player's car | Keys are field names from the physics/graphics/static structs   |
| `var_list_snapshot()`  | `Vec<VarMeta>`                    | —            | All available field names                                       |
| `is_connected()`       | `bool`                            | —            | `true` when status == `AC_STATUS_LIVE` (not paused, not replay) |
| `wait_for_data(ms)`    | —                                 | —            | Sleep up to 16 ms; AC Evo has no data-ready event               |

**`AcFrame` contents by page:**

| Field         | Struct           | Update rate          | What it contains                                                               |
| ------------- | ---------------- | -------------------- | ------------------------------------------------------------------------------ |
| `physics`     | `AcPhysicsData`  | Every sim tick       | Inputs, RPM, speed, tyres, suspension, aero, damage, G-forces, brake bias      |
| `graphics`    | `AcGraphicsData` | Every render frame   | Lap times, position, flags, fuel, pit state, electronics, session/timing state |
| `static_data` | `AcStaticData`   | Once at session load | Car model, track name, player name, session type                               |

```rust
Connection::Ac(conn) => {
    let frame = conn.frame()?;

    let rpms = frame.physics.rpms;
    let gear = frame.physics.gear;
    let speed = frame.physics.speed_kmh;
    println!("{rpms:.0} rpm  gear {gear}  {speed:.1} km/h");

    // Electronics settings
    let abs = frame.graphics.electronics.abs_level;
    let tc  = frame.graphics.electronics.tc_level;
    let bb  = frame.physics.brake_bias;
    println!("ABS={abs}  TC={tc}  BB={bb:.3}");

    // Session info
    let track = &frame.static_data.track;
    let lap   = frame.graphics.session_state.current_lap;
    println!("track={track}  lap={lap}");

    // Brake pad life
    let pad_fl = frame.physics.pad_life[0];
    println!("pad FL: {pad_fl:.0}%");
}
```

To discover all available fields, save a snapshot:

```rust
kerb::save_telemetry_snapshot(&conn, "ac_snapshot.txt")?;
```

### Le Mans Ultimate

**Connection:** `LmuConnection` (via `Connection::Lmu`)

| Method                       | Returns                           | Scope           | Notes                                                                         |
| ---------------------------- | --------------------------------- | --------------- | ----------------------------------------------------------------------------- |
| `frame()`                    | `Box<LmuFrame>`                   | all cars        | ~500 KB struct; boxed to avoid stack overflow                                 |
| `frame.player_telemetry()`   | `Option<&LmuVehicleTelemetry>`    | **player only** | Cross-references scoring + telemetry by vehicle ID; returns None if not found |
| `frame.player_scoring_idx()` | `Option<usize>`                   | **player only** | Index into `frame.vehicles_scoring` for the player's entry                    |
| `telemetry_snapshot()`       | `HashMap<String, TelemetryValue>` | **player only** | Field names from `LmuVehicleTelemetry`                                        |
| `var_list_snapshot()`        | `Vec<VarMeta>`                    | —               | All field names from `LmuVehicleTelemetry`                                    |
| `is_connected()`             | `bool`                            | —               | `true` when plugin is loaded and session has started                          |
| `wait_for_data(ms)`          | —                                 | —               | Sleep up to 16 ms; LMU has no data-ready event                                |

```rust
Connection::Lmu(conn) => {
    let frame = conn.frame()?;

    if let Some(player) = frame.player_telemetry() {
        let rpm = player.engine_rpm;
        let gear = player.gear;
        println!("{:.0} rpm  gear {}", rpm, gear);
    }

    if let Some(idx) = frame.player_scoring_idx() {
        let place = frame.vehicles_scoring[idx].place;
        let last_lap = frame.vehicles_scoring[idx].last_lap_time;
        println!("P{}  last lap {:.3}s", place, last_lap);
    }

    for v in frame.vehicles_scoring() {
        println!("  place {}", v.place);
    }

    let track = &frame.scoring_info.track_name;
    let temp = frame.scoring_info.track_temp;
    println!("track: {}  temp: {:.1}°C", track, temp);
}
```

> [!IMPORTANT]
> **Plugin required.** Install `rFactor2SharedMemoryMapPlugin64.dll` — see [LMU Plugin Setup](#le-mans-ultimate--plugin-setup).

## Save Utilities

```rust
use kerb::{save_telemetry_snapshot, save_var_list_snapshot, save_session};

// All sims — accepts &Connection
save_telemetry_snapshot(&conn, "snapshot.txt")?;
save_var_list_snapshot(&conn, "vars.txt")?;

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
> **End users of the crate do not need this.** `src/iracing/types.rs` is already committed to the repository with all current iRacing variables. Re-run codegen only if iRacing adds or changes variables after an SDK update.

`IracingFrame` is a struct with one pub field per iRacing variable. Field names are snake_case of the iRacing variable name (`SteeringWheelAngle` → `steering_wheel_angle`).

### How to regenerate

1. Start iRacing and enter a session (practice, qualifying, or race)
2. Run codegen — it connects to the live session, reads all variables, and writes `types.rs`:

```bash
cargo run --manifest-path tools/iracing_type_gen/Cargo.toml -- src/iracing/types.rs
```

3. Commit the updated `src/iracing/types.rs`

## Benchmarks

```bash
cargo bench --all-features
```

Covers CP-1252 decoding, frame copies, snapshot HashMap allocation, and iRacing session cache behavior.

## Examples

```bash
cargo run -p kerb-examples --example facade_iracing
cargo run -p kerb-examples --example facade_ac
cargo run -p kerb-examples --example facade_lmu
```

## Simulator SDK References

| Simulator         | Documentation                                                                                                                                                                                                                                                                         |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| iRacing           | [iRacing SDK](https://members.iracing.com/jforum/posts/list/1470675.page) (login required). Community reference: [irsdkdocs](https://sajax.github.io/irsdkdocs/)                                                                                                                      |
| Assetto Corsa Evo | [Shared Memory API Documentation](https://www.assettocorsa.net/forum/index.php?threads/shared-memory-api-documentation.83659/) — official Kunos thread; [struct reference](https://docs.google.com/document/d/1WzqMLkW2o_C0LGcvdMRelAV31ZIifux0CSHD9k6ddz0/edit?tab=t.0) — Google Doc |
| Le Mans Ultimate  | Uses [rF2SharedMemoryMapPlugin](https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin) — community plugin built on ISI/S397 internals sample                                                                                                                                 |

## License

MIT — see [LICENSE](LICENSE)
