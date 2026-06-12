use crate::ac_evo::structs::{
    AC_STATUS_LIVE, SPageFileGraphicsEvo, SPageFilePhysicsEvo, SPageFileStaticEvo,
};
use crate::ac_evo::types::{AcGraphicsData, AcPhysicsData, AcStaticData};
use crate::error::SimError;
use crate::shm::SharedMemRegion;

const SHM_PHYSICS: &str = "Local\\acevo_pmf_physics";
const SHM_GRAPHICS: &str = "Local\\acevo_pmf_graphics";
const SHM_STATIC: &str = "Local\\acevo_pmf_static";

/// Point-in-time snapshot of the three AC Evo shared-memory pages.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AcEvoFrame {
    /// Physics page — per-tick vehicle dynamics data.
    pub physics: AcPhysicsData,
    /// Graphics/HUD page — per-frame data: lap times, position, electronics.
    pub graphics: AcGraphicsData,
    /// Static page — written once at session load.
    pub static_data: AcStaticData,
}

/// Live connection to Assetto Corsa Evo.
///
/// # Threading
///
/// Not [`Send`]: holds raw shared-memory pointers. Create and use the
/// connection on a single thread (e.g. a dedicated telemetry thread).
pub struct AcEvoConnection {
    physics: SharedMemRegion,
    graphics: SharedMemRegion,
    static_data: SharedMemRegion,
}

impl AcEvoConnection {
    /// Connect to AC Evo shared memory (`acevo_pmf_*`).
    ///
    /// Returns [`SimError::NotConnected`] if AC Evo is not running.
    pub(crate) fn connect() -> Result<Self, SimError> {
        let physics = SharedMemRegion::open(SHM_PHYSICS).map_err(SimError::NotConnected)?;
        let graphics = SharedMemRegion::open(SHM_GRAPHICS).map_err(SimError::NotConnected)?;
        let static_data = SharedMemRegion::open(SHM_STATIC).map_err(SimError::NotConnected)?;
        Ok(Self {
            physics,
            graphics,
            static_data,
        })
    }

    /// Read a point-in-time snapshot from AC Evo shared memory.
    pub fn frame(&self) -> Result<AcEvoFrame, SimError> {
        unsafe {
            let raw_p =
                std::ptr::read_unaligned(self.physics.as_ptr() as *const SPageFilePhysicsEvo);
            let raw_g =
                std::ptr::read_unaligned(self.graphics.as_ptr() as *const SPageFileGraphicsEvo);
            let raw_s =
                std::ptr::read_unaligned(self.static_data.as_ptr() as *const SPageFileStaticEvo);
            Ok(AcEvoFrame {
                physics: AcPhysicsData::from(raw_p),
                graphics: AcGraphicsData::from(raw_g),
                static_data: AcStaticData::from(raw_s),
            })
        }
    }

    /// All telemetry variables as a flat `HashMap<String, TelemetryValue>`.
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        match self.frame() {
            Ok(frame) => crate::ac_evo::snapshot::build_snapshot(&frame),
            Err(_) => std::collections::HashMap::new(),
        }
    }

    /// Size of the graphics shared-memory region in bytes (for diagnostics).
    pub fn graphics_shm_len(&self) -> (usize, usize) {
        (
            self.graphics.len(),
            std::mem::size_of::<SPageFileGraphicsEvo>(),
        )
    }

    /// Returns `true` when AC Evo is in a live driving session.
    pub fn is_connected(&self) -> bool {
        unsafe {
            let offset = std::mem::offset_of!(SPageFileGraphicsEvo, status);
            let status = std::ptr::read_unaligned(self.graphics.as_ptr().add(offset) as *const i32);
            status == AC_STATUS_LIVE
        }
    }

    /// Sleep for up to `timeout_ms` milliseconds.
    ///
    /// AC Evo does not expose a data-ready event; use this in your polling loop.
    pub fn wait_for_data(&self, timeout_ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms as u64));
    }
}
