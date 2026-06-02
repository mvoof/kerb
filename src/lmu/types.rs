//! Public mirror types for LMU shared-memory data.
//!
//! These types shadow the internal `#[repr(C, packed)]` structs from `structs.rs`
//! and are safe to hold, clone, and serialize without packed-field UB concerns.

use crate::lmu::structs::{
    rF2Extended, rF2ScoringInfo, rF2VehicleScoring, rF2VehicleTelemetry, rF2Wheel,
};
use crate::sim_string::SimString;

pub use crate::lmu::structs::RF2_MAX_VEHICLES;

// ── LmuWheelData ─────────────────────────────────────────────────────────────

/// Public mirror of `rF2Wheel` — per-wheel physics data.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LmuWheelData {
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
    pub terrain_name: SimString<16>,
    pub surface_type: u8,
    pub flat: u8,
    pub detached: u8,
    pub static_unbalance: u8,
    pub vertical_tire_deflection: f64,
    pub wheel_ylocation: f64,
    pub toe: f64,
    pub tire_carcass_temperature: f64,
    pub tire_inner_layer_temperature: [f64; 3],
}

impl Default for LmuWheelData {
    fn default() -> Self {
        Self {
            suspension_deflection: 0.0,
            ride_height: 0.0,
            susp_force: 0.0,
            brake_temp: 0.0,
            brake_pressure: 0.0,
            rotation: 0.0,
            lateral_patch_vel: 0.0,
            longitudinal_patch_vel: 0.0,
            lateral_ground_vel: 0.0,
            longitudinal_ground_vel: 0.0,
            camber: 0.0,
            lateral_force: 0.0,
            longitudinal_force: 0.0,
            tire_load: 0.0,
            grip_fract: 0.0,
            pressure: 0.0,
            temperature: [0.0; 3],
            wear: 0.0,
            terrain_name: SimString::default(),
            surface_type: 0,
            flat: 0,
            detached: 0,
            static_unbalance: 0,
            vertical_tire_deflection: 0.0,
            wheel_ylocation: 0.0,
            toe: 0.0,
            tire_carcass_temperature: 0.0,
            tire_inner_layer_temperature: [0.0; 3],
        }
    }
}

impl From<rF2Wheel> for LmuWheelData {
    fn from(raw: rF2Wheel) -> Self {
        let terrain_name = raw.terrain_name;
        Self {
            suspension_deflection: raw.suspension_deflection,
            ride_height: raw.ride_height,
            susp_force: raw.susp_force,
            brake_temp: raw.brake_temp,
            brake_pressure: raw.brake_pressure,
            rotation: raw.rotation,
            lateral_patch_vel: raw.lateral_patch_vel,
            longitudinal_patch_vel: raw.longitudinal_patch_vel,
            lateral_ground_vel: raw.lateral_ground_vel,
            longitudinal_ground_vel: raw.longitudinal_ground_vel,
            camber: raw.camber,
            lateral_force: raw.lateral_force,
            longitudinal_force: raw.longitudinal_force,
            tire_load: raw.tire_load,
            grip_fract: raw.grip_fract,
            pressure: raw.pressure,
            temperature: raw.temperature,
            wear: raw.wear,
            terrain_name: SimString::from_bytes(&terrain_name),
            surface_type: raw.surface_type,
            flat: raw.flat,
            detached: raw.detached,
            static_unbalance: raw.static_unbalance,
            vertical_tire_deflection: raw.vertical_tire_deflection,
            wheel_ylocation: raw.wheel_ylocation,
            toe: raw.toe,
            tire_carcass_temperature: raw.tire_carcass_temperature,
            tire_inner_layer_temperature: raw.tire_inner_layer_temperature,
        }
    }
}

// ── LmuVehicleTelemetry ───────────────────────────────────────────────────────

