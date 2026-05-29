use crate::error::SimError;
use crate::lmu::structs::{RF2_MAX_VEHICLES, rF2Extended, rF2Scoring, rF2Telemetry};
use crate::shm::SharedMemRegion;

const SHM_TELEMETRY: &str = "$rFactor2SMMP_Telemetry$";
const SHM_SCORING: &str = "$rFactor2SMMP_Scoring$";
const SHM_EXTENDED: &str = "$rFactor2SMMP_Extended$";

/// A point-in-time snapshot holding copies of all three rF2 shared-memory regions.
///
/// Fields are full copies (not references) because the underlying memory-mapped
/// regions can be mutated by the sim at any time.
///
/// # Which field to use
///
/// - `telemetry` — per-vehicle physics data updated every sim tick: RPM, gear, speed,
///   throttle/brake/steering, tyre temps, suspension, etc. Use [`player_telemetry()`] to
///   get your own car without indexing manually.
///
/// - `scoring` — race state updated ~2 Hz: position, lap count, pit state, flags,
///   sector times, track conditions, session type. Use [`player_scoring_idx()`] to get
///   your own car's entry.
///
/// - `extended` — plugin/session meta: whether the plugin is active, session started
///   flag, physics thread timing. Most users only need this via [`LmuConnection::is_connected()`].
///
/// [`player_telemetry()`]: LmuFrame::player_telemetry
/// [`player_scoring_idx()`]: LmuFrame::player_scoring_idx
#[derive(Clone, Copy, Debug)]
pub struct LmuFrame {
    /// Per-vehicle physics data updated every sim tick: RPM, gear, speed, throttle/brake/steering,
    /// tyre temps, suspension. Use [`player_telemetry()`](LmuFrame::player_telemetry) to get your car.
    pub telemetry: rF2Telemetry,
    /// Race state updated ~2 Hz: position, lap count, pit state, flags, sector times, session type.
    /// Use [`player_scoring_idx()`](LmuFrame::player_scoring_idx) to get your car's entry.
    pub scoring: rF2Scoring,
    /// Plugin/session meta: whether the plugin is active and the session has started.
    /// Most users only need this indirectly via [`LmuConnection::is_connected()`].
    pub extended: rF2Extended,
}

impl LmuFrame {
    /// Returns the index of the player's entry in `scoring.vehicles`, or `None`
    /// if no vehicle has `is_player` set.
    ///
    /// Use this to access `rF2VehicleScoring` fields directly:
    ///
    /// ```ignore
    /// let frame = conn.frame();
    /// if let Some(idx) = frame.player_scoring_idx() {
    ///     let headlights = frame.scoring.vehicles[idx].headlights;
    /// }
    /// ```
    pub fn player_scoring_idx(&self) -> Option<usize> {
        let n = self.scoring.header.num_vehicles as usize;
        self.scoring.vehicles[..n.min(RF2_MAX_VEHICLES)]
            .iter()
            .position(|v| v.is_player != 0)
    }

    /// Returns the player's vehicle telemetry entry.
    ///
    /// The scoring and telemetry arrays are indexed independently, so this
    /// cross-references them by matching `vehicle.id` from the scoring array
    /// against the telemetry array. Falls back to index 0 if no match is found.
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
pub struct LmuConnection {
    telemetry: SharedMemRegion,
    scoring: SharedMemRegion,
    extended: SharedMemRegion,
}

impl LmuConnection {
    /// Reads a consistent snapshot of all three shared-memory regions.
    ///
    /// Returns a `Box<LmuFrame>` because `LmuFrame` holds 128-vehicle arrays and
    /// is too large to safely live on the stack (~500 KB).
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

    /// All current telemetry variables as a map with sim-native names and units.
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        crate::lmu::snapshot::build_snapshot(&self.frame())
    }

    /// Metadata for every telemetry variable LMU exposes.
    pub fn var_list(&self) -> Vec<crate::types::VarMeta> {
        crate::lmu::snapshot::var_list()
    }

    /// Returns `true` if the rF2/LMU plugin is active and a session has started.
    pub fn is_connected(&self) -> bool {
        unsafe {
            let ext = std::ptr::read_unaligned(self.extended.as_ptr() as *const rF2Extended);
            ext.is_plugin_enabled != 0 && ext.session_started != 0
        }
    }

    /// Sleep for up to `timeout_ms` milliseconds waiting for fresh data.
    pub fn wait_for_data(&self, timeout_ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms.min(16) as u64));
    }
}

impl LmuConnection {
    /// Open the three rF2 shared-memory regions.
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
