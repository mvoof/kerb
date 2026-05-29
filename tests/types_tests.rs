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
        type_name: "f32".into(),
        unit: "rev/min".into(),
        desc: "Engine RPM".into(),
        count: 1,
    };
    assert_eq!(m.name, "RPM");
}
