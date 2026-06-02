//! Public mirror types for AC Evo shared-memory structs.
//!
//! These are safe, non-packed copies of the raw `SPageFile*` and `SMEvo*` structs.
//! `[u8; N]` string fields are wrapped in [`SimString<N>`].

use crate::ac::structs::{
    SMEvoAssistsState, SMEvoDamageState, SMEvoElectronics, SMEvoInstrumentation, SMEvoPitInfo,
    SMEvoSessionState, SMEvoTimingState, SMEvoTyreState, SPageFileGraphicsEvo, SPageFilePhysicsEvo,
    SPageFileStaticEvo,
};
use crate::sim_string::SimString;

fn to_str<const N: usize>(bytes: [u8; N]) -> SimString<N> {
    SimString::from_u8_array(bytes)
}

// ── Per-tyre ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoTyreState {
    pub slip: f32,
    pub lock: bool,
    pub tyre_pressure: f32,
    pub tyre_temperature_c: f32,
    pub brake_temperature_c: f32,
    pub brake_pressure: f32,
    pub tyre_temperature_left: f32,
    pub tyre_temperature_center: f32,
    pub tyre_temperature_right: f32,
    pub tyre_compound_front: SimString<33>,
    pub tyre_compound_rear: SimString<33>,
    pub tyre_normalized_pressure: f32,
    pub tyre_normalized_temperature_left: f32,
    pub tyre_normalized_temperature_center: f32,
    pub tyre_normalized_temperature_right: f32,
    pub brake_normalized_temperature: f32,
    pub tyre_normalized_temperature_core: f32,
}

impl From<SMEvoTyreState> for AcEvoTyreState {
    fn from(r: SMEvoTyreState) -> Self {
        Self {
            slip: r.slip,
            lock: r.lock,
            tyre_pressure: r.tyre_pressure,
            tyre_temperature_c: r.tyre_temperature_c,
            brake_temperature_c: r.brake_temperature_c,
            brake_pressure: r.brake_pressure,
            tyre_temperature_left: r.tyre_temperature_left,
            tyre_temperature_center: r.tyre_temperature_center,
            tyre_temperature_right: r.tyre_temperature_right,
            tyre_compound_front: to_str(r.tyre_compound_front),
            tyre_compound_rear: to_str(r.tyre_compound_rear),
            tyre_normalized_pressure: r.tyre_normalized_pressure,
            tyre_normalized_temperature_left: r.tyre_normalized_temperature_left,
            tyre_normalized_temperature_center: r.tyre_normalized_temperature_center,
            tyre_normalized_temperature_right: r.tyre_normalized_temperature_right,
            brake_normalized_temperature: r.brake_normalized_temperature,
            tyre_normalized_temperature_core: r.tyre_normalized_temperature_core,
        }
    }
}

// ── Damage ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoDamageState {
    pub damage_front: f32,
    pub damage_rear: f32,
    pub damage_left: f32,
    pub damage_right: f32,
    pub damage_center: f32,
    pub damage_suspension_lf: f32,
    pub damage_suspension_rf: f32,
    pub damage_suspension_lr: f32,
    pub damage_suspension_rr: f32,
}

impl From<SMEvoDamageState> for AcEvoDamageState {
    fn from(r: SMEvoDamageState) -> Self {
        Self {
            damage_front: r.damage_front,
            damage_rear: r.damage_rear,
            damage_left: r.damage_left,
            damage_right: r.damage_right,
            damage_center: r.damage_center,
            damage_suspension_lf: r.damage_suspension_lf,
            damage_suspension_rf: r.damage_suspension_rf,
            damage_suspension_lr: r.damage_suspension_lr,
            damage_suspension_rr: r.damage_suspension_rr,
        }
    }
}

// ── Pit info ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoPitInfo {
    pub damage: i8,
    pub fuel: i8,
    pub tyres_lf: i8,
    pub tyres_rf: i8,
    pub tyres_lr: i8,
    pub tyres_rr: i8,
}

impl From<SMEvoPitInfo> for AcEvoPitInfo {
    fn from(r: SMEvoPitInfo) -> Self {
        Self {
            damage: r.damage,
            fuel: r.fuel,
            tyres_lf: r.tyres_lf,
            tyres_rf: r.tyres_rf,
            tyres_lr: r.tyres_lr,
            tyres_rr: r.tyres_rr,
        }
    }
}

