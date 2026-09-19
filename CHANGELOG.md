# Changelog

## [0.4.0] - 2026-09-19

### Added
- **`tools/iracing_type_gen/iracing_vars.toml`** — a curated variable catalogue, the union over every car seen so far, and now the source of truth `src/iracing/types.rs` is generated from. A codegen run merges the live session into it and never removes from it; retiring a variable is a manual edit of the catalogue.
- **`IracingFrame` grows from 332 to 408 fields**, merged from six cars. New groups: the GTP hybrid set (`EnergyERSBatteryPct`, `EnergyERSBattery`, `EnergyBatteryToMGU_KLap`, `EnergyMGU_KLapDeployPct`, `PowerMGU_K`, `PowerMGU_H`, `TorqueMGU_K`, `dcMGUKDeployMode`, `dcMGUKRegenGain`); DRS (`DRS_Status`, `dcDRSToggle`); the in-car adjustments (`dcDiffEntry`, `dcDiffMiddle`, `dcDiffExit`, `dcEngineBraking`, `dcBrakeBiasFine`, `dcPeakBrakeBias`, `dcTractionControl2`, `dcTractionControlToggle`, `dcAntiRollFront`, `dcAntiRollRear`, `dcBrakeMisc`, `dcLowFuelAccept`, `dcLaunchRPM`, `dcDashPage`, `dcDashPage2`); the `CF`/`CR`/`HF`/`HR`/`ROLLF`/`ROLLR`/`*SH` shock deflection and velocity channels at 60 Hz and 360 Hz; and the pit-service controls (`dpTireChange`, `dpLTireChange`, `dpRTireChange`, `dpWingFront`, `dpWingRear`).
- Codegen API for the merge: `merge_defs`, `MergeReport`, `catalogue_to_toml`, `parse_catalogue`.

### Changed
- **The generator takes two arguments, `<catalogue.toml> <output.rs>`**, and **no longer needs iRacing running**. Without the sim it generates from the catalogue alone, which is what CI and anyone without iRacing installed can do; with the sim it merges the session first and reports what it added, what it reshaped, and how many catalogued entries this car does not expose.

### Fixed
- **Corner and axle prefixes no longer split into the wrong snake_case.** The generic split read a leading code as an acronym, so `CFshockDefl` became `c_fshock_defl` and `LFSHshockDefl` became `lfs_hshock_defl`. `ROLLF`, `ROLLR`, `LFSH`, `LRSH`, `RFSH`, `RRSH`, `CF`, `CR`, `HF`, `HR` are now recognised, ordered longest first so `LFSH` is not swallowed by `LF`. No released field is renamed — every affected field is new in this version. `RollRate` is untouched; the match is case sensitive.

### Documentation
- The README and the codegen README described a catalogue that did not exist, and an `IracingFrame<'a>` borrowing the connection with `value(name)`, `all()` and ~90 accessor methods, where the generated type is an owned struct with a single `from_raw`. Both corrected.
- A new section spells out why a typed field can be missing what the sim is publishing right now: catalogued but not declared by this car, so the offset resolves `None` and the field reads as a default indistinguishable from a real zero; or published by the sim but absent from the catalogue, so no field exists at all. Both route to `var_list_snapshot()` and `telemetry_snapshot()`.


## [0.3.0] - 2026-08-06

### Added
- **iRacing remote control** — `iracing::broadcast`, a write-side counterpart to telemetry reading, wrapping the SDK's `IRSDK_BROADCASTMSG` channel. Covers pit service (`send_pit_command`), chat macros (`send_chat_macro`), cameras (`camera_switch_position`, `camera_switch_number`, `camera_set_state`), replay (`replay_search`, `replay_set_position`, `replay_set_play_speed`, `replay_search_session_time`, `replay_set_state`), telemetry and video recording, FFB and texture reloads. Raw escape hatches: `send_broadcast`, `send_broadcast3`, `send_broadcast_float`.
- `pad_car_num` — encodes car numbers carrying leading zeros, for the camera commands.
- `THIRD-PARTY-NOTICES.md` — records that the iRacing interfaces are reimplemented rather than copied, and disclaims any affiliation.

### Changed
- `windows-sys` gains the `Win32_UI_WindowsAndMessaging` feature, needed to register and post the broadcast message. `Win32_Globalization` is dropped, no longer needed now that decoding does not query the system code page.
- **`decode_cp1252` now always decodes cp1252 instead of the reader's Windows ANSI code page.** iRacing specifies a fixed encoding for the non-UTF-8 session string (`irsdkUTF8SessionStr=0` selects iso-8859-1), so it does not vary by locale. Reading it as the local ACP corrupted Latin text with diacritics on non-Western installs — `é` became `щ` on a Russian system — and could not recover non-Latin names anyway, since the sim substitutes those before writing. cp1252 rather than strict iso-8859-1 because the two differ only in `0x80-0x9F`, where the sim writes the printable punctuation cp1252 defines.

### Fixed
- **iRacing: session YAML no longer misdecodes before the first telemetry tick.** The UTF-8 encoding was detected by reading the `irsdkUTF8SessionStr` telemetry variable, which is unavailable until telemetry starts ticking, and "unknown" was treated as "not UTF-8". Detection now sniffs the `Encoding: UTF8` marker the sim writes into the YAML header, which travels with the bytes being decoded and needs no telemetry.

### Documentation
- README documents that `irsdkUTF8SessionStr=0`, iRacing's default, makes the sim substitute non-Latin characters before they reach shared memory, so non-Latin driver names cannot be recovered by any reader and require opting into UTF-8.

## [0.2.1] - 2026-06-27

### Fixed
- **iRacing: non-Latin driver names no longer appear as `???` in session YAML.** Driver name strings from shared memory are now decoded using `decode_cp1252` instead of lossy UTF-8 conversion.

## [0.2.0] - 2026-06-15

### Added
- `ReadResult<F>` — unified return type for all simulators with three variants: `Frame(f)`, `NotReady`, `Disconnected`. Re-exported from the crate root.
- `read_frame(timeout_ms)` on `IRsdkConnection`, `AcEvoConnection`, and `LmuConnection` — combines wait, connectivity check, and frame read into one call.
- `read_frame_into(out, timeout_ms)` on `LmuConnection` — allocation-free variant using a caller-owned buffer.

### Changed
- `frame()`, `frame_into()`, `is_connected()`, and `wait_for_data()` narrowed to `pub(crate)` on all connectors. Use `read_frame()` instead.
- `wait_for_data(0)` on iRacing no longer sleeps 16 ms when the event handle is null — a non-blocking poll now returns immediately.
- `read_frame()` on AC Evo and LMU checks `is_connected()` before sleeping, avoiding an unnecessary delay when the sim is already gone.

### Breaking
- `frame()`, `frame_into()`, `is_connected()`, `wait_for_data()` are no longer public.

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
