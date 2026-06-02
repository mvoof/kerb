//! `#[repr(C, packed)]` structs that mirror Assetto Corsa's and AC Evo's shared
//! memory layouts byte-for-byte. Field order and sizes must match the game
//! definitions exactly — do not reorder or add padding.

use kerb_derive::Snapshot;

pub const AC_STATUS_OFF: i32 = 0;
pub const AC_STATUS_REPLAY: i32 = 1;
pub const AC_STATUS_LIVE: i32 = 2;
pub const AC_STATUS_PAUSE: i32 = 3;

// ── Assetto Corsa (classic) ───────────────────────────────────────────────────

/// Physics telemetry page — updated every simulation tick.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SPageFilePhysics {
    /// Monotonically increasing counter; increments each physics tick.
    pub packet_id: i32,
    /// Throttle pedal position, 0.0 (released) – 1.0 (full).
    pub gas: f32,
    /// Brake pedal position, 0.0 (released) – 1.0 (full).
    pub brake: f32,
    /// Remaining fuel in litres.
    pub fuel: f32,
    /// Current gear: -1 = reverse, 0 = neutral, 1–7 = forward gears.
    pub gear: i32,
    /// Engine speed in RPM.
    pub rpms: i32,
    /// Steering wheel angle in degrees; positive = right.
    pub steer_angle: f32,
    /// Vehicle speed in km/h.
    pub speed_kmh: f32,
    /// Velocity vector in world space (x, y, z), m/s.
    pub velocity: [f32; 3],
    /// Lateral, vertical, and longitudinal G-forces (x, y, z).
    pub acc_g: [f32; 3],
    /// Wheel slip ratio per tyre: [FL, FR, RL, RR].
    pub wheel_slip: [f32; 4],
    /// Vertical load per tyre in Newtons: [FL, FR, RL, RR].
    pub wheel_load: [f32; 4],
    /// Tyre pressure in PSI per tyre: [FL, FR, RL, RR].
    pub wheels_pressure: [f32; 4],
    /// Angular velocity of each wheel in rad/s: [FL, FR, RL, RR].
    pub wheel_angular_speed: [f32; 4],
    /// Tyre wear level 0.0–1.0 per tyre: [FL, FR, RL, RR].
    pub tyre_wear: [f32; 4],
    /// Tyre dirt contamination level 0.0–1.0 per tyre: [FL, FR, RL, RR].
    pub tyre_dirty_level: [f32; 4],
    /// Tyre core temperature in °C per tyre: [FL, FR, RL, RR].
    pub tyre_core_temperature: [f32; 4],
    /// Camber angle in radians per tyre: [FL, FR, RL, RR]. Negative = leaning in.
    pub camber_rad: [f32; 4],
    /// Suspension travel in metres per corner: [FL, FR, RL, RR].
    pub suspension_travel: [f32; 4],
    /// DRS state: 1.0 = open, 0.0 = closed.
    pub drs: f32,
    /// Traction-control intervention level, 0.0–1.0.
    pub tc: f32,
    /// Car heading in radians relative to north; range −π to π.
    pub heading: f32,
    /// Pitch angle in radians; positive = nose up.
    pub pitch: f32,
    /// Roll angle in radians; positive = right side down.
    pub roll: f32,
    /// Centre-of-gravity height above ground in metres.
    pub cg_height: f32,
    /// Damage levels for [front, rear, left, right, centre], 0.0–1.0.
    pub car_damage: [f32; 5],
    /// Number of wheels currently off the track surface.
    pub number_of_tyres_out: i32,
    /// 1 when pit-lane speed limiter is active, 0 otherwise.
    pub pit_limiter_on: i32,
    /// ABS intervention level, 0.0–1.0.
    pub abs: f32,
    /// KERS battery charge level, 0.0–1.0.
    pub kers_charge: f32,
    /// KERS deployment input, 0.0–1.0.
    pub kers_input: f32,
    /// 1 when auto-shifter is active, 0 otherwise.
    pub auto_shifter_on: i32,
    /// Front and rear ride heights in metres: [front, rear].
    pub ride_height: [f32; 2],
    /// Turbocharger boost pressure in bar above atmospheric.
    pub turbo_boost: f32,
    /// Ballast mass added to the car in kg.
    pub ballast: f32,
    /// Ambient air density in kg/m³.
    pub air_density: f32,
    /// Ambient air temperature in °C.
    pub air_temp: f32,
    /// Track surface temperature in °C.
    pub road_temp: f32,
    /// Local angular velocity vector (x, y, z) in rad/s.
    pub local_angular_vel: [f32; 3],
    /// Final force-feedback torque output, Nm.
    pub final_ff: f32,
    /// Performance delta relative to best lap; negative = faster.
    pub performance_meter: f32,
    /// Engine brake setting index (sim-defined range).
    pub engine_brake: i32,
    /// ERS recovery level setting (0 = off, higher = more recovery).
    pub ers_recovery_level: i32,
    /// ERS power deployment level (0 = off, higher = more deployment).
    pub ers_power_level: i32,
    /// 1 when ERS is in heat-charging mode, 0 otherwise.
    pub ers_heat_charging: i32,
    /// 1 when ERS is actively charging, 0 otherwise.
    pub ers_is_charging: i32,
    /// Current KERS energy stored in kJ.
    pub kers_current_kj: f32,
    /// 1 when DRS is available (zone + activation conditions met), 0 otherwise.
    pub drs_available: i32,
    /// 1 when DRS is currently open, 0 otherwise.
    pub drs_enabled: i32,
    /// Brake disc temperature in °C per corner: [FL, FR, RL, RR].
    pub brake_temp: [f32; 4],
    /// Clutch pedal position, 0.0 (disengaged) – 1.0 (engaged).
    pub clutch: f32,
    /// Tyre inner surface temperature in °C: [FL, FR, RL, RR].
    pub tyre_temp_i: [f32; 4],
    /// Tyre middle surface temperature in °C: [FL, FR, RL, RR].
    pub tyre_temp_m: [f32; 4],
    /// Tyre outer surface temperature in °C: [FL, FR, RL, RR].
    pub tyre_temp_o: [f32; 4],
    /// 1 when the car is driven by the AI, 0 for human player.
    pub is_ai_controlled: i32,
    /// World-space contact point of each tyre (x, y, z) in metres: [FL, FR, RL, RR].
    pub tyre_contact_point: [[f32; 3]; 4],
    /// Surface normal at each tyre contact point (unit vector): [FL, FR, RL, RR].
    pub tyre_contact_normal: [[f32; 3]; 4],
    /// Heading vector at each tyre contact point (unit vector): [FL, FR, RL, RR].
    pub tyre_contact_heading: [[f32; 3]; 4],
    /// Brake bias as a fraction of total braking applied to front axle, 0.0–1.0.
    pub brake_bias: f32,
    /// Velocity in the car's local coordinate frame (x, y, z), m/s.
    pub local_velocity: [f32; 3],
}