// ── Electronics ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoElectronics {
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
}

impl From<SMEvoElectronics> for AcEvoElectronics {
    fn from(r: SMEvoElectronics) -> Self {
        Self {
            tc_level: r.tc_level,
            tc_cut_level: r.tc_cut_level,
            abs_level: r.abs_level,
            esc_level: r.esc_level,
            ebb_level: r.ebb_level,
            brake_bias: r.brake_bias,
            engine_map_level: r.engine_map_level,
            turbo_level: r.turbo_level,
            ers_deployment_map: r.ers_deployment_map,
            ers_recharge_map: r.ers_recharge_map,
            is_ers_heat_charging_on: r.is_ers_heat_charging_on,
            is_ers_overtake_mode_on: r.is_ers_overtake_mode_on,
            is_drs_open: r.is_drs_open,
            diff_power_level: r.diff_power_level,
            diff_coast_level: r.diff_coast_level,
            front_bump_damper_level: r.front_bump_damper_level,
            front_rebound_damper_level: r.front_rebound_damper_level,
            rear_bump_damper_level: r.rear_bump_damper_level,
            rear_rebound_damper_level: r.rear_rebound_damper_level,
            is_ignition_on: r.is_ignition_on,
            is_pitlimiter_on: r.is_pitlimiter_on,
            active_performance_mode: r.active_performance_mode,
        }
    }
}

// ── Instrumentation ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoInstrumentation {
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
}

impl From<SMEvoInstrumentation> for AcEvoInstrumentation {
    fn from(r: SMEvoInstrumentation) -> Self {
        Self {
            main_light_stage: r.main_light_stage,
            special_light_stage: r.special_light_stage,
            cockpit_light_stage: r.cockpit_light_stage,
            wiper_level: r.wiper_level,
            rain_lights: r.rain_lights,
            direction_light_left: r.direction_light_left,
            direction_light_right: r.direction_light_right,
            flashing_lights: r.flashing_lights,
            warning_lights: r.warning_lights,
            selected_display_index: r.selected_display_index,
            display_current_page_index: r.display_current_page_index,
            are_headlights_visible: r.are_headlights_visible,
        }
    }
}

// ── Session state ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoSessionState {
    pub phase_name: SimString<33>,
    pub time_left: SimString<15>,
    pub time_left_ms: i32,
    pub wait_time: SimString<15>,
    pub total_lap: i32,
    pub current_lap: i32,
    pub lights_on: i32,
    pub lights_mode: i32,
    pub lap_length_km: f32,
    pub end_session_flag: i32,
    pub time_to_next_session: SimString<15>,
    pub disconnected_from_server: bool,
    pub restart_season_enabled: bool,
    pub ui_enable_drive: bool,
    pub ui_enable_setup: bool,
    pub is_ready_to_next_blinking: bool,
    pub show_waiting_for_players: bool,
}

impl From<SMEvoSessionState> for AcEvoSessionState {
    fn from(r: SMEvoSessionState) -> Self {
        Self {
            phase_name: to_str(r.phase_name),
            time_left: to_str(r.time_left),
            time_left_ms: r.time_left_ms,
            wait_time: to_str(r.wait_time),
            total_lap: r.total_lap,
            current_lap: r.current_lap,
            lights_on: r.lights_on,
            lights_mode: r.lights_mode,
            lap_length_km: r.lap_length_km,
            end_session_flag: r.end_session_flag,
            time_to_next_session: to_str(r.time_to_next_session),
            disconnected_from_server: r.disconnected_from_server,
            restart_season_enabled: r.restart_season_enabled,
            ui_enable_drive: r.ui_enable_drive,
            ui_enable_setup: r.ui_enable_setup,
            is_ready_to_next_blinking: r.is_ready_to_next_blinking,
            show_waiting_for_players: r.show_waiting_for_players,
        }
    }
}

// ── Timing state ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoTimingState {
    pub current_laptime: SimString<15>,
    pub delta_current: SimString<15>,
    pub delta_current_p: i32,
    pub last_laptime: SimString<15>,
    pub delta_last: SimString<15>,
    pub delta_last_p: i32,
    pub best_laptime: SimString<15>,
    pub ideal_laptime: SimString<15>,
    pub total_time: SimString<15>,
    pub is_invalid: bool,
}

