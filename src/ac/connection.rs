use crate::ac::structs::{
    AC_STATUS_LIVE, SPageFileGraphics, SPageFileGraphicsEvo, SPageFilePhysics, SPageFilePhysicsEvo,
    SPageFileStatic, SPageFileStaticEvo,
};
use crate::ac::types::{
    AcGraphicsData, AcGraphicsDataEvo, AcPhysicsData, AcPhysicsDataEvo, AcStaticData,
    AcStaticDataEvo,
};
use crate::error::SimError;
use crate::shm::SharedMemRegion;

const SHM_CLASSIC_PHYSICS: &str = "Local\\acpmf_physics";
const SHM_CLASSIC_GRAPHICS: &str = "Local\\acpmf_graphics";
const SHM_CLASSIC_STATIC: &str = "Local\\acpmf_static";

const SHM_EVO_PHYSICS: &str = "Local\\acevo_pmf_physics";
const SHM_EVO_GRAPHICS: &str = "Local\\acevo_pmf_graphics";
const SHM_EVO_STATIC: &str = "Local\\acevo_pmf_static";

/// Point-in-time snapshot of the three classic Assetto Corsa shared-memory pages.
///
/// All fields are the player's car only — AC does not expose other cars via SHM.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AcClassicFrame {
    /// Physics page — per-tick data: inputs, RPM, speed, tyres, damage.
    pub physics: crate::ac::types::AcPhysicsData,
    /// Graphics/HUD page — per-frame data: lap times, race position, flags, pit state.
    pub graphics: crate::ac::types::AcGraphicsData,
    /// Static page — written once at session load: car model, track, aids, limits.
    pub static_data: crate::ac::types::AcStaticData,
}

/// Point-in-time snapshot of the three AC Evo shared-memory pages.
///
/// All fields are the player's car only — AC Evo does not expose other cars via SHM.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AcEvoFrame {
    /// Physics page — per-tick data including Evo-only fields (brake compound, pad/disc life, engine state).
    pub physics: crate::ac::types::AcPhysicsDataEvo,
    /// Graphics/HUD page — per-frame data including Evo-only fields (rain forecast, delta, per-tyre detail).
    pub graphics: crate::ac::types::AcGraphicsDataEvo,
    /// Static page — written once at session load; includes wet/dry tyre compound names.
    pub static_data: crate::ac::types::AcStaticDataEvo,
}

/// Point-in-time snapshot from either Assetto Corsa or Assetto Corsa Evo.
///
/// Match on the variant to access fields:
///
/// ```ignore
/// let frame = conn.frame()?;
///
/// match &frame {
///     AcFrame::Classic(f) => {
///         println!("{:.0} rpm  gear {}", f.physics.rpms, f.physics.gear);
///     }
///     AcFrame::Evo(f) => {
///         println!("{:.0} rpm  gear {}", f.physics.rpms, f.physics.gear);
///         println!("pad_life: {:?}", f.physics.pad_life);
///     }
/// }
/// ```
#[derive(Clone, Debug, serde::Serialize)]
pub enum AcFrame {
    Classic(Box<AcClassicFrame>),
    Evo(Box<AcEvoFrame>),
}

enum AcShmVariant {
    Classic {
        physics: SharedMemRegion,
        graphics: SharedMemRegion,
        static_data: SharedMemRegion,
    },
    Evo {
        physics: SharedMemRegion,
        graphics: SharedMemRegion,
        static_data: SharedMemRegion,
    },
}

/// Live connection to Assetto Corsa or Assetto Corsa Evo.
///
/// [`connect`](AcConnection::connect) auto-detects which game is running by
/// trying the AC Evo shared memory first, then falling back to classic AC.
pub struct AcConnection {
    variant: AcShmVariant,
}

