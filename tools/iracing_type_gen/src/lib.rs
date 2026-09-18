use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct VarDef {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub desc: String,
}

fn default_count() -> u32 {
    1
}

#[derive(Deserialize)]
struct VarFile {
    var: Vec<VarDef>,
}

/// What a merge did to the catalogue, for the operator to read before committing.
#[derive(Default)]
pub struct MergeReport {
    /// Variables the session declared that the catalogue had never seen.
    pub added: Vec<String>,
    /// Variables whose type or element count the session declares differently.
    pub redefined: Vec<String>,
    /// Variables the catalogue holds that this car does not expose. They are
    /// kept — another car still publishes them — and listed only so a genuine
    /// removal by iRacing is not silently carried forever.
    pub absent: Vec<String>,
}

/// Fold the variables of one session into the catalogue.
///
/// The catalogue is the union over every car ever seen, because iRacing
/// declares only the variables the current car has: merging is what keeps a
/// regeneration in a GTP car from dropping the fields that only a formula car
/// or only a road car publishes. Entries are therefore never removed here.
///
/// Where both sides know a variable, the session wins on shape (type and
/// count) and on any description or unit it actually carries — the sim is
/// authoritative, and an empty string from it is absence, not a correction.
pub fn merge_defs(catalogue: &[VarDef], session: &[VarDef]) -> (Vec<VarDef>, MergeReport) {
    let mut merged: Vec<VarDef> = catalogue.to_vec();
    let mut report = MergeReport::default();

    for incoming in session {
        let existing = merged.iter_mut().find(|held| held.name == incoming.name);

        let Some(held) = existing else {
            report.added.push(incoming.name.clone());
            merged.push(incoming.clone());

            continue;
        };

        if held.type_ != incoming.type_ || held.count != incoming.count {
            report.redefined.push(incoming.name.clone());
            held.type_ = incoming.type_.clone();
            held.count = incoming.count;
        }

        if !incoming.unit.is_empty() {
            held.unit = incoming.unit.clone();
        }

        if !incoming.desc.is_empty() {
            held.desc = incoming.desc.clone();
        }
    }

    for held in &merged {
        if !session.iter().any(|incoming| incoming.name == held.name) {
            report.absent.push(held.name.clone());
        }
    }

    merged.sort_by(|left, right| left.name.cmp(&right.name));

    (merged, report)
}

/// Render the catalogue back as TOML, sorted by name.
pub fn catalogue_to_toml(vars: &[VarDef]) -> String {
    let mut out = String::new();

    out.push_str("# Catalogue of iRacing telemetry variables known to kerb.\n");
    out.push_str("# Source of truth for src/iracing/types.rs — see the README.\n");
    out.push_str("#\n");
    out.push_str("# The union over every car ever seen. iRacing declares only the variables the\n");
    out.push_str("# current car has, so entries are added by a merge and never removed by one;\n");
    out.push_str("# a variable absent from a session simply resolves to None at connect time.\n");

    for var in vars {
        out.push_str("\n[[var]]\n");
        out.push_str(&format!("name = {}\n", toml_string(&var.name)));
        out.push_str(&format!("type = {}\n", toml_string(&var.type_)));

        if var.count != 1 {
            out.push_str(&format!("count = {}\n", var.count));
        }

        if !var.unit.is_empty() {
            out.push_str(&format!("unit = {}\n", toml_string(&var.unit)));
        }

        if !var.desc.is_empty() {
            out.push_str(&format!("desc = {}\n", toml_string(&var.desc)));
        }
    }

    out
}

/// Descriptions come from shared memory and are not guaranteed to be free of
/// quotes or backslashes, so they are escaped rather than interpolated raw.
fn toml_string(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");

    format!("\"{escaped}\"")
}

/// Read a catalogue file. An absent file is an empty catalogue, so the first
/// run bootstraps it from the session instead of failing.
pub fn parse_catalogue(toml_str: &str) -> Result<Vec<VarDef>, toml::de::Error> {
    if toml_str.trim().is_empty() {
        return Ok(Vec::new());
    }

    let file: VarFile = toml::from_str(toml_str)?;

    Ok(file.var)
}