/// Graphics/HUD page — updated each rendered frame.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SPageFileGraphics {
    /// Monotonically increasing counter; increments each graphics frame.
    pub packet_id: i32,
    /// Session status: 0 = off, 1 = replay, 2 = live, 3 = paused. Use `AC_STATUS_*` constants.
    pub status: i32,
    /// Session type index: 0 = practice, 1 = qualify, 2 = race.
    pub session: i32,
    /// Current lap time as a UTF-16 string (mm:ss.mmm), null-terminated, max 15 chars.
    pub current_time: [u16; 15],
    /// Last completed lap time as a UTF-16 string (mm:ss.mmm), null-terminated.
    pub last_time: [u16; 15],
    /// Session best lap time as a UTF-16 string (mm:ss.mmm), null-terminated.
    pub best_time: [u16; 15],
    /// Current sector split time as a UTF-16 string, null-terminated.
    pub split: [u16; 15],
    /// Number of completed laps.
    pub completed_laps: i32,
    /// Current race position (1 = leader).
    pub position: i32,
    /// Current lap time in milliseconds.
    pub i_current_time: i32,
    /// Last completed lap time in milliseconds.
    pub i_last_time: i32,
    /// Session best lap time in milliseconds.
    pub i_best_time: i32,
    /// Remaining session time in seconds; -1 if laps-based race.
    pub session_time_left: f32,
    /// Total distance travelled this session in metres.
    pub distance_traveled: f32,
    /// 1 when the car is inside the pit box, 0 otherwise.
    pub is_in_pit: i32,
    /// Index of the current track sector (0-based).
    pub current_sector_index: i32,
    /// Last sector time in milliseconds.
    pub last_sector_time: i32,
    /// Total number of laps in the race; -1 for timed races.
    pub number_of_laps: i32,
    /// Active tyre compound name as UTF-16, null-terminated, max 33 chars.
    pub tyre_compound: [u16; 33],
    /// Replay playback speed multiplier (1.0 = normal).
    pub replay_time_multiplier: f32,
    /// Car position on track spline, 0.0 (start) – 1.0 (finish line).
    pub normalized_car_position: f32,
    /// Number of cars currently active on track.
    pub active_cars: i32,
    /// World-space coordinates (x, y, z) for up to 60 cars.
    pub car_coordinates: [[f32; 3]; 60],
    /// Sim-internal IDs for up to 60 cars (maps to `player_car_id`).
    pub car_id: [i32; 60],
    /// Sim-internal ID of the player's car (matches an entry in `car_id`).
    pub player_car_id: i32,
    /// Drive-through or stop-go penalty time remaining in seconds.
    pub penalty_time: f32,
    /// Current flag being shown: 0 = none, 1 = blue, 2 = yellow, 3 = black, etc.
    pub flag: i32,
    /// Active penalty type index (sim-defined enumeration).
    pub penalty: i32,
    /// 1 when the ideal-line driving aid is enabled, 0 otherwise.
    pub ideal_line_on: i32,
    /// 1 when the car is in the pit lane (not necessarily in the pit box), 0 otherwise.
    pub is_in_pit_lane: i32,
    /// Track surface grip coefficient, 0.0–1.0.
    pub surface_grip: f32,
    /// 1 when the mandatory pit stop has been served, 0 otherwise.
    pub mandatory_pit_done: i32,
    /// Wind speed in m/s.
    pub wind_speed: f32,
    /// Wind direction in radians relative to north.
    pub wind_direction: f32,
    /// 1 when the setup menu is currently visible, 0 otherwise.
    pub is_setup_menu_visible: i32,
    /// Index of the currently active main MFD page.
    pub main_display_index: i32,
    /// Index of the currently active secondary MFD page.
    pub secondary_display_index: i32,
    /// Active TC level setting (sim-defined integer).
    pub tc: i32,
    /// Active TC cut level setting (sim-defined integer).
    pub tc_cut: i32,
    /// Engine map setting index.
    pub engine_map: i32,
    /// Active ABS level setting (sim-defined integer).
    pub abs: i32,
    /// Estimated fuel consumption per lap in litres.
    pub fuel_xlap: f32,
    /// 1 when rain lights are on, 0 otherwise.
    pub rain_lights: i32,
    /// 1 when hazard/flashing lights are active, 0 otherwise.
    pub flashing_lights: i32,
    /// Headlight state index: 0 = off, higher = on/high-beam.
    pub lights_stage: i32,
    /// Exhaust gas temperature in °C.
    pub exhaust_temperature: f32,
    /// Windscreen wiper level (0 = off, higher = faster).
    pub wiper_lv: i32,
    /// Total driver stint time remaining in seconds (championship rules).
    pub driver_stint_total_time_left: i32,
    /// Current stint time remaining in seconds.
    pub driver_stint_time_left: i32,
    /// 1 when rain tyres are fitted, 0 for slicks.
    pub rain_tyres: i32,
}

