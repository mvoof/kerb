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
pub struct SPageFilePhysics {
    pub packet_id: i32,
    pub gas: f32,
    pub brake: f32,
    pub fuel: f32,
    pub gear: i32,
    pub rpms: i32,
    pub steer_angle: f32,
    pub speed_kmh: f32,
    pub velocity: [f32; 3],
    pub acc_g: [f32; 3],
    pub wheel_slip: [f32; 4],
    pub wheel_load: [f32; 4],
    pub wheels_pressure: [f32; 4],
    pub wheel_angular_speed: [f32; 4],
    pub tyre_wear: [f32; 4],
    pub tyre_dirty_level: [f32; 4],
    pub tyre_core_temperature: [f32; 4],
    pub camber_rad: [f32; 4],
    pub suspension_travel: [f32; 4],
    pub drs: f32,
    pub tc: f32,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub cg_height: f32,
    pub car_damage: [f32; 5],
    pub number_of_tyres_out: i32,
    pub pit_limiter_on: i32,
    pub abs: f32,
    pub kers_charge: f32,
    pub kers_input: f32,
    pub auto_shifter_on: i32,
    pub ride_height: [f32; 2],
    pub turbo_boost: f32,
    pub ballast: f32,
    pub air_density: f32,
    pub air_temp: f32,
    pub road_temp: f32,
    pub local_angular_vel: [f32; 3],
    pub final_ff: f32,
    pub performance_meter: f32,
    pub engine_brake: i32,
    pub ers_recovery_level: i32,
    pub ers_power_level: i32,
    pub ers_heat_charging: i32,
    pub ers_is_charging: i32,
    pub kers_current_kj: f32,
    pub drs_available: i32,
    pub drs_enabled: i32,
    pub brake_temp: [f32; 4],
    pub clutch: f32,
    pub tyre_temp_i: [f32; 4],
    pub tyre_temp_m: [f32; 4],
    pub tyre_temp_o: [f32; 4],
    pub is_ai_controlled: i32,
    pub tyre_contact_point: [[f32; 3]; 4],
    pub tyre_contact_normal: [[f32; 3]; 4],
    pub tyre_contact_heading: [[f32; 3]; 4],
    pub brake_bias: f32,
    pub local_velocity: [f32; 3],
}

/// Graphics/HUD page — updated each rendered frame.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub struct SPageFileGraphics {
    pub packet_id: i32,
    pub status: i32,
    pub session: i32,
    pub current_time: [u16; 15],
    pub last_time: [u16; 15],
    pub best_time: [u16; 15],
    pub split: [u16; 15],
    pub completed_laps: i32,
    pub position: i32,
    pub i_current_time: i32,
    pub i_last_time: i32,
    pub i_best_time: i32,
    pub session_time_left: f32,
    pub distance_traveled: f32,
    pub is_in_pit: i32,
    pub current_sector_index: i32,
    pub last_sector_time: i32,
    pub number_of_laps: i32,
    pub tyre_compound: [u16; 33],
    pub replay_time_multiplier: f32,
    pub normalized_car_position: f32,
    pub active_cars: i32,
    pub car_coordinates: [[f32; 3]; 60],
    pub car_id: [i32; 60],
    pub player_car_id: i32,
    pub penalty_time: f32,
    pub flag: i32,
    pub penalty: i32,
    pub ideal_line_on: i32,
    pub is_in_pit_lane: i32,
    pub surface_grip: f32,
    pub mandatory_pit_done: i32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub is_setup_menu_visible: i32,
    pub main_display_index: i32,
    pub secondary_display_index: i32,
    pub tc: i32,
    pub tc_cut: i32,
    pub engine_map: i32,
    pub abs: i32,
    pub fuel_xlap: f32,
    pub rain_lights: i32,
    pub flashing_lights: i32,
    pub lights_stage: i32,
    pub exhaust_temperature: f32,
    pub wiper_lv: i32,
    pub driver_stint_total_time_left: i32,
    pub driver_stint_time_left: i32,
    pub rain_tyres: i32,
}

