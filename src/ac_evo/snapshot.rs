use crate::ac_evo::connection::AcEvoFrame;
use crate::{TelemetryValue, VarMeta};
use std::collections::HashMap;

pub fn build_snapshot(f: &AcEvoFrame) -> HashMap<String, TelemetryValue> {
    let mut serializer = crate::serializer::TelemetrySerializer::new();
    let _ = serde::Serialize::serialize(&f.physics, &mut serializer);
    let _ = serde::Serialize::serialize(&f.graphics, &mut serializer);
    let _ = serde::Serialize::serialize(&f.static_data, &mut serializer);
    serializer.into_map()
}

pub fn var_list_snapshot() -> Vec<VarMeta> {
    vec![
        VarMeta {
            name: "packet_id".into(),
            type_name: "int",
            unit: "".into(),
            desc: "Physics tick counter".into(),
            count: 1,
        },
        VarMeta {
            name: "speed_kmh".into(),
            type_name: "float",
            unit: "km/h".into(),
            desc: "Vehicle speed".into(),
            count: 1,
        },
        VarMeta {
            name: "brake_bias".into(),
            type_name: "float",
            unit: "".into(),
            desc: "Front brake bias 0.0–1.0".into(),
            count: 1,
        },
        VarMeta {
            name: "status".into(),
            type_name: "int",
            unit: "".into(),
            desc: "Session status (0=off 2=live)".into(),
            count: 1,
        },
        VarMeta {
            name: "abs_level".into(),
            type_name: "int8",
            unit: "".into(),
            desc: "ABS setting level".into(),
            count: 1,
        },
        VarMeta {
            name: "tc_level".into(),
            type_name: "int8",
            unit: "".into(),
            desc: "TC setting level".into(),
            count: 1,
        },
        VarMeta {
            name: "current_lap_time_ms".into(),
            type_name: "int",
            unit: "ms".into(),
            desc: "Current lap time".into(),
            count: 1,
        },
        VarMeta {
            name: "session".into(),
            type_name: "int",
            unit: "".into(),
            desc: "Session type (ACEVO_SESSION_TYPE)".into(),
            count: 1,
        },
        VarMeta {
            name: "track".into(),
            type_name: "string",
            unit: "".into(),
            desc: "Track identifier".into(),
            count: 1,
        },
    ]
}
