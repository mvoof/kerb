use crate::ac::structs::{
    AC_STATUS_LIVE, SPageFileGraphics, SPageFileGraphicsEvo, SPageFilePhysics, SPageFilePhysicsEvo,
    SPageFileStatic, SPageFileStaticEvo,
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
///
/// - `physics` — per-tick physics: inputs, speeds, tyres, damage. See [`SPageFilePhysics`].
/// - `graphics` — per-frame HUD state: lap times, position, flags, fuel. See [`SPageFileGraphics`].
/// - `static_data` — session constants written at load: car model, track, aids. See [`SPageFileStatic`].
#[derive(Clone, Copy, Debug)]
pub struct AcClassicFrame {
    /// Physics page — per-tick data: inputs, RPM, speed, tyres, damage.
    pub physics: SPageFilePhysics,
    /// Graphics/HUD page — per-frame data: lap times, race position, flags, pit state.
    pub graphics: SPageFileGraphics,
    /// Static page — written once at session load: car model, track, aids, limits.
    pub static_data: SPageFileStatic,
}

/// Point-in-time snapshot of the three AC Evo shared-memory pages.
///
/// All fields are the player's car only — AC Evo does not expose other cars via SHM.
///
/// - `physics` — per-tick physics including Evo-specific fields (pad/disc life, engine state, vibrations). See [`SPageFilePhysicsEvo`].
/// - `graphics` — per-frame HUD state with Evo extensions (rain forecast, delta, tyre detail). See [`SPageFileGraphicsEvo`].
/// - `static_data` — session constants including wet/dry tyre names. See [`SPageFileStaticEvo`].
#[derive(Clone, Copy, Debug)]
pub struct AcEvoFrame {
    /// Physics page — per-tick data including Evo-only fields (brake compound, pad/disc life, engine state).
    pub physics: SPageFilePhysicsEvo,
    /// Graphics/HUD page — per-frame data including Evo-only fields (rain forecast, delta, per-tyre detail).
    pub graphics: SPageFileGraphicsEvo,
    /// Static page — written once at session load; includes wet/dry tyre compound names.
    pub static_data: SPageFileStaticEvo,
}

/// Point-in-time snapshot from either Assetto Corsa or Assetto Corsa Evo.
///
/// Use the common accessor methods for fields shared by both games.
/// Match on the variant to access game-specific fields:
///
/// ```ignore
/// let frame = conn.frame();
///
/// // Common fields — work for both AC and AC Evo
/// println!("{:.0} rpm  gear {}", frame.rpms(), frame.gear());
///
/// // Evo-specific fields — match on the variant
/// if let AcFrame::Evo(f) = &frame {
///     println!("pad_life: {:?}", f.physics.pad_life);
/// }
/// ```
#[derive(Clone, Copy, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum AcFrame {
    Classic(AcClassicFrame),
    Evo(AcEvoFrame),
}

