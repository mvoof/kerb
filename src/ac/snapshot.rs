use crate::ac::connection::AcFrame;
use crate::{TelemetryValue, VarMeta};
use std::collections::HashMap;

pub fn build_snapshot(f: &AcFrame) -> HashMap<String, TelemetryValue> {
    match f {
        AcFrame::Classic(f) => {
            let mut m = f.physics.to_snapshot();
            m.extend(f.graphics.to_snapshot());
            m.extend(f.static_data.to_snapshot());
            m
        }
        AcFrame::Evo(f) => {
            let mut m = f.physics.to_snapshot();
            m.extend(f.graphics.to_snapshot());
            m.extend(f.static_data.to_snapshot());
            m
        }
    }
}

pub fn var_list() -> Vec<VarMeta> {
    use crate::ac::structs::{SPageFileGraphics, SPageFilePhysics, SPageFileStatic};
    let mut keys: Vec<String> = unsafe {
        let p: SPageFilePhysics = std::mem::zeroed();
        let g: SPageFileGraphics = std::mem::zeroed();
        let s: SPageFileStatic = std::mem::zeroed();
        p.to_snapshot()
            .into_keys()
            .chain(g.to_snapshot().into_keys())
            .chain(s.to_snapshot().into_keys())
            .collect()
    };
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .map(|name| VarMeta {
            type_name: "f32".into(),
            unit: "".into(),
            desc: "".into(),
            count: 1,
            name,
        })
        .collect()
}