/// Public mirror of `rF2VehicleTelemetry` — per-vehicle telemetry.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LmuVehicleTelemetry {
    pub id: i32,
    pub delta_time: f64,
    pub elapsed_time: f64,
    pub lap_number: i32,
    pub lap_start_et: f64,
    pub vehicle_name: SimString<64>,
    pub track_name: SimString<64>,
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
    pub front_tire_compound_name: SimString<18>,
    pub rear_tire_compound_name: SimString<18>,
    pub speed_limiter_available: u8,
    pub anti_stall_activated: u8,
    pub visual_steering_wheel_range: f32,
    pub rear_brake_bias: f64,
    pub turbo_boost_pressure: f64,
    pub physics_to_graphics_offset: [f32; 3],
    pub physical_steering_wheel_range: f32,
    pub wheels: [LmuWheelData; 4],
}

impl Default for LmuVehicleTelemetry {
    fn default() -> Self {
        Self {
            id: 0,
            delta_time: 0.0,
            elapsed_time: 0.0,
            lap_number: 0,
            lap_start_et: 0.0,
            vehicle_name: SimString::default(),
            track_name: SimString::default(),
            pos: [0.0; 3],
            local_vel: [0.0; 3],
            local_accel: [0.0; 3],
            ori: [[0.0; 3]; 3],
            local_rot: [0.0; 3],
            local_rot_accel: [0.0; 3],
            gear: 0,
            engine_rpm: 0.0,
            engine_water_temp: 0.0,
            engine_oil_temp: 0.0,
            clutch_rpm: 0.0,
            unfiltered_throttle: 0.0,
            unfiltered_brake: 0.0,
            unfiltered_steering: 0.0,
            unfiltered_clutch: 0.0,
            filtered_throttle: 0.0,
            filtered_brake: 0.0,
            filtered_steering: 0.0,
            filtered_clutch: 0.0,
            steering_shaft_torque: 0.0,
            front3rd_deflection: 0.0,
            rear3rd_deflection: 0.0,
            front_wing_height: 0.0,
            front_ride_height: 0.0,
            rear_ride_height: 0.0,
            drag: 0.0,
            front_downforce: 0.0,
            rear_downforce: 0.0,
            fuel: 0.0,
            engine_max_rpm: 0.0,
            scheduled_stops: 0,
            overheating: 0,
            detached: 0,
            headlights: 0,
            dent_severity: [0; 8],
            last_impact_et: 0.0,
            last_impact_magnitude: 0.0,
            last_impact_pos: [0.0; 3],
            engine_torque: 0.0,
            current_sector: 0,
            speed_limiter: 0,
            max_gears: 0,
            front_tire_compound_index: 0,
            rear_tire_compound_index: 0,
            fuel_capacity: 0.0,
            front_flap_activated: 0,
            rear_flap_activated: 0,
            rear_flap_legal_status: 0,
            ignition_starter: 0,
            front_tire_compound_name: SimString::default(),
            rear_tire_compound_name: SimString::default(),
            speed_limiter_available: 0,
            anti_stall_activated: 0,
            visual_steering_wheel_range: 0.0,
            rear_brake_bias: 0.0,
            turbo_boost_pressure: 0.0,
            physics_to_graphics_offset: [0.0; 3],
            physical_steering_wheel_range: 0.0,
            wheels: [
                LmuWheelData::default(),
                LmuWheelData::default(),
                LmuWheelData::default(),
                LmuWheelData::default(),
            ],
        }
    }
}