impl From<SMEvoTimingState> for AcEvoTimingState {
    fn from(r: SMEvoTimingState) -> Self {
        Self {
            current_laptime: to_str(r.current_laptime),
            delta_current: to_str(r.delta_current),
            delta_current_p: r.delta_current_p,
            last_laptime: to_str(r.last_laptime),
            delta_last: to_str(r.delta_last),
            delta_last_p: r.delta_last_p,
            best_laptime: to_str(r.best_laptime),
            ideal_laptime: to_str(r.ideal_laptime),
            total_time: to_str(r.total_time),
            is_invalid: r.is_invalid,
        }
    }
}

// ── Assists ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcEvoAssistsState {
    pub auto_gear: u8,
    pub auto_blip: u8,
    pub auto_clutch: u8,
    pub auto_clutch_on_start: u8,
    pub manual_ignition_e_start: u8,
    pub auto_pit_limiter: u8,
    pub standing_start_assist: u8,
    pub auto_steer: f32,
    pub arcade_stability_control: f32,
}

impl From<SMEvoAssistsState> for AcEvoAssistsState {
    fn from(r: SMEvoAssistsState) -> Self {
        Self {
            auto_gear: r.auto_gear,
            auto_blip: r.auto_blip,
            auto_clutch: r.auto_clutch,
            auto_clutch_on_start: r.auto_clutch_on_start,
            manual_ignition_e_start: r.manual_ignition_e_start,
            auto_pit_limiter: r.auto_pit_limiter,
            standing_start_assist: r.standing_start_assist,
            auto_steer: r.auto_steer,
            arcade_stability_control: r.arcade_stability_control,
        }
    }
}

// ── Physics ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcPhysicsData {
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

impl From<SPageFilePhysicsEvo> for AcPhysicsData {
    fn from(r: SPageFilePhysicsEvo) -> Self {
        Self {
            packet_id: r.packet_id,
            gas: r.gas,
            brake: r.brake,
            fuel: r.fuel,
            gear: r.gear,
            rpms: r.rpms,
            steer_angle: r.steer_angle,
            speed_kmh: r.speed_kmh,
            velocity: r.velocity,
            acc_g: r.acc_g,
            wheel_slip: r.wheel_slip,
            wheel_load: r.wheel_load,
            wheels_pressure: r.wheels_pressure,
            wheel_angular_speed: r.wheel_angular_speed,
            tyre_wear: r.tyre_wear,
            tyre_dirty_level: r.tyre_dirty_level,
            tyre_core_temperature: r.tyre_core_temperature,
            camber_rad: r.camber_rad,
            suspension_travel: r.suspension_travel,
            drs: r.drs,
            tc: r.tc,
            heading: r.heading,
            pitch: r.pitch,
            roll: r.roll,
            cg_height: r.cg_height,
            car_damage: r.car_damage,
            number_of_tyres_out: r.number_of_tyres_out,
            pit_limiter_on: r.pit_limiter_on,
            abs: r.abs,
            kers_charge: r.kers_charge,
            kers_input: r.kers_input,
            auto_shifter_on: r.auto_shifter_on,
            ride_height: r.ride_height,
            turbo_boost: r.turbo_boost,
            ballast: r.ballast,
            air_density: r.air_density,
            air_temp: r.air_temp,
            road_temp: r.road_temp,
            local_angular_vel: r.local_angular_vel,
            final_ff: r.final_ff,
            performance_meter: r.performance_meter,
            engine_brake: r.engine_brake,
            ers_recovery_level: r.ers_recovery_level,
            ers_power_level: r.ers_power_level,
            ers_heat_charging: r.ers_heat_charging,
            ers_is_charging: r.ers_is_charging,
            kers_current_kj: r.kers_current_kj,
            drs_available: r.drs_available,
            drs_enabled: r.drs_enabled,
            brake_temp: r.brake_temp,
            clutch: r.clutch,
            tyre_temp_i: r.tyre_temp_i,
            tyre_temp_m: r.tyre_temp_m,
            tyre_temp_o: r.tyre_temp_o,
            is_ai_controlled: r.is_ai_controlled,
            tyre_contact_point: r.tyre_contact_point,
            tyre_contact_normal: r.tyre_contact_normal,
            tyre_contact_heading: r.tyre_contact_heading,
            brake_bias: r.brake_bias,
            local_velocity: r.local_velocity,
            p2p_activations: r.p2p_activations,
            p2p_status: r.p2p_status,
            current_max_rpm: r.current_max_rpm,
            mz: r.mz,
            fx: r.fx,
            fy: r.fy,
            slip_ratio: r.slip_ratio,
            slip_angle: r.slip_angle,
            tcin_action: r.tcin_action,
            abs_in_action: r.abs_in_action,
            suspension_damage: r.suspension_damage,
            tyre_temp: r.tyre_temp,
            water_temp: r.water_temp,
            brake_torque: r.brake_torque,
            front_brake_compound: r.front_brake_compound,
            rear_brake_compound: r.rear_brake_compound,
            pad_life: r.pad_life,
            disc_life: r.disc_life,
            ignition_on: r.ignition_on,
            starter_engine_on: r.starter_engine_on,
            is_engine_running: r.is_engine_running,
            kerb_vibration: r.kerb_vibration,
            slip_vibrations: r.slip_vibrations,
            road_vibrations: r.road_vibrations,
            abs_vibrations: r.abs_vibrations,
        }
    }
}

