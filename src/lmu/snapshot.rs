use crate::lmu::connection::LmuFrame;
use crate::{TelemetryValue, VarMeta};
use std::collections::HashMap;

/// Extracts the player's telemetry from an [`LmuFrame`] into a flat `HashMap`.
///
/// All fields from `rF2VehicleTelemetry` are included automatically via the
/// `#[derive(Snapshot)]` macro — no manual field list needed.
pub fn build_snapshot(f: &LmuFrame) -> HashMap<String, TelemetryValue> {
    f.player_telemetry().to_snapshot()
}

/// Returns metadata for every variable in the snapshot.
///
/// Since field names come from the struct directly, this just maps snapshot
/// keys to generic metadata entries.
pub fn var_list() -> Vec<VarMeta> {
    // Build one snapshot from a zeroed frame to enumerate keys
    let zeroed: crate::lmu::structs::rF2VehicleTelemetry = unsafe { std::mem::zeroed() };
    zeroed
        .to_snapshot()
        .into_keys()
        .map(|name| VarMeta {
            type_name: "f64".into(),
            unit: "".into(),
            desc: "".into(),
            count: 1,
            name,
        })
        .collect()
}
