#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
use crate::error::SimError;

/// Decode a byte slice from Windows-1252 (CP-1252) into a Rust `String`.
///
/// iRacing stores all shared-memory strings in CP-1252. Use this wherever
/// raw bytes from iRacing shared memory are converted to `String`.
pub fn decode_cp1252(bytes: &[u8]) -> String {
    let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);

    decoded.into_owned()
}

/// Implemented by every connection type that can produce a telemetry snapshot.
///
/// Allows [`save_telemetry_snapshot`] and [`save_var_list_snapshot`] to accept any
/// connection directly — `&IRsdkConnection`, `&LmuConnection`, `&AcEvoConnection`, or `&Connection`.
#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
pub trait HasSnapshot {
    fn telemetry_snapshot(&self)
    -> std::collections::HashMap<String, crate::types::TelemetryValue>;

    fn var_list_snapshot(&self) -> Vec<crate::types::VarMeta>;
}

macro_rules! impl_has_snapshot {
    ($feature:literal, $ty:path, $var_list_fn:expr) => {
        #[cfg(feature = $feature)]
        impl HasSnapshot for $ty {
            fn telemetry_snapshot(
                &self,
            ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
                self.telemetry_snapshot()
            }

            fn var_list_snapshot(&self) -> Vec<crate::types::VarMeta> {
                $var_list_fn(self)
            }
        }
    };
}

impl_has_snapshot!(
    "iracing",
    crate::iracing::connection::IRsdkConnection,
    |s: &crate::iracing::connection::IRsdkConnection| s.var_list_snapshot()
);
impl_has_snapshot!(
    "ac-evo",
    crate::ac_evo::connection::AcEvoConnection,
    |_s: &crate::ac_evo::connection::AcEvoConnection| crate::ac_evo::snapshot::var_list_snapshot()
);
impl_has_snapshot!(
    "lmu",
    crate::lmu::connection::LmuConnection,
    |_s: &crate::lmu::connection::LmuConnection| crate::lmu::snapshot::var_list_snapshot()
);

#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
impl HasSnapshot for crate::connection::Connection {
    fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        use crate::connection::Connection;

        match self {
            #[cfg(feature = "iracing")]
            Connection::IRacing(c) => c.telemetry_snapshot(),
            #[cfg(feature = "ac-evo")]
            Connection::AcEvo(c) => c.telemetry_snapshot(),
            #[cfg(feature = "lmu")]
            Connection::Lmu(c) => c.telemetry_snapshot(),
        }
    }

    fn var_list_snapshot(&self) -> Vec<crate::types::VarMeta> {
        use crate::connection::Connection;

        match self {
            #[cfg(feature = "iracing")]
            Connection::IRacing(c) => c.var_list_snapshot(),
            #[cfg(feature = "ac-evo")]
            Connection::AcEvo(c) => c.var_list_snapshot(),
            #[cfg(feature = "lmu")]
            Connection::Lmu(c) => c.var_list_snapshot(),
        }
    }
}

/// Capture one telemetry frame and write all variables to `path`, sorted by name.
///
/// Uses sim-native variable names and units. Format: one variable per line,
/// name left-padded to 32 chars, followed by its current value.
///
/// Accepts any connection type directly:
///
/// ```ignore
/// // from a Connection enum
/// kerb::save_telemetry_snapshot(&conn, "snapshot.txt")?;
///
/// // or from a specific connection inside a match arm
/// Ok(Connection::Lmu(conn)) => {
///     kerb::save_telemetry_snapshot(&conn, "lmu_snapshot.txt")?;
/// }
/// ```
#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
pub fn save_telemetry_snapshot(conn: &impl HasSnapshot, path: &str) -> Result<(), SimError> {
    let snap = conn.telemetry_snapshot();

    let mut entries: Vec<_> = snap.iter().collect();
    entries.sort_by_key(|(k, _)| k.as_str());

    let mut out = String::new();

    for (k, v) in entries {
        out.push_str(&format!("{:<32} {}\n", k, v));
    }

    std::fs::write(path, out)?;

    Ok(())
}

/// Write the variable catalogue (name, type, count, unit, description) to `path`,
/// sorted alphabetically.
///
/// Accepts any connection type directly — see [`save_telemetry_snapshot`].
#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
pub fn save_var_list_snapshot(conn: &impl HasSnapshot, path: &str) -> Result<(), SimError> {
    let mut vars = conn.var_list_snapshot();
    vars.sort_by(|a, b| a.name.cmp(&b.name));

    let mut out = String::new();

    for v in vars {
        let count_str = if v.count > 1 {
            format!("[{}]", v.count)
        } else {
            String::new()
        };

        out.push_str(&format!(
            "{:<32} {}{:<12} {:<16} {}\n",
            v.name, v.type_name, count_str, v.unit, v.desc
        ));
    }

    std::fs::write(path, out)?;

    Ok(())
}

/// Write the iRacing session YAML to `path`.
#[cfg(feature = "iracing")]
pub fn save_session(
    conn: &crate::iracing::connection::IRsdkConnection,
    path: &str,
) -> Result<(), SimError> {
    let yaml = conn.session_yaml().unwrap_or_default();

    std::fs::write(path, yaml)?;

    Ok(())
}