impl AcConnection {
    /// Connect to whichever Assetto Corsa game is currently running.
    ///
    /// Tries AC Evo shared memory (`acevo_pmf_*`) first, then falls back to classic AC (`acpmf_*`).
    /// Returns [`SimError::NotConnected`] if neither game's shared memory is available.
    pub fn connect() -> Result<Self, SimError> {
        // Try Evo first
        if let (Ok(p), Ok(g), Ok(s)) = (
            SharedMemRegion::open(SHM_EVO_PHYSICS),
            SharedMemRegion::open(SHM_EVO_GRAPHICS),
            SharedMemRegion::open(SHM_EVO_STATIC),
        ) {
            return Ok(Self {
                variant: AcShmVariant::Evo {
                    physics: p,
                    graphics: g,
                    static_data: s,
                },
            });
        }
        // Fall back to classic AC
        let physics = SharedMemRegion::open(SHM_CLASSIC_PHYSICS).map_err(SimError::NotConnected)?;
        let graphics =
            SharedMemRegion::open(SHM_CLASSIC_GRAPHICS).map_err(SimError::NotConnected)?;
        let static_data =
            SharedMemRegion::open(SHM_CLASSIC_STATIC).map_err(SimError::NotConnected)?;
        Ok(Self {
            variant: AcShmVariant::Classic {
                physics,
                graphics,
                static_data,
            },
        })
    }

    /// Read a point-in-time snapshot from the connected game.
    ///
    /// Returns [`AcFrame::Classic`] or [`AcFrame::Evo`] depending on which game was detected at
    /// [`connect`](AcConnection::connect) time.
    pub fn frame(&self) -> Result<AcFrame, SimError> {
        unsafe {
            match &self.variant {
                AcShmVariant::Classic {
                    physics,
                    graphics,
                    static_data,
                } => {
                    let raw_p =
                        std::ptr::read_unaligned(physics.as_ptr() as *const SPageFilePhysics);
                    let raw_g =
                        std::ptr::read_unaligned(graphics.as_ptr() as *const SPageFileGraphics);
                    let raw_s =
                        std::ptr::read_unaligned(static_data.as_ptr() as *const SPageFileStatic);
                    Ok(AcFrame::Classic(Box::new(AcClassicFrame {
                        physics: AcPhysicsData::from(raw_p),
                        graphics: AcGraphicsData::from(raw_g),
                        static_data: AcStaticData::from(raw_s),
                    })))
                }
                AcShmVariant::Evo {
                    physics,
                    graphics,
                    static_data,
                } => {
                    let raw_p =
                        std::ptr::read_unaligned(physics.as_ptr() as *const SPageFilePhysicsEvo);
                    let raw_g =
                        std::ptr::read_unaligned(graphics.as_ptr() as *const SPageFileGraphicsEvo);
                    let raw_s =
                        std::ptr::read_unaligned(static_data.as_ptr() as *const SPageFileStaticEvo);
                    Ok(AcFrame::Evo(Box::new(AcEvoFrame {
                        physics: AcPhysicsDataEvo::from(raw_p),
                        graphics: AcGraphicsDataEvo::from(raw_g),
                        static_data: AcStaticDataEvo::from(raw_s),
                    })))
                }
            }
        }
    }

    /// All telemetry variables as a flat `HashMap<String, TelemetryValue>`.
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        match self.frame() {
            Ok(frame) => crate::ac::snapshot::build_snapshot(&frame),
            Err(_) => std::collections::HashMap::new(),
        }
    }

    /// Metadata for every field exposed in the telemetry snapshot.
    pub fn var_list(&self) -> Vec<crate::types::VarMeta> {
        crate::ac::snapshot::var_list()
    }

    /// Returns `true` when the sim is actively running (status == `AC_STATUS_LIVE`).
    ///
    /// Returns `false` during menus, replays, or when the game is paused.
    pub fn is_connected(&self) -> bool {
        unsafe {
            let status = match &self.variant {
                AcShmVariant::Classic { graphics, .. } => {
                    std::ptr::read_unaligned(graphics.as_ptr() as *const SPageFileGraphics).status
                }
                AcShmVariant::Evo { graphics, .. } => {
                    std::ptr::read_unaligned(graphics.as_ptr() as *const SPageFileGraphicsEvo)
                        .status
                }
            };
            status == AC_STATUS_LIVE
        }
    }

    /// Sleep for up to `timeout_ms` milliseconds.
    ///
    /// AC does not expose a data-ready event, so this is a simple sleep.
    /// Call in your polling loop to avoid busy-waiting.
    pub fn wait_for_data(&self, timeout_ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms as u64));
    }
}
