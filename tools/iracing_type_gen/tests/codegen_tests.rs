use iracing_type_gen::{camel_to_snake, catalogue_to_toml, generate, merge_defs, parse_catalogue};

#[test]
fn camel_to_snake_simple() {
    assert_eq!(camel_to_snake("RPM"), "rpm");
    assert_eq!(camel_to_snake("Speed"), "speed");
    assert_eq!(camel_to_snake("Gear"), "gear");
}

#[test]
fn camel_to_snake_compound() {
    assert_eq!(camel_to_snake("CarIdxLapDistPct"), "car_idx_lap_dist_pct");
    assert_eq!(camel_to_snake("FuelLevelPct"), "fuel_level_pct");
    assert_eq!(camel_to_snake("OnPitRoad"), "on_pit_road");
}

#[test]
fn camel_to_snake_digit_word_boundary() {
    assert_eq!(camel_to_snake("CarIdxF2Time"), "car_idx_f2_time");
    assert_eq!(camel_to_snake("CarIdxP2P_Status"), "car_idx_p2p_status");
    assert_eq!(camel_to_snake("CarIdxP2P_Count"), "car_idx_p2p_count");
}

#[test]
fn camel_to_snake_acronym_overrides() {
    assert_eq!(camel_to_snake("BrakeABSactive"), "brake_abs_active");
}

#[test]
fn camel_to_snake_corner_prefixes() {
    assert_eq!(camel_to_snake("LFtempCL"), "lf_temp_cl");
    assert_eq!(camel_to_snake("LFtempCM"), "lf_temp_cm");
    assert_eq!(camel_to_snake("RRwearR"), "rr_wear_r");
    assert_eq!(camel_to_snake("LRshockDefl_ST"), "lr_shock_defl_st");
    assert_eq!(camel_to_snake("RFcoldPressure"), "rf_cold_pressure");
    assert_eq!(camel_to_snake("LFbrakeLinePress"), "lf_brake_line_press");
    assert_eq!(camel_to_snake("LFodometer"), "lf_odometer");
    assert_eq!(camel_to_snake("LFshockVel"), "lf_shock_vel");
}

#[test]
fn generate_f32_field() {
    let toml = r#"
[[var]]
name = "RPM"
type = "f32"
unit = "rev/min"
desc = "Engine revolutions per minute"
"#;
    let output = generate(toml);
    assert!(output.contains("pub rpm: f32,"));
    assert!(output.contains("read_unaligned"));
    assert!(output.contains("as *const f32"));
    assert!(output.contains("rev/min"));
}

#[test]
fn generate_array_field() {
    let toml = r#"
[[var]]
name = "CarIdxLapDistPct"
type = "f32"
count = 64
unit = ""
desc = "Lap distance pct per car"
"#;
    let output = generate(toml);
    assert!(output.contains("pub car_idx_lap_dist_pct: Vec<f32>,"));
    assert!(output.contains("read_unaligned"));
    assert!(output.contains("as *const f32"));
    assert!(output.contains("Vec::new()"));
}

#[test]
fn generate_i32_field() {
    let toml = r#"
[[var]]
name = "Gear"
type = "i32"
unit = ""
desc = "Gear number"
"#;
    let output = generate(toml);
    assert!(output.contains("pub gear: i32,"));
    assert!(output.contains("read_unaligned"));
    assert!(output.contains("as *const i32"));
}

#[test]
fn generate_bool_field() {
    let toml = r#"
[[var]]
name = "OnPitRoad"
type = "bool"
unit = ""
desc = "Is on pit road"
"#;
    let output = generate(toml);
    assert!(output.contains("pub on_pit_road: bool,"));
    assert!(output.contains("read_unaligned"));
    assert!(output.contains("!= 0"));
}

