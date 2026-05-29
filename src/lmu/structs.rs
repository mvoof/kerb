//! `#[repr(C, packed)]` struct definitions that mirror the rFactor 2 shared-memory
//! plugin layout byte-for-byte. Field order and sizes must match the C definitions
//! in `rF2SharedMemoryPlugin` exactly — do not reorder or add padding.

use kerb_derive::Snapshot;

/// Maximum number of vehicles the shared-memory buffers can hold.
pub const RF2_MAX_VEHICLES: usize = 128;

/// Per-wheel physics data (suspension, tyre temps, wear, forces, etc.).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default, Snapshot)]
pub struct rF2Wheel {
    pub suspension_deflection: f64,
    pub ride_height: f64,
    pub susp_force: f64,
    pub brake_temp: f64,
    pub brake_pressure: f64,
    pub rotation: f64,
    pub lateral_patch_vel: f64,
    pub longitudinal_patch_vel: f64,
    pub lateral_ground_vel: f64,
    pub longitudinal_ground_vel: f64,
    pub camber: f64,
    pub lateral_force: f64,
    pub longitudinal_force: f64,
    pub tire_load: f64,
    pub grip_fract: f64,
    pub pressure: f64,
    pub temperature: [f64; 3],
    pub wear: f64,
    pub terrain_name: [u8; 16],
    pub surface_type: u8,
    pub flat: u8,
    pub detached: u8,
    pub static_unbalance: u8,
    pub vertical_tire_deflection: f64,
    pub wheel_ylocation: f64,
    pub toe: f64,
    pub tire_carcass_temperature: f64,
    pub tire_inner_layer_temperature: [f64; 3],
    pub _expansion: [u8; 24],
}

/// Per-vehicle telemetry: engine, inputs, wheels, position and orientation.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub struct rF2VehicleTelemetry {
    pub id: i32,
    pub delta_time: f64,
    pub elapsed_time: f64,
    pub lap_number: i32,
    pub lap_start_et: f64,
    pub vehicle_name: [u8; 64],
    pub track_name: [u8; 64],
    pub pos: [f64; 3],
    pub local_vel: [f64; 3],
    pub local_accel: [f64; 3],
    pub ori: [[f64; 3]; 3],
    pub local_rot: [f64; 3],
    pub local_rot_accel: [f64; 3],
    pub gear: i32,
    pub engine_rpm: f64,
    pub engine_water_temp: f64,
    pub engine_oil_temp: f64,
    pub clutch_rpm: f64,
    pub unfiltered_throttle: f64,
    pub unfiltered_brake: f64,
    pub unfiltered_steering: f64,
    pub unfiltered_clutch: f64,
    pub filtered_throttle: f64,
    pub filtered_brake: f64,
    pub filtered_steering: f64,
    pub filtered_clutch: f64,
    pub steering_shaft_torque: f64,
    pub front3rd_deflection: f64,
    pub rear3rd_deflection: f64,
    pub front_wing_height: f64,
    pub front_ride_height: f64,
    pub rear_ride_height: f64,
    pub drag: f64,
    pub front_downforce: f64,
    pub rear_downforce: f64,
    pub fuel: f64,
    pub engine_max_rpm: f64,
    pub scheduled_stops: u8,
    pub overheating: u8,
    pub detached: u8,
    pub headlights: u8,
    pub dent_severity: [u8; 8],
    pub last_impact_et: f64,
    pub last_impact_magnitude: f64,
    pub last_impact_pos: [f64; 3],
    pub engine_torque: f64,
    pub current_sector: i32,
    pub speed_limiter: u8,
    pub max_gears: u8,
    pub front_tire_compound_index: u8,
    pub rear_tire_compound_index: u8,
    pub fuel_capacity: f64,
    pub front_flap_activated: u8,
    pub rear_flap_activated: u8,
    pub rear_flap_legal_status: u8,
    pub ignition_starter: u8,
    pub front_tire_compound_name: [u8; 18],
    pub rear_tire_compound_name: [u8; 18],
    pub speed_limiter_available: u8,
    pub anti_stall_activated: u8,
    pub _unused: [u8; 2],
    pub visual_steering_wheel_range: f32,
    pub rear_brake_bias: f64,
    pub turbo_boost_pressure: f64,
    pub physics_to_graphics_offset: [f32; 3],
    pub physical_steering_wheel_range: f32,
    pub wheels: [rF2Wheel; 4],
    pub _expansion: [u8; 152],
}

/// Header preceding the vehicle-telemetry array in shared memory.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct rF2TelemetryHeader {
    pub version_update_begin: u32,
    pub version_update_end: u32,
    pub bytes_in_version: i32,
    pub bytes_in_header: i32,
    pub bytes_in_vehicle_telemetry: i32,
    pub num_vehicles: i32,
}