/// Static data page — written once when a session loads; does not update during the session.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SPageFileStatic {
    /// Shared-memory plugin version as UTF-16, null-terminated.
    pub sm_version: [u16; 15],
    /// Assetto Corsa game version as UTF-16, null-terminated.
    pub ac_version: [u16; 15],
    /// Total number of sessions in the event (practice + qualify + race, etc.).
    pub number_of_sessions: i32,
    /// Number of cars in the race.
    pub num_cars: i32,
    /// Player's car model identifier as UTF-16, null-terminated (e.g. "ferrari_458").
    pub car_model: [u16; 33],
    /// Track identifier as UTF-16, null-terminated (e.g. "monza").
    pub track: [u16; 33],
    /// Player's first name as UTF-16, null-terminated.
    pub player_name: [u16; 33],
    /// Player's surname as UTF-16, null-terminated.
    pub player_surname: [u16; 33],
    /// Player's nickname/tag as UTF-16, null-terminated.
    pub player_nick: [u16; 33],
    /// Number of track sectors.
    pub sector_count: i32,
    /// Maximum engine torque in Nm.
    pub max_torque: f32,
    /// Maximum engine power in W.
    pub max_power: f32,
    /// Rev limiter RPM.
    pub max_rpm: i32,
    /// Fuel tank capacity in litres.
    pub max_fuel: f32,
    /// Maximum suspension travel per corner in metres: [FL, FR, RL, RR].
    pub suspension_max_travel: [f32; 4],
    /// Tyre radius per corner in metres: [FL, FR, RL, RR].
    pub tyre_radius: [f32; 4],
    /// Maximum turbo boost pressure in bar.
    pub max_turbo_boost: f32,
    #[doc(hidden)]
    pub _deprecated_1: f32,
    #[doc(hidden)]
    pub _deprecated_2: f32,
    /// 1 when track penalties (cut penalties, etc.) are enforced, 0 otherwise.
    pub penalties_enabled: i32,
    /// Fuel-consumption aid multiplier (1.0 = realistic, lower = reduced consumption).
    pub aid_fuel_rate: f32,
    /// Tyre-wear aid multiplier (1.0 = realistic, lower = reduced wear).
    pub aid_tire_rate: f32,
    /// Mechanical-damage aid multiplier (1.0 = realistic, 0.0 = no damage).
    pub aid_mechanical_damage: f32,
    /// 1 when tyre blankets (pre-heat) are allowed, 0 otherwise.
    pub aid_allow_tyre_blankets: i32,
    /// Stability-control aid level, 0.0 (off) – 1.0 (maximum).
    pub aid_stability: f32,
    /// 1 when auto-clutch aid is enabled, 0 otherwise.
    pub aid_auto_clutch: i32,
    /// 1 when auto-blip (throttle on downshift) aid is enabled, 0 otherwise.
    pub aid_auto_blip: i32,
    /// 1 when the car has DRS capability, 0 otherwise.
    pub has_drs: i32,
    /// 1 when the car has ERS (Energy Recovery System), 0 otherwise.
    pub has_ers: i32,
    /// 1 when the car has KERS, 0 otherwise.
    pub has_kers: i32,
    /// Maximum KERS energy capacity in J.
    pub kers_max_j: f32,
    /// Number of available engine-brake settings.
    pub engine_brake_settings_count: i32,
    /// Number of available ERS power-controller modes.
    pub ers_power_controller_count: i32,
    /// Length of the track spline in metres (used to normalise car position).
    pub track_spline_length: f32,
    /// Track layout/configuration variant as UTF-16, null-terminated.
    pub track_configuration: [u16; 33],
    /// Maximum ERS energy capacity in J.
    pub ers_max_j: f32,
    /// 1 when the race is timed (rather than lap-based), 0 otherwise.
    pub is_timed_race: i32,
    /// 1 when an extra lap is added after time expires (Indianapolis-style), 0 otherwise.
    pub has_extra_lap: i32,
    /// Player's car livery/skin name as UTF-16, null-terminated.
    pub car_skin: [u16; 33],
    /// Number of grid positions reversed at the start.
    pub reversed_grid_positions: i32,
    /// Lap number at which the pit window opens (-1 = no window).
    pub pit_window_start: i32,
    /// Lap number at which the pit window closes (-1 = no window).
    pub pit_window_end: i32,
    /// 1 when this is an online (multiplayer) session, 0 for offline.
    pub is_online: i32,
}

