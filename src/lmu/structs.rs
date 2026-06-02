//! `#[repr(C, packed)]` struct definitions that mirror the rFactor 2 shared-memory
//! plugin layout byte-for-byte. Field order and sizes must match the C definitions
//! in `rF2SharedMemoryPlugin` exactly — do not reorder or add padding.

use kerb_derive::Snapshot;

/// Maximum number of vehicles the shared-memory buffers can hold.
pub const RF2_MAX_VEHICLES: usize = 128;


/// Per-wheel physics data (suspension, tyre temps, wear, forces, etc.).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default, Snapshot)]
pub(crate) struct rF2Wheel {
    /// Suspension deflection (compression) in metres; positive = compressed.
    pub suspension_deflection: f64,
    /// Ride height at this corner in metres.
    pub ride_height: f64,
    /// Suspension spring force in Newtons.
    pub susp_force: f64,
    /// Brake disc temperature in °C.
    pub brake_temp: f64,
    /// Brake pressure as a fraction of maximum, 0.0–1.0.
    pub brake_pressure: f64,
    /// Wheel rotation speed in rad/s; positive = forward.
    pub rotation: f64,
    /// Lateral velocity of the tyre contact patch relative to ground, m/s.
    pub lateral_patch_vel: f64,
    /// Longitudinal velocity of the tyre contact patch relative to ground, m/s.
    pub longitudinal_patch_vel: f64,
    /// Lateral velocity of the ground surface under the tyre, m/s.
    pub lateral_ground_vel: f64,
    /// Longitudinal velocity of the ground surface under the tyre, m/s.
    pub longitudinal_ground_vel: f64,
    /// Camber angle in radians; negative = top leaning inward.
    pub camber: f64,
    /// Lateral (cornering) force at the tyre contact patch in Newtons.
    pub lateral_force: f64,
    /// Longitudinal (tractive/braking) force at the tyre contact patch in Newtons.
    pub longitudinal_force: f64,
    /// Total vertical load on this tyre in Newtons.
    pub tire_load: f64,
    /// Grip fraction relative to nominal grip, 0.0–1.0+.
    pub grip_fract: f64,
    /// Tyre inflation pressure in kPa.
    pub pressure: f64,
    /// Tyre temperatures at [inner, middle, outer] positions in °C.
    pub temperature: [f64; 3],
    /// Tyre wear level, 0.0 (new) – 1.0 (completely worn).
    pub wear: f64,
    /// Name of the terrain type at the contact point (ASCII, null-terminated).
    pub terrain_name: [u8; 16],
    /// Surface type index: 0 = dry asphalt, other values are sim-defined.
    pub surface_type: u8,
    /// 1 when this tyre has gone flat (puncture), 0 otherwise.
    pub flat: u8,
    /// 1 when this wheel/tyre has detached from the car, 0 otherwise.
    pub detached: u8,
    /// Static tyre imbalance in grams (affects vibration).
    pub static_unbalance: u8,
    /// Vertical tyre carcass deflection in metres.
    pub vertical_tire_deflection: f64,
    /// Wheel Y-axis position offset used for visual representation, metres.
    pub wheel_ylocation: f64,
    /// Toe angle in radians; positive = toe-out.
    pub toe: f64,
    /// Tyre carcass (belt) temperature in °C.
    pub tire_carcass_temperature: f64,
    /// Tyre inner-liner temperatures at [inner, middle, outer] positions in °C.
    pub tire_inner_layer_temperature: [f64; 3],
    #[doc(hidden)]
    pub _expansion: [u8; 24],
}

