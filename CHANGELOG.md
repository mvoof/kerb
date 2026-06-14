# Changelog

## [0.1.2] - 2026-06-14

### Fixed
- **iRacing: `wait_for_data` now correctly signals disconnect when `h_event` is null.**
  When `OpenEventW` failed (insufficient privileges or the event not yet created), `wait_for_data` fell back to `sleep(16ms)` and unconditionally returned `true` — callers could never detect a disconnect through this path. It now returns `is_connected()` after the sleep, so callers see `false` as soon as iRacing closes, matching the behaviour of the `WaitForSingleObject` path.
- Fixed alignment UB in the `make_header` test helper.

### Tests
- Added three unit tests via `new_mock`: connected status → `true`, disconnected status → `false`, `is_connected` bit reading.

## [0.1.1] - 2026-06-14

### Added
- `[package.metadata.docs.rs]` — docs.rs now builds under `x86_64-pc-windows-msvc` with all features enabled, fixing the failed build badge
- Per-variant doc comments on `TelemetryValue` — clearly distinguishes `Char` (raw byte), `String` (null-terminated CP-1252 buffer decoded to UTF-8), and `Text` (human-readable prose)
- Field-level doc comments on `VarMeta` (`name`, `type_name`, `unit`, `desc`, `count`)
- Method doc comments on `HasSnapshot::telemetry_snapshot` and `HasSnapshot::var_list_snapshot`

### Changed
- README badges: removed CI badge, added Windows-only platform badge and crates.io download count badge; badges are now centered

## [0.1.0] - 2026-06-14

### Added
- Initial release
- iRacing support via Windows Shared Memory (`Local\IRSDKMemMapFileName`) with event-based sync and zero CPU idle
- Assetto Corsa Evo support (`Local\acevo_pmf_*`)
- Le Mans Ultimate / rFactor 2 support with seqlock-guarded zero-alloc frame reads (`$rFactor2SMMP_*$`)
- `SimConnection::connect()` — auto-detects the first running sim
- `SimConnection::connect_to(SimType)` — explicit sim selection
- `IracingFrame` — ~90 typed fields generated from `iracing_vars.toml`
- `IracingSession` — parsed YAML session info with version-based caching
- `HasSnapshot` trait — unified `telemetry_snapshot()` / `var_list_snapshot()` across all sims
- `save_telemetry_snapshot`, `save_var_list_snapshot`, `save_session` utility helpers
- `decode_cp1252` for correct iRacing string decoding