impl Default for SPageFilePhysics {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for SPageFileGraphics {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for SPageFileStatic {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// ── Assetto Corsa Evo ─────────────────────────────────────────────────────────

/// Per-tyre state embedded in the AC Evo graphics page.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SMEvoTyreState {
    /// Tyre compound symbol as UTF-16, null-terminated (e.g. "S", "M", "H").
    pub tyre_compound_symbol: [u16; 33],
    /// Full tyre compound name as UTF-16, null-terminated (e.g. "Pirelli Soft").
    pub tyre_compound_name: [u16; 50],
    /// Short tyre compound abbreviation as UTF-16, null-terminated.
    pub tyre_compound_short: [u16; 5],
    /// Tyre compound colour name as UTF-16, null-terminated (e.g. "Red").
    pub tyre_compound_color: [u16; 10],
    /// 1 when a wet-weather tyre compound is fitted, 0 for slicks.
    pub is_wet_tyre: i32,
    /// Tyre surface temperatures at [inner, middle, outer] positions in °C.
    pub tyre_surface_temp: [f32; 3],
    /// Tyre inner-liner temperatures at [inner, middle, outer] positions in °C.
    pub tyre_inner_temp: [f32; 3],
    /// Tyre core (carcass) temperature in °C.
    pub tyre_core_temp: f32,
    /// Tyre inflation pressure in PSI.
    pub tyre_pressure: f32,
    /// Longitudinal slip ratio (dimensionless).
    pub tyre_slip_ratio: f32,
    /// Lateral slip angle in radians.
    pub tyre_slip_angle: f32,
    /// Normalised combined slip (0.0 = no slip, 1.0+ = losing grip).
    pub tyre_nd_slip: f32,
    /// Vertical tyre load in Newtons.
    pub tyre_load: f32,
    /// Tyre dirt contamination level, 0.0 (clean) – 1.0 (fully dirty).
    pub tyre_dirt_level: f32,
    /// Tyre wear level, 0.0 (new) – 1.0 (fully worn).
    pub tyre_wear: f32,
    /// Wheel angular velocity in rad/s.
    pub tyre_angular_speed: f32,
    /// Suspension travel in metres.
    pub suspension_travel: f32,
    /// Camber angle in radians; negative = leaning inward.
    pub camber: f32,
    /// Wheel rim temperature in °C.
    pub rim_temp: f32,
    /// Brake disc temperature in °C.
    pub brake_disc_temp: f32,
    /// Brake disc wear level, 0.0 (new) – 1.0 (fully worn).
    pub brake_disc_wear: f32,
    /// Brake caliper temperature in °C.
    pub brake_temp: f32,
}

impl Default for SMEvoTyreState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Physics page for AC Evo — superset of classic AC physics.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SPageFilePhysicsEvo {
    /// Monotonically increasing counter; increments each physics tick.
    pub packet_id: i32,
    /// Throttle pedal position, 0.0 (released) – 1.0 (full).
    pub gas: f32,
    /// Brake pedal position, 0.0 (released) – 1.0 (full).
    pub brake: f32,
    /// Remaining fuel in litres.
    pub fuel: f32,
    /// Current gear: -1 = reverse, 0 = neutral, 1–7 = forward gears.
    pub gear: i32,
    /// Engine speed in RPM.
    pub rpms: i32,
    /// Steering wheel angle in degrees; positive = right.
    pub steer_angle: f32,
    /// Vehicle speed in km/h.
    pub speed_kmh: f32,
    /// Velocity vector in world space (x, y, z), m/s.
    pub velocity: [f32; 3],
    /// Lateral, vertical, and longitudinal G-forces (x, y, z).
    pub acc_g: [f32; 3],
    /// Wheel slip ratio per tyre: [FL, FR, RL, RR].
    pub wheel_slip: [f32; 4],
    /// Vertical load per tyre in Newtons: [FL, FR, RL, RR].
    pub wheel_load: [f32; 4],
    /// Tyre pressure in PSI per tyre: [FL, FR, RL, RR].
    pub wheels_pressure: [f32; 4],
    /// Angular velocity of each wheel in rad/s: [FL, FR, RL, RR].
    pub wheel_angular_speed: [f32; 4],
    /// Tyre wear level 0.0–1.0 per tyre: [FL, FR, RL, RR].
    pub tyre_wear: [f32; 4],
    /// Tyre dirt contamination level 0.0–1.0 per tyre: [FL, FR, RL, RR].
    pub tyre_dirty_level: [f32; 4],
    /// Tyre core temperature in °C per tyre: [FL, FR, RL, RR].
    pub tyre_core_temperature: [f32; 4],
    /// Camber angle in radians per tyre: [FL, FR, RL, RR]. Negative = leaning in.
    pub camber_rad: [f32; 4],
    /// Suspension travel in metres per corner: [FL, FR, RL, RR].
    pub suspension_travel: [f32; 4],
    /// DRS state: 1.0 = open, 0.0 = closed.
    pub drs: f32,
    /// Traction-control intervention level, 0.0–1.0.
    pub tc: f32,
    /// Car heading in radians relative to north; range −π to π.
    pub heading: f32,
    /// Pitch angle in radians; positive = nose up.
    pub pitch: f32,
    /// Roll angle in radians; positive = right side down.
    pub roll: f32,
    /// Centre-of-gravity height above ground in metres.
    pub cg_height: f32,
    /// Damage levels for [front, rear, left, right, centre], 0.0–1.0.
    pub car_damage: [f32; 5],
    /// Number of wheels currently off the track surface.
    pub number_of_tyres_out: i32,
    /// 1 when pit-lane speed limiter is active, 0 otherwise.
    pub pit_limiter_on: i32,
    /// ABS intervention level, 0.0–1.0.
    pub abs: f32,
    /// 1 when auto-shifter is active, 0 otherwise.
    pub auto_shifter_on: i32,
    /// Front and rear ride heights in metres: [front, rear].
    pub ride_height: [f32; 2],
    /// Turbocharger boost pressure in bar above atmospheric.
    pub turbo_boost: f32,
    /// Ballast mass added to the car in kg.
    pub ballast: f32,
    /// Ambient air density in kg/m³.
    pub air_density: f32,
    /// Ambient air temperature in °C.
    pub air_temp: f32,
    /// Track surface temperature in °C.
    pub road_temp: f32,
    /// Local angular velocity vector (x, y, z) in rad/s.
    pub local_angular_vel: [f32; 3],
    /// Final force-feedback torque output, Nm.
    pub final_ff: f32,
    /// Engine brake setting index (sim-defined range).
    pub engine_brake: i32,
    /// Brake disc temperature in °C per corner: [FL, FR, RL, RR].
    pub brake_temp: [f32; 4],
    /// Clutch pedal position, 0.0 (disengaged) – 1.0 (engaged).
    pub clutch: f32,
    /// Tyre inner surface temperature in °C: [FL, FR, RL, RR].
    pub tyre_temp_i: [f32; 4],
    /// Tyre middle surface temperature in °C: [FL, FR, RL, RR].
    pub tyre_temp_m: [f32; 4],
    /// Tyre outer surface temperature in °C: [FL, FR, RL, RR].
    pub tyre_temp_o: [f32; 4],
    /// 1 when the car is driven by the AI, 0 for human player.
    pub is_ai_controlled: i32,
    /// Brake bias as a fraction of total braking applied to front axle, 0.0–1.0.
    pub brake_bias: f32,
    /// Velocity in the car's local coordinate frame (x, y, z), m/s.
    pub local_velocity: [f32; 3],
    /// Front brake compound index: 0 = default/soft, higher = harder compounds.
    pub front_brake_compound: i32,
    /// Rear brake compound index: 0 = default/soft, higher = harder compounds.
    pub rear_brake_compound: i32,
    /// Brake pad remaining life per corner, 0.0 (gone) – 1.0 (new): [FL, FR, RL, RR].
    pub pad_life: [f32; 4],
    /// Brake disc remaining life per corner, 0.0 (gone) – 1.0 (new): [FL, FR, RL, RR].
    pub disc_life: [f32; 4],
    /// 1 when the ignition switch is on, 0 otherwise.
    pub ignition_on: i32,
    /// 1 when the starter motor is engaged, 0 otherwise.
    pub starter_engine_on: i32,
    /// 1 when the engine is running, 0 when stalled or off.
    pub is_engine_running: i32,
    /// Force-feedback vibration component from kerb impacts, 0.0–1.0.
    pub kerb_vibration: f32,
    /// Force-feedback vibration component from tyre slip, 0.0–1.0.
    pub slip_vibrations: f32,
    /// Force-feedback vibration component from G-forces, 0.0–1.0.
    pub g_vibrations: f32,
    /// Force-feedback vibration component from ABS pulses, 0.0–1.0.
    pub abs_vibrations: f32,
}

/// Graphics/HUD page for AC Evo — superset of classic AC graphics.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SPageFileGraphicsEvo {
    /// Monotonically increasing counter; increments each graphics frame.
    pub packet_id: i32,
    /// Session status: 0 = off, 1 = replay, 2 = live, 3 = paused. Use `AC_STATUS_*` constants.
    pub status: i32,
    /// Session type index: 0 = practice, 1 = qualify, 2 = race.
    pub session: i32,
    /// Current lap time as a UTF-16 string (mm:ss.mmm), null-terminated, max 15 chars.
    pub current_time: [u16; 15],
    /// Last completed lap time as a UTF-16 string (mm:ss.mmm), null-terminated.
    pub last_time: [u16; 15],
    /// Session best lap time as a UTF-16 string (mm:ss.mmm), null-terminated.
    pub best_time: [u16; 15],
    /// Current sector split time as a UTF-16 string, null-terminated.
    pub split: [u16; 15],
    /// Number of completed laps.
    pub completed_laps: i32,
    /// Current race position (1 = leader).
    pub position: i32,
    /// Current lap time in milliseconds.
    pub i_current_time: i32,
    /// Last completed lap time in milliseconds.
    pub i_last_time: i32,
    /// Session best lap time in milliseconds.
    pub i_best_time: i32,
    /// Remaining session time in seconds; -1 if laps-based race.
    pub session_time_left: f32,
    /// Total distance travelled this session in metres.
    pub distance_traveled: f32,
    /// 1 when the car is inside the pit box, 0 otherwise.
    pub is_in_pit: i32,
    /// Index of the current track sector (0-based).
    pub current_sector_index: i32,
    /// Last sector time in milliseconds.
    pub last_sector_time: i32,
    /// Total number of laps in the race; -1 for timed races.
    pub number_of_laps: i32,
    /// Active tyre compound name as UTF-16, null-terminated, max 33 chars.
    pub tyre_compound: [u16; 33],
    /// Car position on track spline, 0.0 (start) – 1.0 (finish line).
    pub normalized_car_position: f32,
    /// Number of cars currently active on track.
    pub active_cars: i32,
    /// World-space coordinates (x, y, z) for up to 60 cars.
    pub car_coordinates: [[f32; 3]; 60],
    /// Sim-internal IDs for up to 60 cars (maps to `player_car_id`).
    pub car_id: [i32; 60],
    /// Sim-internal ID of the player's car (matches an entry in `car_id`).
    pub player_car_id: i32,
    /// Drive-through or stop-go penalty time remaining in seconds.
    pub penalty_time: f32,
    /// Current flag being shown: 0 = none, 1 = blue, 2 = yellow, 3 = black, etc.
    pub flag: i32,
    /// Active penalty type index (sim-defined enumeration).
    pub penalty: i32,
    /// 1 when the ideal-line driving aid is enabled, 0 otherwise.
    pub ideal_line_on: i32,
    /// 1 when the car is in the pit lane (not necessarily in the pit box), 0 otherwise.
    pub is_in_pit_lane: i32,
    /// Track surface grip coefficient, 0.0–1.0.
    pub surface_grip: f32,
    /// 1 when the mandatory pit stop has been served, 0 otherwise.
    pub mandatory_pit_done: i32,
    /// Wind speed in m/s.
    pub wind_speed: f32,
    /// Wind direction in radians relative to north.
    pub wind_direction: f32,
    /// Active TC level setting (sim-defined integer).
    pub tc: i32,
    /// Active TC cut level setting (sim-defined integer).
    pub tc_cut: i32,
    /// Engine map setting index.
    pub engine_map: i32,
    /// Active ABS level setting (sim-defined integer).
    pub abs: i32,
    /// Estimated fuel consumption per lap in litres.
    pub fuel_xlap: f32,
    /// 1 when rain lights are on, 0 otherwise.
    pub rain_lights: i32,
    /// 1 when hazard/flashing lights are active, 0 otherwise.
    pub flashing_lights: i32,
    /// Headlight state index: 0 = off, higher = on/high-beam.
    pub lights_stage: i32,
    /// Exhaust gas temperature in °C.
    pub exhaust_temperature: f32,
    /// Windscreen wiper level (0 = off, higher = faster).
    pub wiper_lv: i32,
    /// Total driver stint time remaining in seconds (championship rules).
    pub driver_stint_total_time_left: i32,
    /// Current stint time remaining in seconds.
    pub driver_stint_time_left: i32,
    /// 1 when rain tyres are fitted, 0 for slicks.
    pub rain_tyres: i32,
    /// Session index within the event (0-based).
    pub session_index: i32,
    /// Fuel consumed so far this stint in litres.
    pub used_fuel: f32,
    /// Delta lap time vs. reference as UTF-16, null-terminated.
    pub delta_lap_time: [u16; 15],
    /// Delta lap time in milliseconds; negative = ahead of reference.
    pub i_delta_lap_time: i32,
    /// Estimated lap time based on current pace as UTF-16, null-terminated.
    pub estimated_lap_time: [u16; 15],
    /// Estimated lap time in milliseconds.
    pub i_estimated_lap_time: i32,
    /// 1 when delta is positive (slower than reference), 0 when negative.
    pub is_delta_positive: i32,
    /// Current sector split in milliseconds.
    pub i_split: i32,
    /// 1 when current lap is valid (no track-limits violations), 0 otherwise.
    pub is_valid_lap: i32,
    /// Estimated number of laps remaining on current fuel load.
    pub fuel_estimated_laps: f32,
    /// Track status description as UTF-16, null-terminated (e.g. "Green", "Fast").
    pub track_status: [u16; 33],
    /// Number of mandatory pit stops still to be served.
    pub missing_mandatory_pits: i32,
    /// Real-world clock time in seconds since midnight.
    pub clock: f32,
    /// 1 when left indicator/direction light is active, 0 otherwise.
    pub direction_lights_left: i32,
    /// 1 when right indicator/direction light is active, 0 otherwise.
    pub direction_lights_right: i32,
    /// 1 when a global yellow flag is displayed on the whole circuit, 0 otherwise.
    pub global_yellow: i32,
    /// 1 when yellow flag is active in sector 1, 0 otherwise.
    pub global_yellow_1: i32,
    /// 1 when yellow flag is active in sector 2, 0 otherwise.
    pub global_yellow_2: i32,
    /// 1 when yellow flag is active in sector 3, 0 otherwise.
    pub global_yellow_3: i32,
    /// 1 when a global white flag is shown, 0 otherwise.
    pub global_white: i32,
    /// 1 when a global green flag is shown, 0 otherwise.
    pub global_green: i32,
    /// 1 when the chequered flag is shown, 0 otherwise.
    pub global_chequered: i32,
    /// 1 when a global red flag is shown, 0 otherwise.
    pub global_red: i32,
    /// MFD selected tyre set index.
    pub mfd_tyre_set: i32,
    /// Fuel-to-add value set in the MFD pit strategy, in litres.
    pub mfd_fuel_to_add: f32,
    /// MFD target tyre pressure for left-front in PSI.
    pub mfd_tyre_pressure_lf: f32,
    /// MFD target tyre pressure for right-front in PSI.
    pub mfd_tyre_pressure_rf: f32,
    /// MFD target tyre pressure for left-rear in PSI.
    pub mfd_tyre_pressure_lr: f32,
    /// MFD target tyre pressure for right-rear in PSI.
    pub mfd_tyre_pressure_rr: f32,
    /// Track grip status index: 0 = green, 1 = fast, 2 = optimum, 3 = greasy, 4 = damp, 5 = wet, 6 = flooded.
    pub track_grip_status: i32,
    /// Current rain intensity: 0 = none, 1 = drizzle, 2 = light, 3 = medium, 4 = heavy, 5 = thunderstorm.
    pub rain_intensity: i32,
    /// Forecast rain intensity in 10 minutes (same scale as `rain_intensity`).
    pub rain_intensity_in_10min: i32,
    /// Forecast rain intensity in 30 minutes (same scale as `rain_intensity`).
    pub rain_intensity_in_30min: i32,
    /// Index of the currently fitted tyre set.
    pub current_tyre_set: i32,
    /// Index of the tyre set planned for the next pit stop strategy.
    pub strategy_tyre_set: i32,
    /// Per-tyre detailed state: [FL, FR, RL, RR].
    pub tyres: [SMEvoTyreState; 4],
}

/// Static data page for AC Evo — written once when the session loads; does not update during the session.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub(crate) struct SPageFileStaticEvo {
    /// Shared-memory plugin version as UTF-16, null-terminated.
    pub sm_version: [u16; 15],
    /// Game version as UTF-16, null-terminated.
    pub ac_version: [u16; 15],
    /// Total number of sessions in the event.
    pub number_of_sessions: i32,
    /// Number of cars in the race.
    pub num_cars: i32,
    /// Player's car model identifier as UTF-16, null-terminated.
    pub car_model: [u16; 33],
    /// Track identifier as UTF-16, null-terminated.
    pub track: [u16; 33],
    /// Player's first name as UTF-16, null-terminated.
    pub player_name: [u16; 33],
    /// Player's surname as UTF-16, null-terminated.
    pub player_surname: [u16; 33],
    /// Player's nickname/tag as UTF-16, null-terminated.
    pub player_nick: [u16; 33],
    /// Number of track sectors.
    pub sector_count: i32,
    /// Maximum engine torque in Nm.
    pub max_torque: f32,
    /// Maximum engine power in W.
    pub max_power: f32,
    /// Rev limiter RPM.
    pub max_rpm: i32,
    /// Fuel tank capacity in litres.
    pub max_fuel: f32,
    /// Maximum suspension travel per corner in metres: [FL, FR, RL, RR].
    pub suspension_max_travel: [f32; 4],
    /// Tyre radius per corner in metres: [FL, FR, RL, RR].
    pub tyre_radius: [f32; 4],
    /// Maximum turbo boost pressure in bar.
    pub max_turbo_boost: f32,
    /// 1 when track penalties (cut penalties, etc.) are enforced, 0 otherwise.
    pub penalties_enabled: i32,
    /// Fuel-consumption aid multiplier (1.0 = realistic, lower = reduced consumption).
    pub aid_fuel_rate: f32,
    /// Tyre-wear aid multiplier (1.0 = realistic, lower = reduced wear).
    pub aid_tire_rate: f32,
    /// Mechanical-damage aid multiplier (1.0 = realistic, 0.0 = no damage).
    pub aid_mechanical_damage: f32,
    /// 1 when tyre blankets are allowed, 0 otherwise.
    pub aid_allow_tyre_blankets: i32,
    /// Stability-control aid level, 0.0 (off) – 1.0 (maximum).
    pub aid_stability: f32,
    /// 1 when auto-clutch aid is enabled, 0 otherwise.
    pub aid_auto_clutch: i32,
    /// 1 when auto-blip (throttle on downshift) aid is enabled, 0 otherwise.
    pub aid_auto_blip: i32,
    /// Lap number at which the pit window opens (-1 = no window).
    pub pit_window_start: i32,
    /// Lap number at which the pit window closes (-1 = no window).
    pub pit_window_end: i32,
    /// 1 when this is an online (multiplayer) session, 0 for offline.
    pub is_online: i32,
    /// Dry-weather tyre compound name as UTF-16, null-terminated.
    pub dry_tyres_name: [u16; 33],
    /// Wet-weather tyre compound name as UTF-16, null-terminated.
    pub wet_tyres_name: [u16; 33],
}

impl Default for SPageFilePhysicsEvo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for SPageFileGraphicsEvo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for SPageFileStaticEvo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