impl From<rF2VehicleTelemetry> for LmuVehicleTelemetry {
    fn from(raw: rF2VehicleTelemetry) -> Self {
        let vehicle_name = raw.vehicle_name;
        let track_name = raw.track_name;
        let front_tire_compound_name = raw.front_tire_compound_name;
        let rear_tire_compound_name = raw.rear_tire_compound_name;
        let wheels_raw = raw.wheels;
        Self {
            id: raw.id,
            delta_time: raw.delta_time,
            elapsed_time: raw.elapsed_time,
            lap_number: raw.lap_number,
            lap_start_et: raw.lap_start_et,
            vehicle_name: SimString::from_bytes(&vehicle_name),
            track_name: SimString::from_bytes(&track_name),
            pos: raw.pos,
            local_vel: raw.local_vel,
            local_accel: raw.local_accel,
            ori: raw.ori,
            local_rot: raw.local_rot,
            local_rot_accel: raw.local_rot_accel,
            gear: raw.gear,
            engine_rpm: raw.engine_rpm,
            engine_water_temp: raw.engine_water_temp,
            engine_oil_temp: raw.engine_oil_temp,
            clutch_rpm: raw.clutch_rpm,
            unfiltered_throttle: raw.unfiltered_throttle,
            unfiltered_brake: raw.unfiltered_brake,
            unfiltered_steering: raw.unfiltered_steering,
            unfiltered_clutch: raw.unfiltered_clutch,
            filtered_throttle: raw.filtered_throttle,
            filtered_brake: raw.filtered_brake,
            filtered_steering: raw.filtered_steering,
            filtered_clutch: raw.filtered_clutch,
            steering_shaft_torque: raw.steering_shaft_torque,
            front3rd_deflection: raw.front3rd_deflection,
            rear3rd_deflection: raw.rear3rd_deflection,
            front_wing_height: raw.front_wing_height,
            front_ride_height: raw.front_ride_height,
            rear_ride_height: raw.rear_ride_height,
            drag: raw.drag,
            front_downforce: raw.front_downforce,
            rear_downforce: raw.rear_downforce,
            fuel: raw.fuel,
            engine_max_rpm: raw.engine_max_rpm,
            scheduled_stops: raw.scheduled_stops,
            overheating: raw.overheating,
            detached: raw.detached,
            headlights: raw.headlights,
            dent_severity: raw.dent_severity,
            last_impact_et: raw.last_impact_et,
            last_impact_magnitude: raw.last_impact_magnitude,
            last_impact_pos: raw.last_impact_pos,
            engine_torque: raw.engine_torque,
            current_sector: raw.current_sector,
            speed_limiter: raw.speed_limiter,
            max_gears: raw.max_gears,
            front_tire_compound_index: raw.front_tire_compound_index,
            rear_tire_compound_index: raw.rear_tire_compound_index,
            fuel_capacity: raw.fuel_capacity,
            front_flap_activated: raw.front_flap_activated,
            rear_flap_activated: raw.rear_flap_activated,
            rear_flap_legal_status: raw.rear_flap_legal_status,
            ignition_starter: raw.ignition_starter,
            front_tire_compound_name: SimString::from_bytes(&front_tire_compound_name),
            rear_tire_compound_name: SimString::from_bytes(&rear_tire_compound_name),
            speed_limiter_available: raw.speed_limiter_available,
            anti_stall_activated: raw.anti_stall_activated,
            visual_steering_wheel_range: raw.visual_steering_wheel_range,
            rear_brake_bias: raw.rear_brake_bias,
            turbo_boost_pressure: raw.turbo_boost_pressure,
            physics_to_graphics_offset: raw.physics_to_graphics_offset,
            physical_steering_wheel_range: raw.physical_steering_wheel_range,
            wheels: [
                LmuWheelData::from(wheels_raw[0]),
                LmuWheelData::from(wheels_raw[1]),
                LmuWheelData::from(wheels_raw[2]),
                LmuWheelData::from(wheels_raw[3]),
            ],
        }
    }
}

// ── LmuVehicleScoring ─────────────────────────────────────────────────────────

/// Public mirror of `rF2VehicleScoring` — per-vehicle scoring data.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LmuVehicleScoring {
    pub id: i32,
    pub driver_name: SimString<32>,
    pub vehicle_name: SimString<64>,
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
    pub vehicle_class: SimString<32>,
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
    pub pit_group: SimString<24>,
    pub flag: u8,
    pub under_yellow: u8,
    pub count_laps_invalid_flags: u8,
    pub in_garage_stall: u8,
    pub upgrade_pack: SimString<16>,
    pub pit_lap_dist: f32,
    pub best_lap_sector1: f32,
    pub best_lap_sector2: f32,
}

