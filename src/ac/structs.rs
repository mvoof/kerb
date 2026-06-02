//! `#[repr(C, packed)]` structs that mirror Assetto Corsa Evo's shared
//! memory layouts byte-for-byte. Field order and sizes must match the game
//! definitions exactly — do not reorder or add padding.
//!
//! Source: https://www.assettocorsa.net/forum/index.php?threads/shared-memory-api-documentation.83659/.
//! https://docs.google.com/document/d/1WzqMLkW2o_C0LGcvdMRelAV31ZIifux0CSHD9k6ddz0/edit?tab=t.0

pub const AC_STATUS_OFF: i32 = 0;
pub const AC_STATUS_REPLAY: i32 = 1;
pub const AC_STATUS_LIVE: i32 = 2;
pub const AC_STATUS_PAUSE: i32 = 3;

// ── Assetto Corsa Evo ─────────────────────────────────────────────────────────

/// Physics telemetry page — updated every simulation tick.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SPageFilePhysicsEvo {
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
    pub p2p_activations: i32,
    pub p2p_status: i32,
    pub current_max_rpm: i32,
    pub mz: [f32; 4],
    pub fx: [f32; 4],
    pub fy: [f32; 4],
    pub slip_ratio: [f32; 4],
    pub slip_angle: [f32; 4],
    pub tcin_action: i32,
    pub abs_in_action: i32,
    pub suspension_damage: [f32; 4],
    pub tyre_temp: [f32; 4],
    pub water_temp: f32,
    pub brake_torque: [f32; 4],
    pub front_brake_compound: i32,
    pub rear_brake_compound: i32,
    pub pad_life: [f32; 4],
    pub disc_life: [f32; 4],
    pub ignition_on: i32,
    pub starter_engine_on: i32,
    pub is_engine_running: i32,
    pub kerb_vibration: f32,
    pub slip_vibrations: f32,
    pub road_vibrations: f32,
    pub abs_vibrations: f32,
}

/// Per-tyre state. Fixed size: 256 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoTyreState {
    pub slip: f32,
    pub lock: bool,
    pub tyre_pressure: f32,
    pub tyre_temperature_c: f32,
    pub brake_temperature_c: f32,
    pub brake_pressure: f32,
    pub tyre_temperature_left: f32,
    pub tyre_temperature_center: f32,
    pub tyre_temperature_right: f32,
    pub tyre_compound_front: [u8; 33],
    pub tyre_compound_rear: [u8; 33],
    pub tyre_normalized_pressure: f32,
    pub tyre_normalized_temperature_left: f32,
    pub tyre_normalized_temperature_center: f32,
    pub tyre_normalized_temperature_right: f32,
    pub brake_normalized_temperature: f32,
    pub tyre_normalized_temperature_core: f32,
    pub _pad: [u8; 133],
}

/// Structural damage per body zone. Fixed size: 128 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoDamageState {
    pub damage_front: f32,
    pub damage_rear: f32,
    pub damage_left: f32,
    pub damage_right: f32,
    pub damage_center: f32,
    pub damage_suspension_lf: f32,
    pub damage_suspension_rf: f32,
    pub damage_suspension_lr: f32,
    pub damage_suspension_rr: f32,
    pub _pad: [u8; 92],
}

/// Pit-stop service action states. Fixed size: 64 bytes.
/// Values: -1 = will not perform, 0 = completed, 1 = in progress.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoPitInfo {
    pub damage: i8,
    pub fuel: i8,
    pub tyres_lf: i8,
    pub tyres_rf: i8,
    pub tyres_lr: i8,
    pub tyres_rr: i8,
    pub _pad: [u8; 58],
}

/// Driver-adjustable electronic aid settings. Fixed size: 128 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoElectronics {
    pub tc_level: i8,
    pub tc_cut_level: i8,
    pub abs_level: i8,
    pub esc_level: i8,
    pub ebb_level: i8,
    pub brake_bias: f32,
    pub engine_map_level: i8,
    pub turbo_level: f32,
    pub ers_deployment_map: i8,
    pub ers_recharge_map: f32,
    pub is_ers_heat_charging_on: bool,
    pub is_ers_overtake_mode_on: bool,
    pub is_drs_open: bool,
    pub diff_power_level: i8,
    pub diff_coast_level: i8,
    pub front_bump_damper_level: i8,
    pub front_rebound_damper_level: i8,
    pub rear_bump_damper_level: i8,
    pub rear_rebound_damper_level: i8,
    pub is_ignition_on: bool,
    pub is_pitlimiter_on: bool,
    pub active_performance_mode: i8,
    pub _pad: [u8; 97],
}

