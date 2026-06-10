use serde::Deserialize;

#[derive(Deserialize)]
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

/// iRacing names that defeat mechanical CamelCase splitting (irregular
/// acronym/word boundaries). Checked before the generic conversion.
const SNAKE_OVERRIDES: &[(&str, &str)] = &[("BrakeABSactive", "brake_abs_active")];

/// Tire/corner prefixes: `LFtempCL` means "LF temp CL", so the two-letter
/// corner code is one word (`lf_temp_cl`), not the generic split (`l_ftemp_cl`).
const CORNER_PREFIXES: &[&str] = &["LF", "LR", "RF", "RR"];

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
            let first = tail_chars.next().unwrap().to_uppercase().next().unwrap();
            let tail: String = std::iter::once(first).chain(tail_chars).collect();
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
