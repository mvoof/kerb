use kerb::{TelemetryValue, VarMeta};

#[test]
fn telemetry_value_display_float() {
    let v = TelemetryValue::Float(123.456_f32);
    assert_eq!(v.to_string(), "123.4560");
}

#[test]
fn telemetry_value_display_bitfield() {
    let v = TelemetryValue::BitField(0xDEAD);
    assert_eq!(v.to_string(), "0x0000DEAD");
}

#[test]
fn var_meta_fields() {
    let m = VarMeta {
        name: "RPM".into(),
        type_name: "f32",
        unit: "rev/min".into(),
        desc: "Engine RPM".into(),
        count: 1,
    };
    assert_eq!(m.name, "RPM");
}

#[cfg(feature = "ac-evo")]
#[test]
fn test_ac_snapshot_non_empty() {
    use kerb::ac_evo::connection::AcEvoFrame;
    use kerb::ac_evo::snapshot::build_snapshot;
    let frame: AcEvoFrame = unsafe { std::mem::zeroed() };
    let snap = build_snapshot(&frame);
    assert!(!snap.is_empty());
    assert!(snap.contains_key("packet_id"));
}

#[test]
fn test_lmu_snapshot_non_empty() {
    use kerb::lmu::snapshot::build_snapshot;
    use kerb::lmu::types::LmuFrame;
    let mut frame = LmuFrame::default();
    frame.vehicles_scoring[0].is_player = 1;
    frame.num_vehicles = 1;
    let snap = build_snapshot(&frame);
    assert!(!snap.is_empty());
    assert!(snap.contains_key("id"));
}