/// iRacing names that defeat mechanical CamelCase splitting (irregular
/// acronym/word boundaries). Checked before the generic conversion.
const SNAKE_OVERRIDES: &[(&str, &str)] = &[("BrakeABSactive", "brake_abs_active")];

/// Corner and axle codes that open a variable name. `LFtempCL` means
/// "LF temp CL", so the code is one word (`lf_temp_cl`) rather than the
/// generic split on the case change (`l_ftemp_cl`).
///
/// Longest first: `LFSHshockDefl` is an `LFSH` channel, and matching `LF`
/// against it would leave `lfs_hshock_defl`.
const CORNER_PREFIXES: &[&str] = &[
    "LFSH", "LRSH", "RFSH", "RRSH", "CF", "CR", "HF", "HR", "LF", "LR", "RF", "RR",
];

pub fn camel_to_snake(name: &str) -> String {
    if let Some((_, snake)) = SNAKE_OVERRIDES
        .iter()
        .find(|(original, _)| *original == name)
    {
        return (*snake).to_string();
    }

    for prefix in CORNER_PREFIXES {
        let rest = match name.strip_prefix(prefix) {
            Some(rest) => rest,
            None => continue,
        };

        if rest.starts_with(|c: char| c.is_lowercase()) {
            let mut out = prefix.to_lowercase();
            out.push('_');
            // Capitalize the first letter so the tail converts as a normal word.
            let mut tail_chars = rest.chars();
            let mut tail = String::new();
            if let Some(first) = tail_chars.next() {
                tail.extend(first.to_uppercase());
            }
            tail.extend(tail_chars);
            out.push_str(&camel_to_snake_generic(&tail));

            return out;
        }
    }

    camel_to_snake_generic(name)
}

fn camel_to_snake_generic(name: &str) -> String {
    let mut out = String::new();

    let chars: Vec<char> = name.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_uppercase();
            let prev_digit = i > 0 && chars[i - 1].is_ascii_digit();

            // `next_lower && prev_digit` splits "F2Time" → f2_time while
            // keeping acronym runs like "P2P_Status" → p2p_status intact.
            if i > 0 && (prev_lower || (next_lower && (prev_upper || prev_digit))) {
                out.push('_');
            }

            out.push(c.to_lowercase().next().unwrap());
        } else {
            out.push(c);
        }
    }

    out
}

fn rust_type(type_: &str, count: u32) -> String {
    if count > 1 {
        match type_ {
            "f32" => "Vec<f32>",
            "f64" => "Vec<f64>",
            "i32" => "Vec<i32>",
            "bool" => "Vec<bool>",
            _ => "Vec<i32>",
        }
        .to_string()
    } else {
        type_.to_string()
    }
}

fn raw_extract_expr(type_: &str, count: u32, field_name: &str) -> String {
    if count > 1 {
        if type_ == "bool" {
            format!(
                r#"match offsets.{} {{
                Some(ref off) => unsafe {{
                    let ptr = buf.add(off.offset);
                    let mut vec = Vec::with_capacity(off.count);
                    for idx in 0..off.count {{
                        vec.push(*ptr.add(idx) != 0);
                    }}
                    vec
                }},
                None => Vec::new(),
            }}"#,
                field_name
            )
        } else {
            let (cast, _default) = match type_ {
                "f32" => ("as *const f32", "0.0f32"),
                "f64" => ("as *const f64", "0.0f64"),
                _ => ("as *const i32", "0i32"),
            };
            format!(
                r#"match offsets.{} {{
                Some(ref off) => unsafe {{
                    let src = buf.add(off.offset) {};
                    (0..off.count).map(|i| std::ptr::read_unaligned(src.add(i))).collect()
                }},
                None => Vec::new(),
            }}"#,
                field_name, cast
            )
        }
    } else {
        match type_ {
            "f32" => format!(
                r#"match offsets.{} {{ Some(ref off) => unsafe {{ std::ptr::read_unaligned(buf.add(off.offset) as *const f32) }}, None => 0.0 }}"#,
                field_name
            ),
            "f64" => format!(
                r#"match offsets.{} {{ Some(ref off) => unsafe {{ std::ptr::read_unaligned(buf.add(off.offset) as *const f64) }}, None => 0.0 }}"#,
                field_name
            ),
            "bool" => format!(
                r#"match offsets.{} {{ Some(ref off) => unsafe {{ std::ptr::read_unaligned(buf.add(off.offset)) != 0 }}, None => false }}"#,
                field_name
            ),
            _ => format!(
                r#"match offsets.{} {{ Some(ref off) => unsafe {{ std::ptr::read_unaligned(buf.add(off.offset) as *const i32) }}, None => 0 }}"#,
                field_name
            ),
        }
    }
}