/// Cockpit lights, display, and wiper states. Fixed size: 128 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoInstrumentation {
    pub main_light_stage: i8,
    pub special_light_stage: i8,
    pub cockpit_light_stage: i8,
    pub wiper_level: i8,
    pub rain_lights: bool,
    pub direction_light_left: bool,
    pub direction_light_right: bool,
    pub flashing_lights: bool,
    pub warning_lights: bool,
    pub selected_display_index: i8,
    pub display_current_page_index: [i8; 16],
    pub are_headlights_visible: bool,
    pub _pad: [u8; 101],
}

/// Session lifecycle and countdown. Fixed size: 256 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoSessionState {
    pub phase_name: [u8; 33],
    pub time_left: [u8; 15],
    pub time_left_ms: i32,
    pub wait_time: [u8; 15],
    pub total_lap: i32,
    pub current_lap: i32,
    pub lights_on: i32,
    pub lights_mode: i32,
    pub lap_length_km: f32,
    pub end_session_flag: i32,
    pub time_to_next_session: [u8; 15],
    pub disconnected_from_server: bool,
    pub restart_season_enabled: bool,
    pub ui_enable_drive: bool,
    pub ui_enable_setup: bool,
    pub is_ready_to_next_blinking: bool,
    pub show_waiting_for_players: bool,
    pub _pad: [u8; 144],
}

/// HUD lap timing and delta display. Fixed size: 256 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoTimingState {
    pub current_laptime: [u8; 15],
    pub delta_current: [u8; 15],
    pub delta_current_p: i32,
    pub last_laptime: [u8; 15],
    pub delta_last: [u8; 15],
    pub delta_last_p: i32,
    pub best_laptime: [u8; 15],
    pub ideal_laptime: [u8; 15],
    pub total_time: [u8; 15],
    pub is_invalid: bool,
    pub _pad: [u8; 142],
}

/// Active driver-assist levels. Fixed size: 64 bytes.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SMEvoAssistsState {
    pub auto_gear: u8,
    pub auto_blip: u8,
    pub auto_clutch: u8,
    pub auto_clutch_on_start: u8,
    pub manual_ignition_e_start: u8,
    pub auto_pit_limiter: u8,
    pub standing_start_assist: u8,
    pub auto_steer: f32,
    pub arcade_stability_control: f32,
    pub _pad: [u8; 49],
}