/// Static data page — written once when a session loads.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub struct SPageFileStatic {
    pub sm_version: [u16; 15],
    pub ac_version: [u16; 15],
    pub number_of_sessions: i32,
    pub num_cars: i32,
    pub car_model: [u16; 33],
    pub track: [u16; 33],
    pub player_name: [u16; 33],
    pub player_surname: [u16; 33],
    pub player_nick: [u16; 33],
    pub sector_count: i32,
    pub max_torque: f32,
    pub max_power: f32,
    pub max_rpm: i32,
    pub max_fuel: f32,
    pub suspension_max_travel: [f32; 4],
    pub tyre_radius: [f32; 4],
    pub max_turbo_boost: f32,
    pub _deprecated_1: f32,
    pub _deprecated_2: f32,
    pub penalties_enabled: i32,
    pub aid_fuel_rate: f32,
    pub aid_tire_rate: f32,
    pub aid_mechanical_damage: f32,
    pub aid_allow_tyre_blankets: i32,
    pub aid_stability: f32,
    pub aid_auto_clutch: i32,
    pub aid_auto_blip: i32,
    pub has_drs: i32,
    pub has_ers: i32,
    pub has_kers: i32,
    pub kers_max_j: f32,
    pub engine_brake_settings_count: i32,
    pub ers_power_controller_count: i32,
    pub track_spline_length: f32,
    pub track_configuration: [u16; 33],
    pub ers_max_j: f32,
    pub is_timed_race: i32,
    pub has_extra_lap: i32,
    pub car_skin: [u16; 33],
    pub reversed_grid_positions: i32,
    pub pit_window_start: i32,
    pub pit_window_end: i32,
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

impl SPageFileStatic {
    pub fn track_name(&self) -> String {
        let t = self.track;
        String::from_utf16_lossy(&t[..t.iter().position(|&x| x == 0).unwrap_or(33)]).to_string()
    }
    pub fn car_model_name(&self) -> String {
        let c = self.car_model;
        String::from_utf16_lossy(&c[..c.iter().position(|&x| x == 0).unwrap_or(33)]).to_string()
    }
    pub fn player_full_name(&self) -> String {
        let n = self.player_name;
        let s = self.player_surname;
        let first = String::from_utf16_lossy(&n[..n.iter().position(|&x| x == 0).unwrap_or(33)]);
        let last = String::from_utf16_lossy(&s[..s.iter().position(|&x| x == 0).unwrap_or(33)]);
        format!("{} {}", first, last)
    }
}

// ── Assetto Corsa Evo ─────────────────────────────────────────────────────────

/// Per-tyre state embedded in the Evo graphics page.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub struct SMEvoTyreState {
    pub tyre_compound_symbol: [u16; 33],
    pub tyre_compound_name: [u16; 50],
    pub tyre_compound_short: [u16; 5],
    pub tyre_compound_color: [u16; 10],
    pub is_wet_tyre: i32,
    pub tyre_surface_temp: [f32; 3],
    pub tyre_inner_temp: [f32; 3],
    pub tyre_core_temp: f32,
    pub tyre_pressure: f32,
    pub tyre_slip_ratio: f32,
    pub tyre_slip_angle: f32,
    pub tyre_nd_slip: f32,
    pub tyre_load: f32,
    pub tyre_dirt_level: f32,
    pub tyre_wear: f32,
    pub tyre_angular_speed: f32,
    pub suspension_travel: f32,
    pub camber: f32,
    pub rim_temp: f32,
    pub brake_disc_temp: f32,
    pub brake_disc_wear: f32,
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
pub struct SPageFilePhysicsEvo {
    pub packet_id: i32,
    pub gas: f32,
    pub brake: f32,
    pub fuel: f32,
    pub gear: i32,
    pub rpms: i32,
    pub steer_angle: f32,
    pub speed_kmh: f32,
    pub velocity: [f32; 3],
    pub acc_g: [f32; 3],
    pub wheel_slip: [f32; 4],
    pub wheel_load: [f32; 4],
    pub wheels_pressure: [f32; 4],
    pub wheel_angular_speed: [f32; 4],
    pub tyre_wear: [f32; 4],
    pub tyre_dirty_level: [f32; 4],
    pub tyre_core_temperature: [f32; 4],
    pub camber_rad: [f32; 4],
    pub suspension_travel: [f32; 4],
    pub drs: f32,
    pub tc: f32,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub cg_height: f32,
    pub car_damage: [f32; 5],
    pub number_of_tyres_out: i32,
    pub pit_limiter_on: i32,
    pub abs: f32,
    pub auto_shifter_on: i32,
    pub ride_height: [f32; 2],
    pub turbo_boost: f32,
    pub ballast: f32,
    pub air_density: f32,
    pub air_temp: f32,
    pub road_temp: f32,
    pub local_angular_vel: [f32; 3],
    pub final_ff: f32,
    pub engine_brake: i32,
    pub brake_temp: [f32; 4],
    pub clutch: f32,
    pub tyre_temp_i: [f32; 4],
    pub tyre_temp_m: [f32; 4],
    pub tyre_temp_o: [f32; 4],
    pub is_ai_controlled: i32,
    pub brake_bias: f32,
    pub local_velocity: [f32; 3],
    pub front_brake_compound: i32,
    pub rear_brake_compound: i32,
    pub pad_life: [f32; 4],
    pub disc_life: [f32; 4],
    pub ignition_on: i32,
    pub starter_engine_on: i32,
    pub is_engine_running: i32,
    pub kerb_vibration: f32,
    pub slip_vibrations: f32,
    pub g_vibrations: f32,
    pub abs_vibrations: f32,
}