impl Default for LmuVehicleScoring {
    fn default() -> Self {
        Self {
            id: 0,
            driver_name: SimString::default(),
            vehicle_name: SimString::default(),
            total_laps: 0,
            sector: 0,
            finish_status: 0,
            lap_dist: 0.0,
            path_lateral: 0.0,
            track_edge: 0.0,
            best_sector1: 0.0,
            best_sector2: 0.0,
            best_lap_time: 0.0,
            last_sector1: 0.0,
            last_sector2: 0.0,
            last_lap_time: 0.0,
            cur_sector1: 0.0,
            cur_sector2: 0.0,
            num_pitstops: 0,
            num_penalties: 0,
            is_player: 0,
            control: 0,
            in_pits: 0,
            place: 0,
            vehicle_class: SimString::default(),
            time_behind_next: 0.0,
            laps_behind_next: 0,
            time_behind_leader: 0.0,
            laps_behind_leader: 0,
            lap_start_et: 0.0,
            pos: [0.0; 3],
            local_vel: [0.0; 3],
            local_accel: [0.0; 3],
            ori: [[0.0; 3]; 3],
            local_rot: [0.0; 3],
            local_rot_accel: [0.0; 3],
            headlights: 0,
            pit_state: 0,
            server_scored: 0,
            individual_phase: 0,
            qualification: 0,
            time_into_lap: 0.0,
            estimated_lap_time: 0.0,
            pit_group: SimString::default(),
            flag: 0,
            under_yellow: 0,
            count_laps_invalid_flags: 0,
            in_garage_stall: 0,
            upgrade_pack: SimString::default(),
            pit_lap_dist: 0.0,
            best_lap_sector1: 0.0,
            best_lap_sector2: 0.0,
        }
    }
}

impl From<rF2VehicleScoring> for LmuVehicleScoring {
    fn from(raw: rF2VehicleScoring) -> Self {
        let driver_name = raw.driver_name;
        let vehicle_name = raw.vehicle_name;
        let vehicle_class = raw.vehicle_class;
        let pit_group = raw.pit_group;
        let upgrade_pack = raw.upgrade_pack;
        Self {
            id: raw.id,
            driver_name: SimString::from_bytes(&driver_name),
            vehicle_name: SimString::from_bytes(&vehicle_name),
            total_laps: raw.total_laps,
            sector: raw.sector,
            finish_status: raw.finish_status,
            lap_dist: raw.lap_dist,
            path_lateral: raw.path_lateral,
            track_edge: raw.track_edge,
            best_sector1: raw.best_sector1,
            best_sector2: raw.best_sector2,
            best_lap_time: raw.best_lap_time,
            last_sector1: raw.last_sector1,
            last_sector2: raw.last_sector2,
            last_lap_time: raw.last_lap_time,
            cur_sector1: raw.cur_sector1,
            cur_sector2: raw.cur_sector2,
            num_pitstops: raw.num_pitstops,
            num_penalties: raw.num_penalties,
            is_player: raw.is_player,
            control: raw.control,
            in_pits: raw.in_pits,
            place: raw.place,
            vehicle_class: SimString::from_bytes(&vehicle_class),
            time_behind_next: raw.time_behind_next,
            laps_behind_next: raw.laps_behind_next,
            time_behind_leader: raw.time_behind_leader,
            laps_behind_leader: raw.laps_behind_leader,
            lap_start_et: raw.lap_start_et,
            pos: raw.pos,
            local_vel: raw.local_vel,
            local_accel: raw.local_accel,
            ori: raw.ori,
            local_rot: raw.local_rot,
            local_rot_accel: raw.local_rot_accel,
            headlights: raw.headlights,
            pit_state: raw.pit_state,
            server_scored: raw.server_scored,
            individual_phase: raw.individual_phase,
            qualification: raw.qualification,
            time_into_lap: raw.time_into_lap,
            estimated_lap_time: raw.estimated_lap_time,
            pit_group: SimString::from_bytes(&pit_group),
            flag: raw.flag,
            under_yellow: raw.under_yellow,
            count_laps_invalid_flags: raw.count_laps_invalid_flags,
            in_garage_stall: raw.in_garage_stall,
            upgrade_pack: SimString::from_bytes(&upgrade_pack),
            pit_lap_dist: raw.pit_lap_dist,
            best_lap_sector1: raw.best_lap_sector1,
            best_lap_sector2: raw.best_lap_sector2,
        }
    }
}

