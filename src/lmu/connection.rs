use crate::error::SimError;
use crate::lmu::structs::{rF2Extended, rF2Scoring, rF2Telemetry};
use crate::lmu::types::{LmuExtended, LmuFrame, LmuScoringInfo, LmuVehicleScoring, LmuVehicleTelemetry};
use crate::shm::SharedMemRegion;

const SHM_TELEMETRY: &str = "$rFactor2SMMP_Telemetry$";
const SHM_SCORING: &str = "$rFactor2SMMP_Scoring$";
const SHM_EXTENDED: &str = "$rFactor2SMMP_Extended$";

/// Read a consistent snapshot of `T` from a shared-memory region guarded by
/// `version_update_begin` / `version_update_end` seqlock counters.
///
/// Spins until both counters match and are even (no write in progress).
/// Returns `Err` after 100 retries to avoid an infinite loop if the sim hangs.
///
/// # Safety
/// Caller must ensure `begin_offset` and `end_offset` are valid byte offsets into
/// the region, and that the region is at least `size_of::<T>()` bytes large.
unsafe fn read_region_consistent<T: Copy>(
    region: &crate::shm::SharedMemRegion,
    begin_offset: usize,
    end_offset: usize,
) -> Result<T, crate::error::SimError> {
    let ptr = region.as_ptr();
    let mut retries = 0u32;
    loop {
        // read_volatile prevents the compiler from caching the reads across loop iterations.
        let (begin, data, end) = unsafe {
            let begin = std::ptr::read_volatile(ptr.add(begin_offset) as *const u32);
            let data = std::ptr::read_unaligned(ptr as *const T);
            let end = std::ptr::read_volatile(ptr.add(end_offset) as *const u32);
            (begin, data, end)
        };
        if begin == end && begin % 2 == 0 {
            return Ok(data);
        }
        retries += 1;
        if retries > 100 {
            return Err(crate::error::SimError::InvalidHeader(
                "Torn read timeout in LMU region".into(),
            ));
        }
        std::hint::spin_loop();
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
    /// is too large to safely live on the stack. Each region is read with
    /// seqlock consistency — the read retries if a write was in progress.
    pub fn frame(&self) -> Result<Box<LmuFrame>, crate::error::SimError> {
        use crate::lmu::structs::{RF2_MAX_VEHICLES, rF2ScoringHeader, rF2TelemetryHeader};
        unsafe {
            let raw_telemetry = read_region_consistent::<rF2Telemetry>(
                &self.telemetry,
                std::mem::offset_of!(rF2Telemetry, header)
                    + std::mem::offset_of!(rF2TelemetryHeader, version_update_begin),
                std::mem::offset_of!(rF2Telemetry, header)
                    + std::mem::offset_of!(rF2TelemetryHeader, version_update_end),
            )?;
            let raw_scoring = read_region_consistent::<rF2Scoring>(
                &self.scoring,
                std::mem::offset_of!(rF2Scoring, header)
                    + std::mem::offset_of!(rF2ScoringHeader, version_update_begin),
                std::mem::offset_of!(rF2Scoring, header)
                    + std::mem::offset_of!(rF2ScoringHeader, version_update_end),
            )?;
            let raw_extended = read_region_consistent::<rF2Extended>(
                &self.extended,
                std::mem::offset_of!(rF2Extended, version_update_begin),
                std::mem::offset_of!(rF2Extended, version_update_end),
            )?;

            let num = (raw_scoring.header.num_vehicles as usize).min(RF2_MAX_VEHICLES);

            let mut frame = Box::<LmuFrame>::new(LmuFrame::default());

            for i in 0..num {
                frame.vehicles_telemetry[i] = LmuVehicleTelemetry::from(raw_telemetry.vehicles[i]);
                frame.vehicles_scoring[i] = LmuVehicleScoring::from(raw_scoring.vehicles[i]);
            }
            frame.num_vehicles = num;
            frame.scoring_info = LmuScoringInfo::from(raw_scoring.scoring_info);
            frame.extended = LmuExtended::from(raw_extended);

            Ok(frame)
        }
    }

    /// All player telemetry variables as a flat `HashMap<String, TelemetryValue>`.
    ///
    /// Scope: **player's car only** (calls `player_telemetry()` internally).
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        match self.frame() {
            Ok(frame) => crate::lmu::snapshot::build_snapshot(&frame),
            Err(_) => std::collections::HashMap::new(),
        }
    }

    /// Metadata for every field in the player telemetry snapshot.
    pub fn var_list(&self) -> Vec<crate::types::VarMeta> {
        crate::lmu::snapshot::var_list()
    }

    /// Returns `true` when the rF2/LMU plugin is active and a session has started.
    pub fn is_connected(&self) -> bool {
        unsafe {
            let ext = std::ptr::read_unaligned(self.extended.as_ptr() as *const rF2Extended);
            ext.is_plugin_enabled != 0 && ext.session_started != 0
        }
    }

    /// Returns the player's vehicle telemetry, if available.
    pub fn player_telemetry(&self) -> Option<LmuVehicleTelemetry> {
        self.frame().ok()?.player_telemetry().cloned()
    }

    /// Sleep for up to `timeout_ms` milliseconds.
    ///
    /// LMU does not expose a data-ready event; this is a simple sleep.
    pub fn wait_for_data(&self, timeout_ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms as u64));
    }
}

impl LmuConnection {
    /// Open the three rF2/LMU shared-memory regions (telemetry, scoring, extended).
    ///
    /// Returns [`SimError::NotConnected`] with a hint message if the plugin DLL is not installed.
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