/// Per-vehicle telemetry: engine, driver inputs, aerodynamics, wheels, position and orientation.
/// Only valid for vehicles present in `rF2Telemetry::vehicles[..header.num_vehicles]`.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct rF2VehicleTelemetry {
    /// Unique vehicle ID — matches `rF2VehicleScoring::id` for the same car.
    pub id: i32,
    /// Physics time step in seconds for this update.
    pub delta_time: f64,
    /// Elapsed session time at this telemetry sample in seconds.
    pub elapsed_time: f64,
    /// Current lap number (1-based).
    pub lap_number: i32,
    /// Elapsed time at the start of the current lap in seconds.
    pub lap_start_et: f64,
    /// Vehicle model name (ASCII, null-terminated, max 64 bytes).
    pub vehicle_name: [u8; 64],
    /// Track name (ASCII, null-terminated, max 64 bytes).
    pub track_name: [u8; 64],
    /// World-space position vector (x, y, z) in metres.
    pub pos: [f64; 3],
    /// Velocity in the vehicle's local coordinate frame (x, y, z) in m/s.
    pub local_vel: [f64; 3],
    /// Acceleration in the vehicle's local coordinate frame (x, y, z) in m/s².
    pub local_accel: [f64; 3],
    /// 3×3 orientation matrix: rows are the vehicle's right, up, and forward axes in world space.
    pub ori: [[f64; 3]; 3],
    /// Angular velocity in the vehicle's local frame (x, y, z) in rad/s.
    pub local_rot: [f64; 3],
    /// Angular acceleration in the vehicle's local frame (x, y, z) in rad/s².
    pub local_rot_accel: [f64; 3],
    /// Current gear: -1 = reverse, 0 = neutral, 1–N = forward gears.
    pub gear: i32,
    /// Engine speed in RPM.
    pub engine_rpm: f64,
    /// Engine coolant temperature in °C.
    pub engine_water_temp: f64,
    /// Engine oil temperature in °C.
    pub engine_oil_temp: f64,
    /// Clutch engagement RPM (RPM at the clutch output shaft).
    pub clutch_rpm: f64,
    /// Raw (unfiltered) throttle pedal position, 0.0 (off) – 1.0 (full).
    pub unfiltered_throttle: f64,
    /// Raw (unfiltered) brake pedal position, 0.0 (off) – 1.0 (full).
    pub unfiltered_brake: f64,
    /// Raw (unfiltered) steering input, −1.0 (full left) – 1.0 (full right).
    pub unfiltered_steering: f64,
    /// Raw (unfiltered) clutch pedal position, 0.0 (disengaged) – 1.0 (engaged).
    pub unfiltered_clutch: f64,
    /// TC/ABS-filtered throttle actually applied to the engine, 0.0–1.0.
    pub filtered_throttle: f64,
    /// TC/ABS-filtered brake actually applied to the brakes, 0.0–1.0.
    pub filtered_brake: f64,
    /// Filtered steering angle sent to the physics engine, −1.0–1.0.
    pub filtered_steering: f64,
    /// Filtered clutch position sent to the physics engine, 0.0–1.0.
    pub filtered_clutch: f64,
    /// Steering column torque in Nm (used for force-feedback).
    pub steering_shaft_torque: f64,
    /// Front third-spring (heave damper) deflection in metres.
    pub front3rd_deflection: f64,
    /// Rear third-spring (heave damper) deflection in metres.
    pub rear3rd_deflection: f64,
    /// Front wing height above ground in metres.
    pub front_wing_height: f64,
    /// Front ride height (average of front corners) in metres.
    pub front_ride_height: f64,
    /// Rear ride height (average of rear corners) in metres.
    pub rear_ride_height: f64,
    /// Total aerodynamic drag force in Newtons.
    pub drag: f64,
    /// Front-axle aerodynamic downforce in Newtons.
    pub front_downforce: f64,
    /// Rear-axle aerodynamic downforce in Newtons.
    pub rear_downforce: f64,
    /// Remaining fuel in litres.
    pub fuel: f64,
    /// Engine redline / maximum RPM.
    pub engine_max_rpm: f64,
    /// Number of scheduled pit stops remaining.
    pub scheduled_stops: u8,
    /// 1 when the engine is overheating, 0 otherwise.
    pub overheating: u8,
    /// 1 when a component has detached from the car, 0 otherwise.
    pub detached: u8,
    /// Headlight state: 0 = off, 1 = on.
    pub headlights: u8,
    /// Dent severity per body panel (8 panels, 0 = no damage, higher = more dented).
    pub dent_severity: [u8; 8],
    /// Elapsed session time of the last significant impact in seconds.
    pub last_impact_et: f64,
    /// Magnitude of the last significant impact force in Newtons.
    pub last_impact_magnitude: f64,
    /// World-space position where the last impact occurred (x, y, z) in metres.
    pub last_impact_pos: [f64; 3],
    /// Current engine output torque in Nm.
    pub engine_torque: f64,
    /// Current track sector: 0 = between S3 finish and S1, 1 = sector 1, 2 = sector 2, 3 = sector 3.
    pub current_sector: i32,
    /// 1 when pit-lane speed limiter is active, 0 otherwise.
    pub speed_limiter: u8,
    /// Total number of forward gears available.
    pub max_gears: u8,
    /// Front tyre compound index (maps to a compound in the vehicle's tyre list).
    pub front_tire_compound_index: u8,
    /// Rear tyre compound index.
    pub rear_tire_compound_index: u8,
    /// Fuel tank capacity in litres.
    pub fuel_capacity: f64,
    /// 1 when the front flap (DRS-equivalent) is activated, 0 otherwise.
    pub front_flap_activated: u8,
    /// 1 when the rear flap is activated, 0 otherwise.
    pub rear_flap_activated: u8,
    /// Rear flap legal status: 0 = illegal, 1 = legal but not activated, 2 = activated.
    pub rear_flap_legal_status: u8,
    /// Ignition/starter state: 0 = off, 1 = ignition only, 2 = starter engaged.
    pub ignition_starter: u8,
    /// Front tyre compound name (ASCII, null-terminated, max 18 bytes).
    pub front_tire_compound_name: [u8; 18],
    /// Rear tyre compound name (ASCII, null-terminated, max 18 bytes).
    pub rear_tire_compound_name: [u8; 18],
    /// 1 when a pit-lane speed limiter is available on this vehicle, 0 otherwise.
    pub speed_limiter_available: u8,
    /// 1 when the anti-stall system is currently preventing the engine from stalling, 0 otherwise.
    pub anti_stall_activated: u8,
    #[doc(hidden)]
    pub _unused: [u8; 2],
    /// Steering wheel visual rotation range in degrees (for cockpit animation).
    pub visual_steering_wheel_range: f32,
    /// Rear brake bias as a fraction of total braking applied to the rear axle, 0.0–1.0.
    pub rear_brake_bias: f64,
    /// Turbo/supercharger boost pressure in kPa above atmospheric.
    pub turbo_boost_pressure: f64,
    /// Offset from physics centre to graphics centre (x, y, z) in metres.
    pub physics_to_graphics_offset: [f32; 3],
    /// Physical (hardware) steering wheel rotation range in degrees.
    pub physical_steering_wheel_range: f32,
    /// Per-wheel physics data: [front-left, front-right, rear-left, rear-right].
    pub wheels: [rF2Wheel; 4],
    #[doc(hidden)]
    pub _expansion: [u8; 152],
}