/// Top-level telemetry region: header + fixed-size array of vehicle telemetry.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct rF2Telemetry {
    pub header: rF2TelemetryHeader,
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

/// Per-vehicle scoring data: position, lap times, sector splits, pit state.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct rF2VehicleScoring {
    pub id: i32,
    pub driver_name: [u8; 32],
    pub vehicle_name: [u8; 64],
    pub total_laps: i16,
    pub sector: i8,
    pub finish_status: i8,
    pub lap_dist: f64,
    pub path_lateral: f64,
    pub track_edge: f64,
    pub best_sector1: f64,
    pub best_sector2: f64,
    pub best_lap_time: f64,
    pub last_sector1: f64,
    pub last_sector2: f64,
    pub last_lap_time: f64,
    pub cur_sector1: f64,
    pub cur_sector2: f64,
    pub num_pitstops: i16,
    pub num_penalties: i16,
    pub is_player: u8,
    pub control: i8,
    pub in_pits: u8,
    pub place: u8,
    pub vehicle_class: [u8; 32],
    pub time_behind_next: f64,
    pub laps_behind_next: i32,
    pub time_behind_leader: f64,
    pub laps_behind_leader: i32,
    pub lap_start_et: f64,
    pub pos: [f64; 3],
    pub local_vel: [f64; 3],
    pub local_accel: [f64; 3],
    pub ori: [[f64; 3]; 3],
    pub local_rot: [f64; 3],
    pub local_rot_accel: [f64; 3],
    pub headlights: u8,
    pub pit_state: u8,
    pub server_scored: u8,
    pub individual_phase: u8,
    pub qualification: i32,
    pub time_into_lap: f64,
    pub estimated_lap_time: f64,
    pub pit_group: [u8; 24],
    pub flag: u8,
    pub under_yellow: u8,
    pub count_laps_invalid_flags: u8,
    pub in_garage_stall: u8,
    pub upgrade_pack: [u8; 16],
    pub pit_lap_dist: f32,
    pub best_lap_sector1: f32,
    pub best_lap_sector2: f32,
    pub _expansion: [u8; 48],
}

/// Session-wide scoring info: track, weather, session type, flags.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct rF2ScoringInfo {
    pub track_name: [u8; 64],
    pub session: i32,
    pub current_et: f64,
    pub end_et: f64,
    pub max_laps: i32,
    pub lap_dist: f64,
    pub result_name: [u8; 64],
    pub num_vehicles: i32,
    pub game_phase: u8,
    pub yellow_flag_state: i8,
    pub sector_flag: [i8; 3],
    pub start_light: u8,
    pub num_red_lights: u8,
    pub in_realtime: u8,
    pub player_name: [u8; 32],
    pub plr_file_name: [u8; 64],
    pub dark_cloud: f64,
    pub raining: f64,
    pub ambient_temp: f64,
    pub track_temp: f64,
    pub wind: [f64; 3],
    pub min_path_wetness: f64,
    pub max_path_wetness: f64,
    pub game_mode: u8,
    pub is_password_protected: u8,
    pub server_port: u16,
    pub server_public_ip: u32,
    pub max_players: i32,
    pub server_name: [u8; 32],
    pub start_et: f32,
    pub avg_path_wetness: f64,
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
pub struct rF2ScoringHeader {
    pub version_update_begin: u32,
    pub version_update_end: u32,
    pub bytes_in_version: i32,
    pub bytes_in_header: i32,
    pub bytes_in_scoring_info: i32,
    pub bytes_in_vehicle_scoring: i32,
    pub num_vehicles: i32,
}

/// Top-level scoring region: header + session info + vehicle-scoring array.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct rF2Scoring {
    pub header: rF2ScoringHeader,
    pub scoring_info: rF2ScoringInfo,
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

/// Extended plugin info: plugin/session status flags and misc physics data.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct rF2Extended {
    pub version_update_begin: u32,
    pub version_update_end: u32,
    pub bytes_in_version: i32,
    pub bytes_in_extended: i32,
    pub physics_to_graphics_offset: [f32; 3],
    pub is_plugin_enabled: u8,
    pub direct_memory_access_enabled: u8,
    pub _padding: [u8; 2],
    pub session_started: u8,
    pub phys_avg_thread_time_ms: f64,
    pub _expansion: [u8; 508],
}

impl Default for rF2Extended {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// Converts a null-terminated byte buffer (as stored in rF2 structs) to a `String`.
///
/// Non-UTF-8 bytes are replaced with the Unicode replacement character.
pub fn parse_rf2_str(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).into_owned()
}