/// Graphics/HUD page for AC Evo. Updated each rendered frame.
///
/// Layout: ACE_SharedFileOut_Documentation_v1.md (changelog 2026-04-01).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SPageFileGraphicsEvo {
    pub packet_id: i32,
    pub status: i32,
    pub focused_car_id_a: u64,
    pub focused_car_id_b: u64,
    pub player_car_id_a: u64,
    pub player_car_id_b: u64,
    pub rpm: u16,
    pub is_rpm_limiter_on: bool,
    pub is_change_up_rpm: bool,
    pub is_change_down_rpm: bool,
    pub tc_active: bool,
    pub abs_active: bool,
    pub esc_active: bool,
    pub launch_active: bool,
    pub is_ignition_on: bool,
    pub is_engine_running: bool,
    pub kers_is_charging: bool,
    pub is_wrong_way: bool,
    pub is_drs_available: bool,
    pub battery_is_charging: bool,
    pub is_max_kj_per_lap_reached: bool,
    pub is_max_charge_kj_per_lap_reached: bool,
    pub display_speed_kmh: i16,
    pub display_speed_mph: i16,
    pub display_speed_ms: i16,
    pub pitspeeding_delta: f32,
    pub gear_int: i16,
    pub rpm_percent: f32,
    pub gas_percent: f32,
    pub brake_percent: f32,
    pub handbrake_percent: f32,
    pub clutch_percent: f32,
    pub steering_percent: f32,
    pub ffb_strength: f32,
    pub car_ffb_multiplier: f32,
    pub water_temperature_percent: f32,
    pub water_pressure_bar: f32,
    pub fuel_pressure_bar: f32,
    pub water_temperature_c: i8,
    pub air_temperature_c: i8,
    pub oil_temperature_c: f32,
    pub oil_pressure_bar: f32,
    pub exhaust_temperature_c: f32,
    pub g_forces_x: f32,
    pub g_forces_y: f32,
    pub g_forces_z: f32,
    pub turbo_boost: f32,
    pub turbo_boost_level: f32,
    pub turbo_boost_perc: f32,
    pub steer_degrees: i32,
    pub current_km: f32,
    pub total_km: u32,
    pub total_driving_time_s: u32,
    pub time_of_day_hours: i32,
    pub time_of_day_minutes: i32,
    pub time_of_day_seconds: i32,
    pub delta_time_ms: i32,
    pub current_lap_time_ms: i32,
    pub predicted_lap_time_ms: i32,
    pub fuel_liter_current_quantity: f32,
    pub fuel_liter_current_quantity_percent: f32,
    pub fuel_liter_per_km: f32,
    pub km_per_fuel_liter: f32,
    pub current_torque: f32,
    pub current_bhp: i32,
    pub tyre_lf: SMEvoTyreState,
    pub tyre_rf: SMEvoTyreState,
    pub tyre_lr: SMEvoTyreState,
    pub tyre_rr: SMEvoTyreState,
    pub npos: f32,
    pub kers_charge_perc: f32,
    pub kers_current_perc: f32,
    pub control_lock_time: f32,
    pub car_damage: SMEvoDamageState,
    pub car_location: i32,
    pub pit_info: SMEvoPitInfo,
    pub fuel_liter_used: f32,
    pub fuel_liter_per_lap: f32,
    pub laps_possible_with_fuel: f32,
    pub battery_temperature: f32,
    pub battery_voltage: f32,
    pub instantaneous_fuel_liter_per_km: f32,
    pub instantaneous_km_per_fuel_liter: f32,
    pub gear_rpm_window: f32,
    pub instrumentation: SMEvoInstrumentation,
    pub instrumentation_min_limit: SMEvoInstrumentation,
    pub instrumentation_max_limit: SMEvoInstrumentation,
    pub electronics: SMEvoElectronics,
    pub electronics_min_limit: SMEvoElectronics,
    pub electronics_max_limit: SMEvoElectronics,
    pub electronics_is_modifiable: SMEvoElectronics,
    pub total_lap_count: i32,
    pub current_pos: u32,
    pub total_drivers: u32,
    pub last_laptime_ms: i32,
    pub best_laptime_ms: i32,
    pub flag: i32,
    pub global_flag: i32,
    pub max_gears: u32,
    pub engine_type: i32,
    pub has_kers: bool,
    pub is_last_lap: bool,
    pub performance_mode_name: [u8; 33],
    pub diff_coast_raw_value: f32,
    pub diff_power_raw_value: f32,
    pub race_cut_gained_time_ms: i32,
    pub distance_to_deadline: i32,
    pub race_cut_current_delta: f32,
    pub session_state: SMEvoSessionState,
    pub timing_state: SMEvoTimingState,
    pub player_ping: i32,
    pub player_latency: i32,
    pub player_cpu_usage: i32,
    pub player_cpu_usage_avg: i32,
    pub player_qos: i32,
    pub player_qos_avg: i32,
    pub player_fps: i32,
    pub player_fps_avg: i32,
    pub driver_name: [u8; 33],
    pub driver_surname: [u8; 33],
    pub car_model_name: [u8; 33],
    pub is_in_pit_box: bool,
    pub is_in_pit_lane: bool,
    pub is_valid_lap: bool,
    pub car_coordinates: [[f32; 3]; 60],
    pub gap_ahead: f32,
    pub gap_behind: f32,
    pub active_cars: u8,
    pub fuel_per_lap: f32,
    pub fuel_estimated_laps: f32,
    pub assists_state: SMEvoAssistsState,
    pub max_fuel: f32,
    pub max_turbo_boost: f32,
    pub use_single_compound: bool,
    pub car_ids: [[u64; 2]; 60],
}

/// Static session metadata for AC Evo — written once at session load.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SPageFileStaticEvo {
    pub sm_version: [u8; 15],
    pub ac_evo_version: [u8; 15],
    pub session: i32,
    pub session_name: [u8; 33],
    pub event_id: u8,
    pub session_id: u8,
    pub starting_grip: i32,
    pub starting_ambient_temperature_c: f32,
    pub starting_ground_temperature_c: f32,
    pub is_static_weather: bool,
    pub is_timed_race: bool,
    pub is_online: bool,
    pub number_of_sessions: i32,
    pub nation: [u8; 33],
    pub longitude: f32,
    pub latitude: f32,
    pub track: [u8; 33],
    pub track_configuration: [u8; 33],
    pub track_length_m: f32,
}

macro_rules! zeroed_default {
    ($($t:ty),*) => {
        $(impl Default for $t {
            fn default() -> Self {
                // SAFETY: all fields are primitive numeric types (integers, floats,
                // fixed-size arrays of the same). Zero is a valid bit-pattern for all of them.
                unsafe { std::mem::zeroed() }
            }
        })*
    };
}
zeroed_default!(
    SPageFilePhysicsEvo,
    SMEvoTyreState,
    SMEvoDamageState,
    SMEvoPitInfo,
    SMEvoElectronics,
    SMEvoInstrumentation,
    SMEvoSessionState,
    SMEvoTimingState,
    SMEvoAssistsState,
    SPageFileGraphicsEvo,
    SPageFileStaticEvo
);