impl AcFrame {
    pub fn rpms(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.physics.rpms,
            AcFrame::Evo(f) => f.physics.rpms,
        }
    }
    pub fn gear(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.physics.gear,
            AcFrame::Evo(f) => f.physics.gear,
        }
    }
    pub fn speed_kmh(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.speed_kmh,
            AcFrame::Evo(f) => f.physics.speed_kmh,
        }
    }
    pub fn gas(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.gas,
            AcFrame::Evo(f) => f.physics.gas,
        }
    }
    pub fn brake(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.brake,
            AcFrame::Evo(f) => f.physics.brake,
        }
    }
    pub fn fuel(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.fuel,
            AcFrame::Evo(f) => f.physics.fuel,
        }
    }
    pub fn tc(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.tc,
            AcFrame::Evo(f) => f.physics.tc,
        }
    }
    pub fn abs(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.abs,
            AcFrame::Evo(f) => f.physics.abs,
        }
    }
    pub fn heading(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.heading,
            AcFrame::Evo(f) => f.physics.heading,
        }
    }
    pub fn pitch(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.pitch,
            AcFrame::Evo(f) => f.physics.pitch,
        }
    }
    pub fn roll(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.roll,
            AcFrame::Evo(f) => f.physics.roll,
        }
    }
    pub fn brake_bias(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.brake_bias,
            AcFrame::Evo(f) => f.physics.brake_bias,
        }
    }
    pub fn clutch(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.clutch,
            AcFrame::Evo(f) => f.physics.clutch,
        }
    }
    pub fn turbo_boost(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.turbo_boost,
            AcFrame::Evo(f) => f.physics.turbo_boost,
        }
    }
    pub fn air_temp(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.air_temp,
            AcFrame::Evo(f) => f.physics.air_temp,
        }
    }
    pub fn road_temp(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.physics.road_temp,
            AcFrame::Evo(f) => f.physics.road_temp,
        }
    }
    // Graphics common fields
    pub fn position(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.graphics.position,
            AcFrame::Evo(f) => f.graphics.position,
        }
    }
    pub fn completed_laps(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.graphics.completed_laps,
            AcFrame::Evo(f) => f.graphics.completed_laps,
        }
    }
    pub fn i_current_time(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.graphics.i_current_time,
            AcFrame::Evo(f) => f.graphics.i_current_time,
        }
    }
    pub fn i_last_time(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.graphics.i_last_time,
            AcFrame::Evo(f) => f.graphics.i_last_time,
        }
    }
    pub fn i_best_time(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.graphics.i_best_time,
            AcFrame::Evo(f) => f.graphics.i_best_time,
        }
    }
    pub fn is_in_pit(&self) -> i32 {
        match self {
            AcFrame::Classic(f) => f.graphics.is_in_pit,
            AcFrame::Evo(f) => f.graphics.is_in_pit,
        }
    }
    pub fn session_time_left(&self) -> f32 {
        match self {
            AcFrame::Classic(f) => f.graphics.session_time_left,
            AcFrame::Evo(f) => f.graphics.session_time_left,
        }
    }
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
    /// [`connect`](AcConnection::connect) time. Use the common accessor methods
    /// (`rpms()`, `speed_kmh()`, `gear()`, etc.) for fields shared by both variants.
    /// Match on the variant to access game-specific fields.
    ///
    /// **Packed-field rule:** Always copy struct fields to local variables before using them in
    /// expressions — taking a reference to an unaligned packed field is a compile error.
    pub fn frame(&self) -> AcFrame {
        unsafe {
            match &self.variant {
                AcShmVariant::Classic {
                    physics,
                    graphics,
                    static_data,
                } => AcFrame::Classic(AcClassicFrame {
                    physics: std::ptr::read_unaligned(physics.as_ptr() as *const SPageFilePhysics),
                    graphics: std::ptr::read_unaligned(
                        graphics.as_ptr() as *const SPageFileGraphics
                    ),
                    static_data: std::ptr::read_unaligned(
                        static_data.as_ptr() as *const SPageFileStatic
                    ),
                }),
                AcShmVariant::Evo {
                    physics,
                    graphics,
                    static_data,
                } => AcFrame::Evo(AcEvoFrame {
                    physics: std::ptr::read_unaligned(
                        physics.as_ptr() as *const SPageFilePhysicsEvo
                    ),
                    graphics: std::ptr::read_unaligned(
                        graphics.as_ptr() as *const SPageFileGraphicsEvo
                    ),
                    static_data: std::ptr::read_unaligned(
                        static_data.as_ptr() as *const SPageFileStaticEvo
                    ),
                }),
            }
        }
    }

    /// All telemetry variables as a flat `HashMap<String, TelemetryValue>`.
    ///
    /// Keys are field names from `SPageFilePhysics` / `SPageFilePhysicsEvo` (and their
    /// graphics/static counterparts). Scope: **player's car only**.
    ///
    /// Use [`save_telemetry_snapshot`](crate::save_telemetry_snapshot) to dump to a text file
    /// for field discovery.
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        crate::ac::snapshot::build_snapshot(&self.frame())
    }

    /// Metadata for every field exposed in the telemetry snapshot.
    ///
    /// Returns one [`VarMeta`](crate::types::VarMeta) per field with name, type, unit, and description.
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
    /// AC does not expose a data-ready event, so this is a simple sleep capped at 16 ms
    /// (one ~60 Hz frame). Call in your polling loop to avoid busy-waiting.
    pub fn wait_for_data(&self, timeout_ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms.min(16) as u64));
    }
}
