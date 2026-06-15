use crate::connection::ReadResult;
use crate::error::SimError;
use crate::lmu::structs::{rF2Extended, rF2Scoring, rF2Telemetry};
use crate::lmu::types::{
    LmuExtended, LmuFrame, LmuScoringInfo, LmuVehicleScoring, LmuVehicleTelemetry,
};
use crate::shm::SharedMemRegion;

const SHM_TELEMETRY: &str = "$rFactor2SMMP_Telemetry$";
const SHM_SCORING: &str = "$rFactor2SMMP_Scoring$";
const SHM_EXTENDED: &str = "$rFactor2SMMP_Extended$";

/// Allocate a zero-initialized `Box<T>` directly on the heap.
///
/// Used for the large rF2/LMU structs (hundreds of KB) — `Box::new(T::default())`
/// would construct the value on the stack first and overflow it.
///
/// # Safety
/// `T` must be valid when all-zeroed (plain numeric data and byte arrays).
unsafe fn alloc_zeroed_box<T>() -> Box<T> {
    let layout = std::alloc::Layout::new::<T>();
    let ptr = unsafe { std::alloc::alloc_zeroed(layout) as *mut T };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    unsafe { Box::from_raw(ptr) }
}

/// Read a consistent snapshot of `T` from a shared-memory region into `out`,
/// guarded by `version_update_begin` / `version_update_end` seqlock counters.
///
/// Uses the classic seqlock reader pattern:
///   1. Read `begin` counter; skip the copy if a write is in progress (begin != end)
///   2. Copy region into `out`
///   3. Read `end` counter
///   4. If begin still equals end → consistent, return
///
/// Writes into a caller-provided buffer so hot paths can reuse the allocation.
/// Gives up after ~100 ms of torn reads to avoid an infinite loop if the sim hangs.
///
/// # Safety
/// Caller must ensure `begin_offset` and `end_offset` are valid byte offsets into
/// the region, and that the region is at least `size_of::<T>()` bytes large.
unsafe fn read_region_consistent_into<T: Copy>(
    region: &crate::shm::SharedMemRegion,
    begin_offset: usize,
    end_offset: usize,
    out: &mut T,
) -> Result<(), crate::error::SimError> {
    let ptr = region.as_ptr();
    let raw = out as *mut T;

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
            return Ok(());
        }

        retries += 1;
        if retries.is_multiple_of(1000) {
            if start.elapsed() > std::time::Duration::from_millis(100) {
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

/// Reusable destination buffers for the raw shared-memory snapshots.
///
/// Allocated lazily on the first `frame_into` call and reused for the lifetime
/// of the connection, so the hot read path performs no heap allocations.
struct LmuScratch {
    telemetry: Box<rF2Telemetry>,
    scoring: Box<rF2Scoring>,
    extended: Box<rF2Extended>,
}

impl LmuScratch {
    fn new() -> Self {
        // SAFETY: the rF2* structs are plain `#[repr(C)]` numeric data and byte
        // arrays — all-zero bytes are a valid value.
        unsafe {
            Self {
                telemetry: alloc_zeroed_box(),
                scoring: alloc_zeroed_box(),
                extended: alloc_zeroed_box(),
            }
        }
    }
}

/// Live connection to Le Mans Ultimate via the rFactor 2 shared-memory plugin.
///
/// Holds handles to the three memory-mapped files exposed by
/// `rFactor2SharedMemoryMapPlugin` (telemetry, scoring, extended).
///
/// Install `rFactor2SharedMemoryMapPlugin64.dll` to `<LMU>/Plugins/` before connecting.
///
/// # Threading
///
/// Not [`Send`]: holds raw shared-memory pointers and an interior-mutability
/// scratch buffer. Create and use the connection on a single thread (e.g. a
/// dedicated telemetry thread).
pub struct LmuConnection {
    telemetry: SharedMemRegion,
    scoring: SharedMemRegion,
    extended: SharedMemRegion,
    scratch: std::cell::RefCell<Option<LmuScratch>>,
}

impl LmuConnection {
    /// Read a consistent point-in-time snapshot of all three shared-memory regions.
    ///
    /// Returns a `Box<LmuFrame>` because `LmuFrame` holds 128-vehicle arrays and
    /// is too large to safely live on the stack. Each region is read with
    /// seqlock consistency — the read retries if a write was in progress.
    ///
    /// Allocates a fresh `LmuFrame` (~500 KB) per call. On hot paths (60 Hz
    /// polling) prefer [`frame_into`](Self::frame_into) with a reused buffer.
    pub(crate) fn frame(&self) -> Result<Box<LmuFrame>, crate::error::SimError> {
        let mut frame = LmuFrame::new_boxed();
        self.frame_into(&mut frame)?;
        Ok(frame)
    }

    /// Read a consistent snapshot into a caller-owned `LmuFrame`, without
    /// allocating.
    ///
    /// Allocation-free on the hot path: the raw shared-memory snapshots go into
    /// internal scratch buffers (allocated once, lazily) and the converted data
    /// is written into `out`. Allocate the buffer once with
    /// [`LmuFrame::new_boxed`] and reuse it across calls:
    ///
    /// ```ignore
    /// let mut frame = kerb::lmu::types::LmuFrame::new_boxed();
    /// while conn.is_connected() {
    ///     conn.wait_for_data(16);
    ///     if conn.frame_into(&mut frame).is_ok() {
    ///         // read frame.player_telemetry(), frame.vehicles_scoring(), …
    ///     }
    /// }
    /// ```
    ///
    /// On success every public field of `out` is overwritten; vehicle entries
    /// beyond `num_vehicles` may hold stale data — use the
    /// [`vehicles_telemetry()`](LmuFrame::vehicles_telemetry) /
    /// [`vehicles_scoring()`](LmuFrame::vehicles_scoring) slice accessors.
    /// On error `out` is left in an unspecified (but initialized) state.
    pub(crate) fn frame_into(&self, out: &mut LmuFrame) -> Result<(), crate::error::SimError> {
        use crate::lmu::structs::{RF2_MAX_VEHICLES, rF2ScoringHeader, rF2TelemetryHeader};

        let mut scratch_slot = self.scratch.borrow_mut();
        let scratch = scratch_slot.get_or_insert_with(LmuScratch::new);

        unsafe {
            read_region_consistent_into::<rF2Telemetry>(
                &self.telemetry,
                std::mem::offset_of!(rF2Telemetry, header)
                    + std::mem::offset_of!(rF2TelemetryHeader, version_update_begin),
                std::mem::offset_of!(rF2Telemetry, header)
                    + std::mem::offset_of!(rF2TelemetryHeader, version_update_end),
                &mut scratch.telemetry,
            )?;
            read_region_consistent_into::<rF2Scoring>(
                &self.scoring,
                std::mem::offset_of!(rF2Scoring, header)
                    + std::mem::offset_of!(rF2ScoringHeader, version_update_begin),
                std::mem::offset_of!(rF2Scoring, header)
                    + std::mem::offset_of!(rF2ScoringHeader, version_update_end),
                &mut scratch.scoring,
            )?;
            read_region_consistent_into::<rF2Extended>(
                &self.extended,
                std::mem::offset_of!(rF2Extended, version_update_begin),
                std::mem::offset_of!(rF2Extended, version_update_end),
                &mut scratch.extended,
            )?;
        }

        let num = (scratch.scoring.scoring_info.num_vehicles as usize).min(RF2_MAX_VEHICLES);

        for i in 0..num {
            out.vehicles_telemetry[i] = LmuVehicleTelemetry::from(scratch.telemetry.vehicles[i]);
            out.vehicles_scoring[i] = LmuVehicleScoring::from(scratch.scoring.vehicles[i]);
        }
        out.num_vehicles = num;
        out.scoring_info = LmuScoringInfo::from(scratch.scoring.scoring_info);
        out.extended = LmuExtended::from(*scratch.extended);

        Ok(())
    }

    /// Read the next telemetry frame after sleeping `timeout_ms`.
    ///
    /// LMU is **poll-based**: shared memory is always readable, so this
    /// method sleeps for `timeout_ms` to rate-limit, then reads. Pass `0`
    /// to read immediately without sleeping.
    ///
    /// Allocates ~500 KB per call. For hot paths (60 Hz polling) use
    /// [`read_frame_into`](Self::read_frame_into) with a reused buffer.
    ///
    /// - [`ReadResult::Frame`] — always returned when connected.
    /// - [`ReadResult::NotReady`] — never returned for LMU.
    /// - [`ReadResult::Disconnected`] — LMU plugin is not active or
    ///   session hasn't started.
    pub fn read_frame(&self, timeout_ms: u32) -> ReadResult<Box<LmuFrame>> {
        if !self.is_connected() {
            return ReadResult::Disconnected;
        }

        if timeout_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(timeout_ms as u64));
        }

        if !self.is_connected() {
            return ReadResult::Disconnected;
        }

        match self.frame() {
            Ok(frame) => ReadResult::Frame(frame),
            Err(_) => ReadResult::Disconnected,
        }
    }

    /// Allocation-free variant of [`read_frame`](Self::read_frame).
    ///
    /// Reads into a caller-owned `LmuFrame` buffer. Allocate once with
    /// [`LmuFrame::new_boxed()`] and reuse across calls:
    ///
    /// ```ignore
    /// let mut frame = kerb::lmu::types::LmuFrame::new_boxed();
    /// loop {
    ///     match conn.read_frame_into(&mut frame, 16) {
    ///         ReadResult::Frame(()) => { /* read frame.player_telemetry() */ }
    ///         ReadResult::NotReady  => continue,
    ///         ReadResult::Disconnected => break,
    ///     }
    /// }
    /// ```
    pub fn read_frame_into(&self, out: &mut LmuFrame, timeout_ms: u32) -> ReadResult<()> {
        if !self.is_connected() {
            return ReadResult::Disconnected;
        }

        if timeout_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(timeout_ms as u64));
        }

        if !self.is_connected() {
            return ReadResult::Disconnected;
        }

        match self.frame_into(out) {
            Ok(()) => ReadResult::Frame(()),
            Err(_) => ReadResult::Disconnected,
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
    pub(crate) fn is_connected(&self) -> bool {
        self.is_plugin_active() && self.is_session_started()
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
            scratch: std::cell::RefCell::new(None),
        })
    }
}
