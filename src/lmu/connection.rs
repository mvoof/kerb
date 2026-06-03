use crate::error::SimError;
use crate::lmu::structs::{rF2Extended, rF2Scoring, rF2Telemetry};
use crate::lmu::types::{
    LmuExtended, LmuFrame, LmuScoringInfo, LmuVehicleScoring, LmuVehicleTelemetry,
};
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
/// Read a consistent snapshot of `T` from a shared-memory region into a heap-allocated Box.
///
/// Allocating on the heap avoids stack overflow for large structs (e.g. rF2Telemetry
/// with 128 vehicles). Spins until the seqlock counters match and are even.
/// Returns `Err` after 100 retries to avoid an infinite loop if the sim hangs.
/// Read a consistent snapshot of `T` from a shared-memory region into a heap-allocated Box.
///
/// Uses the classic seqlock reader pattern:
///   1. Read `begin` counter
///   2. Copy region into heap buffer
///   3. Read `end` counter
///   4. If begin == end and both even → consistent, return
///
/// Allocating on the heap avoids stack overflow for large structs (e.g. rF2Telemetry).
/// Retries up to 1000 times before giving up.
unsafe fn read_region_consistent<T: Copy>(
    region: &crate::shm::SharedMemRegion,
    begin_offset: usize,
    end_offset: usize,
) -> Result<Box<T>, crate::error::SimError> {
    let ptr = region.as_ptr();

    // Pre-allocate the destination buffer once outside the retry loop.
    let layout = std::alloc::Layout::new::<T>();
    let raw = unsafe { std::alloc::alloc(layout) as *mut T };
    if raw.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    let start = std::time::Instant::now();
    let mut retries = 0u32;
    loop {
        // Step 1: read begin; skip the copy if a write is in progress (begin != end).
        let begin = unsafe { std::ptr::read_volatile(ptr.add(begin_offset) as *const u32) };
        let end_pre = unsafe { std::ptr::read_volatile(ptr.add(end_offset) as *const u32) };
        if begin != end_pre {
            // Write in progress — spin without copying.
            retries += 1;
            if retries.is_multiple_of(1000) {
                if start.elapsed() > std::time::Duration::from_millis(100) {
                    unsafe { std::alloc::dealloc(raw as *mut u8, layout) };
                    return Err(crate::error::SimError::InvalidHeader(
                        "Torn read timeout in LMU region".into(),
                    ));
                }
                std::thread::yield_now();
            } else {
                std::hint::spin_loop();
            }
            continue;
        }

        // Step 2: copy the whole region (only when consistent).
        unsafe { std::ptr::copy_nonoverlapping(ptr, raw as *mut u8, std::mem::size_of::<T>()) };

        // Step 3: read end after the copy — if it changed, a write snuck in.
        let end = unsafe { std::ptr::read_volatile(ptr.add(end_offset) as *const u32) };

        // Step 4: consistent if end still matches begin.
        if begin == end {
            return Ok(unsafe { Box::from_raw(raw) });
        }

        retries += 1;
        if retries.is_multiple_of(1000) {
            if start.elapsed() > std::time::Duration::from_millis(100) {
                unsafe { std::alloc::dealloc(raw as *mut u8, layout) };
                return Err(crate::error::SimError::InvalidHeader(
                    "Torn read timeout in LMU region".into(),
                ));
            }
            std::thread::yield_now();
        } else {
            std::hint::spin_loop();
        }
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

            let num = (raw_scoring.scoring_info.num_vehicles as usize).min(RF2_MAX_VEHICLES);

            let mut frame = {
                let layout = std::alloc::Layout::new::<LmuFrame>();
                let ptr = std::alloc::alloc_zeroed(layout) as *mut LmuFrame;
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                Box::from_raw(ptr)
            };

            for i in 0..num {
                frame.vehicles_telemetry[i] = LmuVehicleTelemetry::from(raw_telemetry.vehicles[i]);
                frame.vehicles_scoring[i] = LmuVehicleScoring::from(raw_scoring.vehicles[i]);
            }
            frame.num_vehicles = num;
            frame.scoring_info = LmuScoringInfo::from(raw_scoring.scoring_info);
            frame.extended = LmuExtended::from(*raw_extended);

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
    pub fn var_list_snapshot(&self) -> Vec<crate::types::VarMeta> {
        crate::lmu::snapshot::var_list_snapshot()
    }

    /// Returns `true` when the rF2/LMU shared-memory plugin is loaded and writing data.
    ///
    /// Checks that `version_update_begin` is non-zero — the plugin increments this
    /// seqlock counter on every update, so zero means no data has been written yet.
    ///
    /// This is `true` even when the game is in the main menu (no session running).
    /// Use [`is_session_started`](Self::is_session_started) to check whether a
    /// driveable session (practice / qualifying / race) is currently active.
    pub(crate) fn is_plugin_active(&self) -> bool {
        unsafe { std::ptr::read_volatile(self.extended.as_ptr() as *const u32) != 0 }
    }

    /// Returns `true` when a driveable session has started (practice, qualifying, or race).
    ///
    /// Will be `false` while the game is loading or sitting in the main menu even if
    /// [`is_plugin_active`](Self::is_plugin_active) returns `true`.
    pub(crate) fn is_session_started(&self) -> bool {
        unsafe {
            let offset = std::mem::offset_of!(rF2Extended, session_started);
            let ptr = self.extended.as_ptr().add(offset);
            std::ptr::read_volatile(ptr) != 0
        }
    }

    /// Returns `true` when the plugin is active **and** a session has started.
    ///
    /// Equivalent to `is_plugin_active() && is_session_started()`.
    pub fn is_connected(&self) -> bool {
        self.is_plugin_active() && self.is_session_started()
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
    pub(crate) fn connect() -> Result<Self, SimError> {
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