/// The catalogue is a union over cars, so the variables of the car that
/// happens to be in the session must not narrow it. This is the whole point
/// of merging: a regeneration in a GTP car used to delete `dcThrottleShape`,
/// which only other cars publish.
#[test]
fn merge_keeps_variables_the_session_does_not_declare() {
    let catalogue = parse_catalogue(
        r#"
[[var]]
name = "dcThrottleShape"
type = "f32"
desc = "In car throttle shape adjustment"
"#,
    )
    .expect("catalogue");

    let session = parse_catalogue(
        r#"
[[var]]
name = "EnergyERSBatteryPct"
type = "f32"
unit = "%"
desc = "Engine ERS battery charge as a percent"
"#,
    )
    .expect("session");

    let (merged, report) = merge_defs(&catalogue, &session);

    let names: Vec<&str> = merged.iter().map(|var| var.name.as_str()).collect();
    assert_eq!(names, ["EnergyERSBatteryPct", "dcThrottleShape"]);
    assert_eq!(report.added, ["EnergyERSBatteryPct"]);
    assert_eq!(report.absent, ["dcThrottleShape"]);
    assert!(report.redefined.is_empty());
}

/// The sim is authoritative on shape: a variable whose type or element count
/// changed in an iRacing update takes the session's version, not the stale one.
#[test]
fn merge_takes_the_session_shape_for_a_known_variable() {
    let catalogue = parse_catalogue(
        r#"
[[var]]
name = "CarIdxLapDistPct"
type = "f32"
count = 64
unit = "%"
desc = "old text"
"#,
    )
    .expect("catalogue");

    let session = parse_catalogue(
        r#"
[[var]]
name = "CarIdxLapDistPct"
type = "f32"
count = 72
unit = "%"
desc = "Percentage distance around lap by car index"
"#,
    )
    .expect("session");

    let (merged, report) = merge_defs(&catalogue, &session);

    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].count, 72);
    assert_eq!(
        merged[0].desc,
        "Percentage distance around lap by car index"
    );
    assert_eq!(report.redefined, ["CarIdxLapDistPct"]);
    assert!(report.added.is_empty());
}

/// Shared memory leaves the description empty for plenty of variables. An
/// empty string from the session is absence, not a correction, so a
/// description the catalogue already carries survives.
#[test]
fn merge_does_not_blank_a_description_with_an_empty_one() {
    let catalogue = parse_catalogue(
        r#"
[[var]]
name = "Speed"
type = "f32"
unit = "m/s"
desc = "GPS vehicle speed"
"#,
    )
    .expect("catalogue");

    let session = parse_catalogue(
        r#"
[[var]]
name = "Speed"
type = "f32"
"#,
    )
    .expect("session");

    let (merged, _) = merge_defs(&catalogue, &session);

    assert_eq!(merged[0].desc, "GPS vehicle speed");
    assert_eq!(merged[0].unit, "m/s");
}

/// The catalogue round-trips, so a merge can be written back and read again
/// without the descriptions or array counts drifting.
#[test]
fn catalogue_round_trips_through_toml() {
    let source = r#"
[[var]]
name = "CarIdxLapDistPct"
type = "f32"
count = 72
unit = "%"
desc = "Percentage \"distance\" around lap"

[[var]]
name = "OnPitRoad"
type = "bool"
desc = "On pit road between the cones"
"#;

    let parsed = parse_catalogue(source).expect("parse");
    let reparsed = parse_catalogue(&catalogue_to_toml(&parsed)).expect("reparse");

    assert_eq!(reparsed.len(), 2);
    assert_eq!(reparsed[0].count, 72);
    assert_eq!(reparsed[0].desc, "Percentage \"distance\" around lap");
    assert_eq!(reparsed[1].count, 1);
    assert_eq!(reparsed[1].type_, "bool");
}

/// An absent catalogue is an empty one, so the first run bootstraps from the
/// session instead of failing.
#[test]
fn parse_catalogue_treats_an_empty_file_as_empty() {
    assert!(parse_catalogue("").expect("empty").is_empty());
    assert!(parse_catalogue("   \n").expect("blank").is_empty());
}
