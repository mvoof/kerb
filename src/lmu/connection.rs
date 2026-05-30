use crate::error::SimError;
use crate::lmu::structs::{RF2_MAX_VEHICLES, rF2Extended, rF2Scoring, rF2Telemetry};
use crate::shm::SharedMemRegion;

const SHM_TELEMETRY: &str = "$rFactor2SMMP_Telemetry$";
const SHM_SCORING: &str = "$rFactor2SMMP_Scoring$";
const SHM_EXTENDED: &str = "$rFactor2SMMP_Extended$";

/// Point-in-time snapshot holding copies of all three rF2 shared-memory regions.
///
/// Fields are full copies (not references) because the underlying memory-mapped
/// regions can be mutated by the sim at any time.
///
/// # Data scope
///
/// - [`telemetry`](LmuFrame::telemetry) — raw array for **all vehicles**; use [`player_telemetry()`](LmuFrame::player_telemetry) for the player's car.
/// - [`scoring`](LmuFrame::scoring) — raw array for **all vehicles** plus session-wide info; use [`player_scoring_idx()`](LmuFrame::player_scoring_idx) for the player's entry.
/// - [`extended`](LmuFrame::extended) — plugin/session metadata; most users only need this via [`LmuConnection::is_connected()`].
#[derive(Clone, Copy, Debug)]
pub struct LmuFrame {
    /// Raw telemetry array for **all vehicles** in the session (up to 128).
    ///
    /// Valid entries are `vehicles[0..header.num_vehicles]`. Use [`player_telemetry()`](LmuFrame::player_telemetry)
    /// to get the player's car without manual index lookups. Iterate the full slice to build
    /// leaderboard data.
    pub telemetry: rF2Telemetry,
    /// Scoring array for **all vehicles** in the session (up to 128) plus session-wide info.
    ///
    /// Valid entries are `vehicles[0..header.num_vehicles]`. Use [`player_scoring_idx()`](LmuFrame::player_scoring_idx)
    /// to locate the player's entry. `scoring_info` contains track name, weather, flags, and session type.
    pub scoring: rF2Scoring,
    /// Plugin and session metadata.
    ///
    /// Most users only need this indirectly via [`LmuConnection::is_connected()`]. Contains
    /// `is_plugin_enabled` and `session_started` flags used to confirm live data.
    pub extended: rF2Extended,
}

impl LmuFrame {
    /// Returns the index into `scoring.vehicles` for the player's car, or `None` if not found.
    ///
    /// Use this index to access [`rF2VehicleScoring`](crate::lmu::structs::rF2VehicleScoring) fields
    /// directly (position, lap times, pit state, flags, etc.).
    ///
    /// Returns `None` before a session starts or when the player car is not yet assigned.
    pub fn player_scoring_idx(&self) -> Option<usize> {
        let n = self.scoring.header.num_vehicles as usize;
        self.scoring.vehicles[..n.min(RF2_MAX_VEHICLES)]
            .iter()
            .position(|v| v.is_player != 0)
    }

    /// Returns the player's [`rF2VehicleTelemetry`](crate::lmu::structs::rF2VehicleTelemetry) entry.
    ///
    /// Cross-references `scoring.vehicles` (by `is_player` flag) and `telemetry.vehicles`
    /// (by matching `id`) to find the correct entry. Falls back to `vehicles[0]` if no match
    /// is found (e.g. before session start).
    ///
    /// **Packed-field rule:** Always copy fields to local variables before using them in
    /// expressions (arithmetic, `println!`, function calls) — taking a reference to a packed
    /// field is a compile error in Rust.
    pub fn player_telemetry(&self) -> &crate::lmu::structs::rF2VehicleTelemetry {
        let n = self.telemetry.header.num_vehicles as usize;

        if n == 0 {
            return &self.telemetry.vehicles[0];
        }

        if let Some(idx) = self.player_scoring_idx() {
            let player_id = self.scoring.vehicles[idx].id;

            if let Some(telem_idx) = self.telemetry.vehicles[..n.min(RF2_MAX_VEHICLES)]
                .iter()
                .position(|v| v.id == player_id)
            {
                return &self.telemetry.vehicles[telem_idx];
            }
        }

        &self.telemetry.vehicles[0]
    }
}