pub fn generate_from_defs(vars: &[VarDef]) -> String {
    let mut out = String::new();

    out.push_str("// AUTOGENERATED FILE. Do not edit manually.\n");
    out.push_str("// To regenerate: run iracing_type_gen with iRacing open.\n");
    out.push_str(
        "//   cargo run --manifest-path tools/iracing_type_gen/Cargo.toml -- src/iracing/types.rs\n\n",
    );

    out.push_str("use std::collections::HashMap;\n\n");

    out.push_str(
        "/// Information about a resolved telemetry variable offset and count in shared memory.\n",
    );
    out.push_str("#[derive(Debug, Clone, Copy)]\n");
    out.push_str("pub struct IracingOffset {\n");
    out.push_str("    pub offset: usize,\n");
    out.push_str("    pub count: usize,\n");
    out.push_str("}\n\n");

    out.push_str("/// Cached shared memory offsets for all iRacing telemetry variables.\n");
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct IracingOffsets {\n");
    for v in vars {
        let field = camel_to_snake(&v.name);
        let unit_str = if v.unit.is_empty() {
            String::new()
        } else {
            format!(" [{}]", v.unit)
        };
        out.push_str(&format!("    /// {}{}\n", v.desc, unit_str));
        out.push_str(&format!("    pub {}: Option<IracingOffset>,\n", field));
    }
    out.push_str("}\n\n");

    out.push_str("impl IracingOffsets {\n");
    out.push_str("    pub(crate) fn resolve(vars: &HashMap<String, crate::iracing::structs::irsdk_varHeader>) -> Self {\n");
    out.push_str("        Self {\n");
    for v in vars {
        let field = camel_to_snake(&v.name);
        out.push_str(&format!(
            "            {}: vars.get(\"{}\").map(|v| IracingOffset {{ offset: v.offset as usize, count: v.count as usize }}),\n",
            field, v.name
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("/// Owned telemetry frame from iRacing. All fields populated in one SHM read.\n");
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct IracingFrame {\n");

    for v in vars {
        let field = camel_to_snake(&v.name);

        let unit_str = if v.unit.is_empty() {
            String::new()
        } else {
            format!(" [{}]", v.unit)
        };

        out.push_str(&format!("    /// {}{}\n", v.desc, unit_str));
        out.push_str(&format!(
            "    pub {}: {},\n",
            field,
            rust_type(&v.type_, v.count)
        ));
    }

    out.push_str("}\n\n");

    out.push_str("impl IracingFrame {\n");
    out.push_str(
        "    pub(crate) fn from_raw(buf: *const u8, offsets: &IracingOffsets) -> Self {\n",
    );
    out.push_str("        Self {\n");

    for v in vars {
        let field = camel_to_snake(&v.name);

        out.push_str(&format!(
            "            {}: {},\n",
            field,
            raw_extract_expr(&v.type_, v.count, &field)
        ));
    }

    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}

pub fn generate(toml_str: &str) -> String {
    let file: VarFile = toml::from_str(toml_str).expect("invalid toml");

    generate_from_defs(&file.var)
}