/// Header preceding the vehicle-telemetry array in shared memory.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct rF2TelemetryHeader {
    /// Version counter written before the update begins; compare with `version_update_end` to detect torn reads.
    pub version_update_begin: u32,
    /// Version counter written after the update completes; equals `version_update_begin` when data is consistent.
    pub version_update_end: u32,
    /// Size of this header struct in bytes.
    pub bytes_in_version: i32,
    /// Size of this header in bytes.
    pub bytes_in_header: i32,
    /// Size of a single `rF2VehicleTelemetry` entry in bytes.
    pub bytes_in_vehicle_telemetry: i32,
    /// Number of valid vehicle entries in the `rF2Telemetry::vehicles` array.
    pub num_vehicles: i32,
}

/// Top-level telemetry region: header + fixed-size array of vehicle telemetry.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct rF2Telemetry {
    /// Version and size metadata; `num_vehicles` gives the count of valid entries.
    pub header: rF2TelemetryHeader,
    /// Per-vehicle telemetry data; only `vehicles[0..header.num_vehicles]` are valid.
    pub vehicles: [rF2VehicleTelemetry; RF2_MAX_VEHICLES],
}

impl Default for rF2Telemetry {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for rF2VehicleTelemetry {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Per-vehicle scoring data: position, lap times, sector splits, pit state, and flags.
/// Updated at ~2 Hz. Only valid for vehicles in `rF2Scoring::vehicles[..header.num_vehicles]`.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct rF2VehicleScoring {
    /// Unique vehicle ID — matches `rF2VehicleTelemetry::id` for cross-referencing.
    pub id: i32,
    /// Driver name (ASCII, null-terminated, max 32 bytes).
    pub driver_name: [u8; 32],
    /// Vehicle model name (ASCII, null-terminated, max 64 bytes).
    pub vehicle_name: [u8; 64],
    /// Total completed laps.
    pub total_laps: i16,
    /// Current sector: 0 = between S3 finish and S1, 1 = sector 1, 2 = sector 2, 3 = sector 3.
    pub sector: i8,
    /// Finish status: 0 = none, 1 = finished, 2 = DNF, 3 = DQ, 4 = DNS.
    pub finish_status: i8,
    /// Distance along the track centreline in metres from the start line.
    pub lap_dist: f64,
    /// Lateral distance from the track centreline in metres; positive = right side.
    pub path_lateral: f64,
    /// Distance from the track edge in metres; negative = outside track limits.
    pub track_edge: f64,
    /// Personal best time for sector 1 this session in seconds; -1 = no time set.
    pub best_sector1: f64,
    /// Personal best time for sectors 1+2 combined this session in seconds; -1 = none.
    pub best_sector2: f64,
    /// Personal best lap time this session in seconds; -1 = no lap completed.
    pub best_lap_time: f64,
    /// Last completed sector 1 time in seconds.
    pub last_sector1: f64,
    /// Last completed sectors 1+2 time in seconds.
    pub last_sector2: f64,
    /// Last completed lap time in seconds.
    pub last_lap_time: f64,
    /// Current sector 1 time in progress in seconds; 0 if not in sector 1.
    pub cur_sector1: f64,
    /// Current sectors 1+2 time in progress in seconds; 0 if not in sector 2.
    pub cur_sector2: f64,
    /// Number of pit stops taken so far.
    pub num_pitstops: i16,
    /// Number of active penalties (drive-through, stop-go, etc.).
    pub num_penalties: i16,
    /// 1 when this entry is the local player's car, 0 for AI or remote drivers.
    pub is_player: u8,
    /// Control type: -1 = nobody, 0 = player, 1 = local AI, 2 = remote, 3 = replay.
    pub control: i8,
    /// 1 when the car is currently in the pit lane or pit box, 0 otherwise.
    pub in_pits: u8,
    /// Current race position (1 = leader).
    pub place: u8,
    /// Vehicle class/category name (ASCII, null-terminated, max 32 bytes).
    pub vehicle_class: [u8; 32],
    /// Time gap to the car ahead (next position) in seconds.
    pub time_behind_next: f64,
    /// Laps behind the car ahead (next position).
    pub laps_behind_next: i32,
    /// Time gap to the race leader in seconds.
    pub time_behind_leader: f64,
    /// Laps behind the race leader.
    pub laps_behind_leader: i32,
    /// Elapsed session time at the start of the current lap in seconds.
    pub lap_start_et: f64,
    /// World-space position (x, y, z) in metres.
    pub pos: [f64; 3],
    /// Velocity in the vehicle's local frame (x, y, z) in m/s.
    pub local_vel: [f64; 3],
    /// Acceleration in the vehicle's local frame (x, y, z) in m/s².
    pub local_accel: [f64; 3],
    /// 3×3 orientation matrix (right, up, forward axes in world space).
    pub ori: [[f64; 3]; 3],
    /// Angular velocity in the vehicle's local frame (x, y, z) in rad/s.
    pub local_rot: [f64; 3],
    /// Angular acceleration in the vehicle's local frame (x, y, z) in rad/s².
    pub local_rot_accel: [f64; 3],
    /// Headlight state: 0 = off, 1 = on.
    pub headlights: u8,
    /// Pit state: 0 = none, 1 = requesting, 2 = entering, 3 = stationary, 4 = exiting.
    pub pit_state: u8,
    /// 1 when this vehicle's score is counted by the server, 0 otherwise.
    pub server_scored: u8,
    /// Individual race phase for this vehicle (sim-defined enumeration).
    pub individual_phase: u8,
    /// Qualification order (grid position for race start); -1 = unset.
    pub qualification: i32,
    /// Elapsed time since the start of the current lap in seconds.
    pub time_into_lap: f64,
    /// Estimated lap time based on current performance in seconds.
    pub estimated_lap_time: f64,
    /// Pit group assignment string (ASCII, null-terminated, max 24 bytes).
    pub pit_group: [u8; 24],
    /// Current flag shown to this vehicle: 0 = none, 1 = blue, 2 = yellow, 3 = black, etc.
    pub flag: u8,
    /// 1 when this vehicle is currently under a local yellow flag, 0 otherwise.
    pub under_yellow: u8,
    /// Bitfield of flags counting as invalid lap markers (sim-defined).
    pub count_laps_invalid_flags: u8,
    /// 1 when the car is parked in the garage stall, 0 otherwise.
    pub in_garage_stall: u8,
    /// Upgrade pack identifier string (ASCII, null-terminated, max 16 bytes).
    pub upgrade_pack: [u8; 16],
    /// Distance along the pit lane in metres from the pit entry.
    pub pit_lap_dist: f32,
    /// Best lap's sector 1 time in seconds (from the fastest lap, not the fastest sector).
    pub best_lap_sector1: f32,
    /// Best lap's sectors 1+2 time in seconds.
    pub best_lap_sector2: f32,
    #[doc(hidden)]
    pub _expansion: [u8; 48],
}

/// Session-wide scoring info: track, weather, session type, and flag state.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct rF2ScoringInfo {
    /// Track name (ASCII, null-terminated, max 64 bytes).
    pub track_name: [u8; 64],
    /// Session type: 0 = test day, 1 = practice, 2 = qualify, 3 = warm-up, 4 = race.
    pub session: i32,
    /// Current elapsed session time in seconds.
    pub current_et: f64,
    /// Session end time in seconds; -1 for lap-based races.
    pub end_et: f64,
    /// Maximum number of laps; -1 for timed sessions.
    pub max_laps: i32,
    /// Total lap distance in metres (track length).
    pub lap_dist: f64,
    /// Results file name (ASCII, null-terminated, max 64 bytes).
    pub result_name: [u8; 64],
    /// Number of vehicles currently in the session.
    pub num_vehicles: i32,
    /// Game phase: 0 = before session, 1 = reconnaissance laps, 2 = grid walk, 3 = formation lap, 4 = starting, 5 = green, 6 = full course yellow, 7 = session stopped, 8 = session over.
    pub game_phase: u8,
    /// Yellow flag state: -1 = none, 0 = pending, 1 = pits closed, 2 = pit lead lap, 3 = pits open, 4 = last lap, 5 = resume, 6 = race halt.
    pub yellow_flag_state: i8,
    /// Per-sector flag state [S1, S2, S3]: -1 = none, 1 = yellow, 2 = double yellow.
    pub sector_flag: [i8; 3],
    /// Number of start lights currently lit (0 = all off/go).
    pub start_light: u8,
    /// Number of red lights in the start sequence.
    pub num_red_lights: u8,
    /// 1 when in real-time (non-replay) mode, 0 during replay.
    pub in_realtime: u8,
    /// Player driver name (ASCII, null-terminated, max 32 bytes).
    pub player_name: [u8; 32],
    /// Player's PLR (profile) file name (ASCII, null-terminated, max 64 bytes).
    pub plr_file_name: [u8; 64],
    /// Cloud cover fraction, 0.0 (clear) – 1.0 (overcast).
    pub dark_cloud: f64,
    /// Rainfall intensity, 0.0 (dry) – 1.0 (maximum rain).
    pub raining: f64,
    /// Ambient air temperature in °C.
    pub ambient_temp: f64,
    /// Track surface temperature in °C.
    pub track_temp: f64,
    /// Wind vector (x, y, z) in m/s in world space.
    pub wind: [f64; 3],
    /// Minimum track wetness fraction, 0.0 (dry) – 1.0 (fully wet).
    pub min_path_wetness: f64,
    /// Maximum track wetness fraction, 0.0–1.0.
    pub max_path_wetness: f64,
    /// Game mode: 0 = offline, 1 = online.
    pub game_mode: u8,
    /// 1 when the server requires a password to join, 0 otherwise.
    pub is_password_protected: u8,
    /// Server UDP port number.
    pub server_port: u16,
    /// Server public IP address as a 32-bit integer (network byte order).
    pub server_public_ip: u32,
    /// Maximum number of players allowed on the server.
    pub max_players: i32,
    /// Server name (ASCII, null-terminated, max 32 bytes).
    pub server_name: [u8; 32],
    /// Session start time (elapsed since sim launch) in seconds.
    pub start_et: f32,
    /// Average track wetness fraction, 0.0–1.0.
    pub avg_path_wetness: f64,
    #[doc(hidden)]
    pub _expansion: [u8; 200],
}

