//! Session-data diagnostics for overlay projects (iRacing only).
//!
//! Verifies that kerb exposes the session arrays an overlay needs
//! (drivers / sectors / results), prints scalar `get_value` lookups used by
//! computations, and reports per-car frame array lengths and bitfield values.
//!
//! Run with iRacing open: `cargo run --example session_diagnostics`

use kerb::{Connection, SimConnection};

const SCALAR_PATHS: &[&str] = &[
    "WeekendInfo.TrackDisplayName",
    "WeekendInfo.TrackLength",
    "DriverInfo.DriverCarIdx",
    "SessionInfo.CurrentSessionNum",
    "DriverInfo.Drivers[0].UserName",
    "DriverInfo.Drivers[0].CarClassID",
    "DriverInfo.Drivers[0].CarClassColor",
    "DriverInfo.Drivers[0].CarClassShortName",
    "DriverInfo.Drivers[0].CarClassEstLapTime",
    "SplitTimeInfo.Sectors[0].SectorStartPct",
    "SplitTimeInfo.Sectors[0].SectorNum",
    "SessionInfo.Sessions[0].ResultsPositions[0].CarIdx",
    "SessionInfo.Sessions[0].ResultsPositions[0].Lap",
    "SessionInfo.Sessions[0].ResultsPositions[0].Time",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = match SimConnection::connect() {
        Ok(Connection::IRacing(conn)) => conn,
        Ok(_) => {
            println!("A different simulator is running — this diagnostic is iRacing-only.");
            return Ok(());
        }
        Err(error) => {
            println!("No simulator found: {error}");
            return Ok(());
        }
    };

    println!("--- session diagnostics ---");

    if let Some(yaml) = conn.session_yaml() {
        println!("session_yaml len = {} bytes", yaml.len());

        match serde_yaml::from_str::<serde_yaml::Value>(&yaml) {
            Ok(root) => {
                let drivers = root
                    .get("DriverInfo")
                    .and_then(|info| info.get("Drivers"))
                    .and_then(|drivers| drivers.as_sequence());
                println!("Drivers len = {:?}", drivers.map(|seq| seq.len()));

                let sectors = root
                    .get("SplitTimeInfo")
                    .and_then(|info| info.get("Sectors"))
                    .and_then(|sectors| sectors.as_sequence());
                println!("Sectors len = {:?}", sectors.map(|seq| seq.len()));

                let results = root
                    .get("SessionInfo")
                    .and_then(|info| info.get("Sessions"))
                    .and_then(|sessions| sessions.as_sequence())
                    .and_then(|sessions| sessions.first())
                    .and_then(|session| session.get("ResultsPositions"))
                    .and_then(|results| results.as_sequence());
                println!(
                    "Session[0].ResultsPositions len = {:?}",
                    results.map(|seq| seq.len())
                );
            }
            Err(error) => {
                println!("!! YAML PARSE FAILED: {error}");
            }
        }
    } else {
        println!("session_yaml() = None");
    }

    if let Some(session) = conn.session_info() {
        for path in SCALAR_PATHS {
            println!("{path:52} = {:?}", session.get_value(path));
        }
    }

    if let Ok(frame) = conn.frame() {
        println!("car_idx_lap_dist_pct  len = {}", frame.car_idx_lap_dist_pct.len());
        println!("car_idx_position      len = {}", frame.car_idx_position.len());
        println!("car_idx_class_position len = {}", frame.car_idx_class_position.len());
        println!("car_idx_on_pit_road   len = {}", frame.car_idx_on_pit_road.len());
        println!("car_idx_est_time      len = {}", frame.car_idx_est_time.len());
        println!("car_idx_session_flags len = {}", frame.car_idx_session_flags.len());
        println!(
            "engine_warnings = {}  session_flags = {}  car_left_right = {}",
            frame.engine_warnings, frame.session_flags, frame.car_left_right
        );
    }

    println!("--- end diagnostics ---");

    Ok(())
}