// ── Graphics ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct AcGraphicsData {
    pub packet_id: i32,
    pub status: i32,
    pub rpm: u16,
    pub tc_active: bool,
    pub abs_active: bool,
    pub esc_active: bool,
    pub is_engine_running: bool,
    pub is_drs_available: bool,
    pub display_speed_kmh: i16,
    pub gear_int: i16,
    pub current_lap_time_ms: i32,
    pub predicted_lap_time_ms: i32,
    pub delta_time_ms: i32,
    pub fuel_liter_current_quantity: f32,
    pub fuel_liter_current_quantity_percent: f32,
    pub current_torque: f32,
    pub current_bhp: i32,
    pub tyre_lf: AcEvoTyreState,
    pub tyre_rf: AcEvoTyreState,
    pub tyre_lr: AcEvoTyreState,
    pub tyre_rr: AcEvoTyreState,
    pub npos: f32,
    pub kers_charge_perc: f32,
    pub car_damage: AcEvoDamageState,
    pub car_location: i32,
    pub pit_info: AcEvoPitInfo,
    pub fuel_liter_used: f32,
    pub fuel_liter_per_lap: f32,
    pub instrumentation: AcEvoInstrumentation,
    pub electronics: AcEvoElectronics,
    pub electronics_min_limit: AcEvoElectronics,
    pub electronics_max_limit: AcEvoElectronics,
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
    pub performance_mode_name: SimString<33>,
    pub race_cut_gained_time_ms: i32,
    pub race_cut_current_delta: f32,
    pub session_state: AcEvoSessionState,
    pub timing_state: AcEvoTimingState,
    pub player_fps: i32,
    pub driver_name: SimString<33>,
    pub driver_surname: SimString<33>,
    pub car_model_name: SimString<33>,
    pub is_in_pit_box: bool,
    pub is_in_pit_lane: bool,
    pub is_valid_lap: bool,
    #[serde(skip)]
    pub car_coordinates: [[f32; 3]; 60],
    pub gap_ahead: f32,
    pub gap_behind: f32,
    pub active_cars: u8,
    pub fuel_estimated_laps: f32,
    pub assists_state: AcEvoAssistsState,
    pub max_fuel: f32,
    pub max_turbo_boost: f32,
    pub use_single_compound: bool,
}