impl Default for rF2ScoringInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Header preceding the scoring-info and vehicle-scoring array.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct rF2ScoringHeader {
    /// Version counter written before the update begins; compare with `version_update_end` to detect torn reads.
    pub version_update_begin: u32,
    /// Version counter written after the update completes; equals `version_update_begin` when consistent.
    pub version_update_end: u32,
    /// Size of this header struct in bytes.
    pub bytes_in_version: i32,
    /// Size of this header in bytes.
    pub bytes_in_header: i32,
    /// Size of `rF2ScoringInfo` in bytes.
    pub bytes_in_scoring_info: i32,
    /// Size of a single `rF2VehicleScoring` entry in bytes.
    pub bytes_in_vehicle_scoring: i32,
    /// Number of valid vehicle entries in the `rF2Scoring::vehicles` array.
    pub num_vehicles: i32,
}

/// Top-level scoring region: header + session info + vehicle-scoring array.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct rF2Scoring {
    /// Version and size metadata; `num_vehicles` gives the count of valid entries.
    pub header: rF2ScoringHeader,
    /// Session-wide data: track name, weather, session type, flag state.
    pub scoring_info: rF2ScoringInfo,
    /// Per-vehicle scoring data; only `vehicles[0..header.num_vehicles]` are valid.
    pub vehicles: [rF2VehicleScoring; RF2_MAX_VEHICLES],
}

