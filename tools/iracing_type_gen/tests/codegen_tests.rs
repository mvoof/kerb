use iracing_type_gen::{camel_to_snake, generate};

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