// ── LmuScoringInfo ────────────────────────────────────────────────────────────

/// Public mirror of `rF2ScoringInfo` — session-wide scoring data.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LmuScoringInfo {
    pub track_name: SimString<64>,
    pub session: i32,
    pub current_et: f64,
    pub end_et: f64,
    pub max_laps: i32,
    pub lap_dist: f64,
    pub result_name: SimString<64>,
    pub num_vehicles: i32,
    pub game_phase: u8,
    pub yellow_flag_state: i8,
    pub sector_flag: [i8; 3],
    pub start_light: u8,
    pub num_red_lights: u8,
    pub in_realtime: u8,
    pub player_name: SimString<32>,
    pub plr_file_name: SimString<64>,
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
    pub server_name: SimString<32>,
    pub start_et: f32,
    pub avg_path_wetness: f64,
}

impl Default for LmuScoringInfo {
    fn default() -> Self {
        Self {
            track_name: SimString::default(),
            session: 0,
            current_et: 0.0,
            end_et: 0.0,
            max_laps: 0,
            lap_dist: 0.0,
            result_name: SimString::default(),
            num_vehicles: 0,
            game_phase: 0,
            yellow_flag_state: 0,
            sector_flag: [0; 3],
            start_light: 0,
            num_red_lights: 0,
            in_realtime: 0,
            player_name: SimString::default(),
            plr_file_name: SimString::default(),
            dark_cloud: 0.0,
            raining: 0.0,
            ambient_temp: 0.0,
            track_temp: 0.0,
            wind: [0.0; 3],
            min_path_wetness: 0.0,
            max_path_wetness: 0.0,
            game_mode: 0,
            is_password_protected: 0,
            server_port: 0,
            server_public_ip: 0,
            max_players: 0,
            server_name: SimString::default(),
            start_et: 0.0,
            avg_path_wetness: 0.0,
        }
    }
}

impl From<rF2ScoringInfo> for LmuScoringInfo {
    fn from(raw: rF2ScoringInfo) -> Self {
        let track_name = raw.track_name;
        let result_name = raw.result_name;
        let player_name = raw.player_name;
        let plr_file_name = raw.plr_file_name;
        let server_name = raw.server_name;
        Self {
            track_name: SimString::from_bytes(&track_name),
            session: raw.session,
            current_et: raw.current_et,
            end_et: raw.end_et,
            max_laps: raw.max_laps,
            lap_dist: raw.lap_dist,
            result_name: SimString::from_bytes(&result_name),
            num_vehicles: raw.num_vehicles,
            game_phase: raw.game_phase,
            yellow_flag_state: raw.yellow_flag_state,
            sector_flag: raw.sector_flag,
            start_light: raw.start_light,
            num_red_lights: raw.num_red_lights,
            in_realtime: raw.in_realtime,
            player_name: SimString::from_bytes(&player_name),
            plr_file_name: SimString::from_bytes(&plr_file_name),
            dark_cloud: raw.dark_cloud,
            raining: raw.raining,
            ambient_temp: raw.ambient_temp,
            track_temp: raw.track_temp,
            wind: raw.wind,
            min_path_wetness: raw.min_path_wetness,
            max_path_wetness: raw.max_path_wetness,
            game_mode: raw.game_mode,
            is_password_protected: raw.is_password_protected,
            server_port: raw.server_port,
            server_public_ip: raw.server_public_ip,
            max_players: raw.max_players,
            server_name: SimString::from_bytes(&server_name),
            start_et: raw.start_et,
            avg_path_wetness: raw.avg_path_wetness,
        }
    }
}

// ── LmuExtended ───────────────────────────────────────────────────────────────