/// Graphics/HUD page for AC Evo — superset of classic AC graphics.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub struct SPageFileGraphicsEvo {
    pub packet_id: i32,
    pub status: i32,
    pub session: i32,
    pub current_time: [u16; 15],
    pub last_time: [u16; 15],
    pub best_time: [u16; 15],
    pub split: [u16; 15],
    pub completed_laps: i32,
    pub position: i32,
    pub i_current_time: i32,
    pub i_last_time: i32,
    pub i_best_time: i32,
    pub session_time_left: f32,
    pub distance_traveled: f32,
    pub is_in_pit: i32,
    pub current_sector_index: i32,
    pub last_sector_time: i32,
    pub number_of_laps: i32,
    pub tyre_compound: [u16; 33],
    pub normalized_car_position: f32,
    pub active_cars: i32,
    pub car_coordinates: [[f32; 3]; 60],
    pub car_id: [i32; 60],
    pub player_car_id: i32,
    pub penalty_time: f32,
    pub flag: i32,
    pub penalty: i32,
    pub ideal_line_on: i32,
    pub is_in_pit_lane: i32,
    pub surface_grip: f32,
    pub mandatory_pit_done: i32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub tc: i32,
    pub tc_cut: i32,
    pub engine_map: i32,
    pub abs: i32,
    pub fuel_xlap: f32,
    pub rain_lights: i32,
    pub flashing_lights: i32,
    pub lights_stage: i32,
    pub exhaust_temperature: f32,
    pub wiper_lv: i32,
    pub driver_stint_total_time_left: i32,
    pub driver_stint_time_left: i32,
    pub rain_tyres: i32,
    pub session_index: i32,
    pub used_fuel: f32,
    pub delta_lap_time: [u16; 15],
    pub i_delta_lap_time: i32,
    pub estimated_lap_time: [u16; 15],
    pub i_estimated_lap_time: i32,
    pub is_delta_positive: i32,
    pub i_split: i32,
    pub is_valid_lap: i32,
    pub fuel_estimated_laps: f32,
    pub track_status: [u16; 33],
    pub missing_mandatory_pits: i32,
    pub clock: f32,
    pub direction_lights_left: i32,
    pub direction_lights_right: i32,
    pub global_yellow: i32,
    pub global_yellow_1: i32,
    pub global_yellow_2: i32,
    pub global_yellow_3: i32,
    pub global_white: i32,
    pub global_green: i32,
    pub global_chequered: i32,
    pub global_red: i32,
    pub mfd_tyre_set: i32,
    pub mfd_fuel_to_add: f32,
    pub mfd_tyre_pressure_lf: f32,
    pub mfd_tyre_pressure_rf: f32,
    pub mfd_tyre_pressure_lr: f32,
    pub mfd_tyre_pressure_rr: f32,
    pub track_grip_status: i32,
    pub rain_intensity: i32,
    pub rain_intensity_in_10min: i32,
    pub rain_intensity_in_30min: i32,
    pub current_tyre_set: i32,
    pub strategy_tyre_set: i32,
    pub tyres: [SMEvoTyreState; 4],
}

/// Static page for AC Evo.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Snapshot)]
pub struct SPageFileStaticEvo {
    pub sm_version: [u16; 15],
    pub ac_version: [u16; 15],
    pub number_of_sessions: i32,
    pub num_cars: i32,
    pub car_model: [u16; 33],
    pub track: [u16; 33],
    pub player_name: [u16; 33],
    pub player_surname: [u16; 33],
    pub player_nick: [u16; 33],
    pub sector_count: i32,
    pub max_torque: f32,
    pub max_power: f32,
    pub max_rpm: i32,
    pub max_fuel: f32,
    pub suspension_max_travel: [f32; 4],
    pub tyre_radius: [f32; 4],
    pub max_turbo_boost: f32,
    pub penalties_enabled: i32,
    pub aid_fuel_rate: f32,
    pub aid_tire_rate: f32,
    pub aid_mechanical_damage: f32,
    pub aid_allow_tyre_blankets: i32,
    pub aid_stability: f32,
    pub aid_auto_clutch: i32,
    pub aid_auto_blip: i32,
    pub pit_window_start: i32,
    pub pit_window_end: i32,
    pub is_online: i32,
    pub dry_tyres_name: [u16; 33],
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