/// Live connection to Le Mans Ultimate via the rFactor 2 shared-memory plugin.
///
/// Holds handles to the three memory-mapped files exposed by
/// `rFactor2SharedMemoryMapPlugin` (telemetry, scoring, extended).
///
/// Install `rFactor2SharedMemoryMapPlugin64.dll` to `<LMU>/Plugins/` before connecting.
pub struct LmuConnection {
    telemetry: SharedMemRegion,
    scoring: SharedMemRegion,
    extended: SharedMemRegion,
}

impl LmuConnection {
    /// Read a consistent point-in-time snapshot of all three shared-memory regions.
    ///
    /// Returns a `Box<LmuFrame>` because `LmuFrame` holds 128-vehicle arrays and
    /// is too large to safely live on the stack (~500 KB). The copy is done with
    /// `ptr::copy_nonoverlapping` for consistency.
    pub fn frame(&self) -> Box<LmuFrame> {
        unsafe {
            let mut frame = Box::<LmuFrame>::new_zeroed().assume_init();
            std::ptr::copy_nonoverlapping(
                self.telemetry.as_ptr() as *const rF2Telemetry,
                &mut frame.telemetry as *mut rF2Telemetry,
                1,
            );
            std::ptr::copy_nonoverlapping(
                self.scoring.as_ptr() as *const rF2Scoring,
                &mut frame.scoring as *mut rF2Scoring,
                1,
            );
            std::ptr::copy_nonoverlapping(
                self.extended.as_ptr() as *const rF2Extended,
                &mut frame.extended as *mut rF2Extended,
                1,
            );
            frame
        }
    }

    /// All player telemetry variables as a flat `HashMap<String, TelemetryValue>`.
    ///
    /// Scope: **player's car only** (calls `player_telemetry()` internally).
    /// Keys are field names from [`rF2VehicleTelemetry`](crate::lmu::structs::rF2VehicleTelemetry).
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        crate::lmu::snapshot::build_snapshot(&self.frame())
    }

    /// Metadata for every field in the player telemetry snapshot.
    ///
    /// Returns one [`VarMeta`](crate::types::VarMeta) per field. Only the `name` field is
    /// populated — `unit`, `desc`, and `count` are empty/default because the rF2 plugin
    /// format does not expose that metadata.
    pub fn var_list(&self) -> Vec<crate::types::VarMeta> {
        crate::lmu::snapshot::var_list()
    }

    /// Returns `true` when the rF2/LMU plugin is active and a session has started.
    ///
    /// Reads `rF2Extended::is_plugin_enabled` and `rF2Extended::session_started` from
    /// shared memory. Returns `false` during loading screens or when the plugin is missing.
    pub fn is_connected(&self) -> bool {
        unsafe {
            let ext = std::ptr::read_unaligned(self.extended.as_ptr() as *const rF2Extended);
            ext.is_plugin_enabled != 0 && ext.session_started != 0
        }
    }

    /// Sleep for up to `timeout_ms` milliseconds (capped at 16 ms).
    ///
    /// LMU does not expose a data-ready event; this is a fixed-duration sleep.
    /// Call in your polling loop to avoid busy-waiting at ~60 Hz.
    pub fn wait_for_data(&self, timeout_ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms.min(16) as u64));
    }
}

impl LmuConnection {
    /// Open the three rF2/LMU shared-memory regions (telemetry, scoring, extended).
    ///
    /// Returns [`SimError::NotConnected`] with a hint message if the plugin DLL is not installed.
    /// Install `rFactor2SharedMemoryMapPlugin64.dll` to `<LMU>/Plugins/` first.
    pub fn connect() -> Result<Self, SimError> {
        let telemetry = SharedMemRegion::open(SHM_TELEMETRY).map_err(|e| {
            SimError::NotConnected(format!(
                "{}. Is rFactor2SharedMemoryMapPlugin installed in LMU/Plugins/?",
                e
            ))
        })?;

        let scoring = SharedMemRegion::open(SHM_SCORING).map_err(SimError::NotConnected)?;
        let extended = SharedMemRegion::open(SHM_EXTENDED).map_err(SimError::NotConnected)?;

        Ok(Self {
            telemetry,
            scoring,
            extended,
        })
    }
}