/// Public mirror of `rF2Extended` — plugin/session metadata.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LmuExtended {
    pub version_update_begin: u32,
    pub version_update_end: u32,
    pub bytes_in_version: i32,
    pub bytes_in_extended: i32,
    pub physics_to_graphics_offset: [f32; 3],
    pub is_plugin_enabled: u8,
    pub direct_memory_access_enabled: u8,
    pub session_started: u8,
    pub phys_avg_thread_time_ms: f64,
}

impl Default for LmuExtended {
    fn default() -> Self {
        Self {
            version_update_begin: 0,
            version_update_end: 0,
            bytes_in_version: 0,
            bytes_in_extended: 0,
            physics_to_graphics_offset: [0.0; 3],
            is_plugin_enabled: 0,
            direct_memory_access_enabled: 0,
            session_started: 0,
            phys_avg_thread_time_ms: 0.0,
        }
    }
}

impl From<rF2Extended> for LmuExtended {
    fn from(raw: rF2Extended) -> Self {
        Self {
            version_update_begin: raw.version_update_begin,
            version_update_end: raw.version_update_end,
            bytes_in_version: raw.bytes_in_version,
            bytes_in_extended: raw.bytes_in_extended,
            physics_to_graphics_offset: raw.physics_to_graphics_offset,
            is_plugin_enabled: raw.is_plugin_enabled,
            direct_memory_access_enabled: raw.direct_memory_access_enabled,
            session_started: raw.session_started,
            phys_avg_thread_time_ms: raw.phys_avg_thread_time_ms,
        }
    }
}

// ── LmuFrame ──────────────────────────────────────────────────────────────────

/// Public point-in-time snapshot of all LMU shared-memory data.
///
/// Holds fixed arrays for up to [`RF2_MAX_VEHICLES`] vehicles.
/// Use [`vehicles_telemetry()`](LmuFrame::vehicles_telemetry) and
/// [`vehicles_scoring()`](LmuFrame::vehicles_scoring) to get slices of valid entries.
#[derive(Debug, Clone)]
pub struct LmuFrame {
    pub vehicles_telemetry: [LmuVehicleTelemetry; RF2_MAX_VEHICLES],
    pub vehicles_scoring: [LmuVehicleScoring; RF2_MAX_VEHICLES],
    pub num_vehicles: usize,
    pub scoring_info: LmuScoringInfo,
    pub extended: LmuExtended,
}

impl serde::Serialize for LmuFrame {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("LmuFrame", 5)?;
        s.serialize_field(
            "vehicles_telemetry",
            &self.vehicles_telemetry[..self.num_vehicles],
        )?;
        s.serialize_field(
            "vehicles_scoring",
            &self.vehicles_scoring[..self.num_vehicles],
        )?;
        s.serialize_field("num_vehicles", &self.num_vehicles)?;
        s.serialize_field("scoring_info", &self.scoring_info)?;
        s.serialize_field("extended", &self.extended)?;
        s.end()
    }
}

impl Default for LmuFrame {
    fn default() -> Self {
        // SAFETY: LmuFrame only contains numeric types, SimString<N> ([u8;N]),
        // and arrays thereof — all valid when zeroed.
        unsafe { std::mem::zeroed() }
    }
}

impl LmuFrame {
    /// Slice of valid vehicle telemetry entries (length = `num_vehicles`).
    pub fn vehicles_telemetry(&self) -> &[LmuVehicleTelemetry] {
        &self.vehicles_telemetry[..self.num_vehicles]
    }

    /// Slice of valid vehicle scoring entries (length = `num_vehicles`).
    pub fn vehicles_scoring(&self) -> &[LmuVehicleScoring] {
        &self.vehicles_scoring[..self.num_vehicles]
    }

    /// Returns a reference to the player's telemetry entry, or `None` if not found.
    pub fn player_telemetry(&self) -> Option<&LmuVehicleTelemetry> {
        let idx = self.vehicles_scoring[..self.num_vehicles]
            .iter()
            .position(|v| v.is_player != 0)?;
        Some(&self.vehicles_telemetry[idx])
    }
}