impl From<SPageFileGraphicsEvo> for AcGraphicsData {
    fn from(r: SPageFileGraphicsEvo) -> Self {
        Self {
            packet_id: r.packet_id,
            status: r.status,
            rpm: r.rpm,
            tc_active: r.tc_active,
            abs_active: r.abs_active,
            esc_active: r.esc_active,
            is_engine_running: r.is_engine_running,
            is_drs_available: r.is_drs_available,
            display_speed_kmh: r.display_speed_kmh,
            gear_int: r.gear_int,
            current_lap_time_ms: r.current_lap_time_ms,
            predicted_lap_time_ms: r.predicted_lap_time_ms,
            delta_time_ms: r.delta_time_ms,
            fuel_liter_current_quantity: r.fuel_liter_current_quantity,
            fuel_liter_current_quantity_percent: r.fuel_liter_current_quantity_percent,
            current_torque: r.current_torque,
            current_bhp: r.current_bhp,
            tyre_lf: AcEvoTyreState::from(r.tyre_lf),
            tyre_rf: AcEvoTyreState::from(r.tyre_rf),
            tyre_lr: AcEvoTyreState::from(r.tyre_lr),
            tyre_rr: AcEvoTyreState::from(r.tyre_rr),
            npos: r.npos,
            kers_charge_perc: r.kers_charge_perc,
            car_damage: AcEvoDamageState::from(r.car_damage),
            car_location: r.car_location,
            pit_info: AcEvoPitInfo::from(r.pit_info),
            fuel_liter_used: r.fuel_liter_used,
            fuel_liter_per_lap: r.fuel_liter_per_lap,
            instrumentation: AcEvoInstrumentation::from(r.instrumentation),
            electronics: AcEvoElectronics::from(r.electronics),
            electronics_min_limit: AcEvoElectronics::from(r.electronics_min_limit),
            electronics_max_limit: AcEvoElectronics::from(r.electronics_max_limit),
            total_lap_count: r.total_lap_count,
            current_pos: r.current_pos,
            total_drivers: r.total_drivers,
            last_laptime_ms: r.last_laptime_ms,
            best_laptime_ms: r.best_laptime_ms,
            flag: r.flag,
            global_flag: r.global_flag,
            max_gears: r.max_gears,
            engine_type: r.engine_type,
            has_kers: r.has_kers,
            is_last_lap: r.is_last_lap,
            performance_mode_name: to_str(r.performance_mode_name),
            race_cut_gained_time_ms: r.race_cut_gained_time_ms,
            race_cut_current_delta: r.race_cut_current_delta,
            session_state: AcEvoSessionState::from(r.session_state),
            timing_state: AcEvoTimingState::from(r.timing_state),
            player_fps: r.player_fps,
            driver_name: to_str(r.driver_name),
            driver_surname: to_str(r.driver_surname),
            car_model_name: to_str(r.car_model_name),
            is_in_pit_box: r.is_in_pit_box,
            is_in_pit_lane: r.is_in_pit_lane,
            is_valid_lap: r.is_valid_lap,
            car_coordinates: r.car_coordinates,
            gap_ahead: r.gap_ahead,
            gap_behind: r.gap_behind,
            active_cars: r.active_cars,
            fuel_estimated_laps: r.fuel_estimated_laps,
            assists_state: AcEvoAssistsState::from(r.assists_state),
            max_fuel: r.max_fuel,
            max_turbo_boost: r.max_turbo_boost,
            use_single_compound: r.use_single_compound,
        }
    }
}

// ── Static ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AcStaticData {
    pub sm_version: SimString<15>,
    pub ac_evo_version: SimString<15>,
    pub session: i32,
    pub session_name: SimString<33>,
    pub event_id: u8,
    pub session_id: u8,
    pub starting_grip: i32,
    pub starting_ambient_temperature_c: f32,
    pub starting_ground_temperature_c: f32,
    pub is_static_weather: bool,
    pub is_timed_race: bool,
    pub is_online: bool,
    pub number_of_sessions: i32,
    pub nation: SimString<33>,
    pub longitude: f32,
    pub latitude: f32,
    pub track: SimString<33>,
    pub track_configuration: SimString<33>,
    pub track_length_m: f32,
}

impl From<SPageFileStaticEvo> for AcStaticData {
    fn from(r: SPageFileStaticEvo) -> Self {
        Self {
            sm_version: to_str(r.sm_version),
            ac_evo_version: to_str(r.ac_evo_version),
            session: r.session,
            session_name: to_str(r.session_name),
            event_id: r.event_id,
            session_id: r.session_id,
            starting_grip: r.starting_grip,
            starting_ambient_temperature_c: r.starting_ambient_temperature_c,
            starting_ground_temperature_c: r.starting_ground_temperature_c,
            is_static_weather: r.is_static_weather,
            is_timed_race: r.is_timed_race,
            is_online: r.is_online,
            number_of_sessions: r.number_of_sessions,
            nation: to_str(r.nation),
            longitude: r.longitude,
            latitude: r.latitude,
            track: to_str(r.track),
            track_configuration: to_str(r.track_configuration),
            track_length_m: r.track_length_m,
        }
    }
}