impl Default for rF2Scoring {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for rF2VehicleScoring {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Extended plugin/session metadata: plugin status, session started flag, and physics timing.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct rF2Extended {
    /// Version counter written at the start of each shared-memory update cycle.
    pub version_update_begin: u32,
    /// Version counter written at the end of each update cycle; equals `version_update_begin` when consistent.
    pub version_update_end: u32,
    /// Size of the version header in bytes (for forward-compatibility checks).
    pub bytes_in_version: i32,
    /// Total size of this struct in bytes as reported by the plugin.
    pub bytes_in_extended: i32,
    /// Offset from the physics centre of mass to the graphics origin (x, y, z) in metres.
    pub physics_to_graphics_offset: [f32; 3],
    /// 1 when the rF2/LMU shared-memory plugin is loaded and active, 0 otherwise.
    pub is_plugin_enabled: u8,
    /// 1 when direct memory access mode is enabled in the plugin, 0 otherwise.
    pub direct_memory_access_enabled: u8,
    #[doc(hidden)]
    pub _padding: [u8; 2],
    /// 1 when a session has started (past the loading screen), 0 otherwise.
    /// Used by `LmuConnection::is_connected()` to confirm live data.
    pub session_started: u8,
    /// Average physics thread execution time in milliseconds (performance metric).
    pub phys_avg_thread_time_ms: f64,
    #[doc(hidden)]
    pub _expansion: [u8; 508],
}

impl Default for rF2Extended {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

