#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
use crate::error::SimError;

/// Decode a byte slice written by a sim in the fixed single-byte encoding its
/// SDK specifies.
///
/// That encoding does not depend on the reader's Windows locale. iRacing
/// documents its non-UTF-8 session string as iso-8859-1 and selects it with
/// `irsdkUTF8SessionStr=0`; cp1252 is used here because it is identical to
/// iso-8859-1 except in the 0x80-0x9F range, where iso-8859-1 has unusable
/// control codes and the sim writes the printable characters cp1252 defines
/// there (curly quotes, dashes, ellipsis).
///
/// Non-Latin characters never reach this function: the sim substitutes them
/// before writing. Recovering them means opting into UTF-8 in `app.ini`, which
/// takes a different code path — see [`crate::iracing`].
pub fn decode_cp1252(bytes: &[u8]) -> String {
    if bytes.iter().all(|&b| b < 0x80) {
        // SAFETY: all bytes are valid ASCII, which is a subset of UTF-8
        return unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
    }
    let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
    decoded.into_owned()
}

/// Implemented by every connection type that can produce a telemetry snapshot.
///
/// Allows [`save_telemetry_snapshot`] and [`save_var_list_snapshot`] to accept any
/// connection directly — `&IRsdkConnection`, `&LmuConnection`, `&AcEvoConnection`, or `&Connection`.
#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
pub trait HasSnapshot {
    /// Capture all current telemetry variables as a `name → value` map.
    ///
    /// Variable names are sim-native (e.g. `"Speed"` for iRacing, `"speedKmh"` for AC Evo).
    fn telemetry_snapshot(&self)
    -> std::collections::HashMap<String, crate::types::TelemetryValue>;

    /// Return metadata for every telemetry variable the sim currently exposes.
    ///
    /// Includes name, type, unit, description, and array count for each variable.
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
impl<T: HasSnapshot> HasSnapshot for Box<T> {
    fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        (**self).telemetry_snapshot()
    }

    fn var_list_snapshot(&self) -> Vec<crate::types::VarMeta> {
        (**self).var_list_snapshot()
    }
}

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

    use std::fmt::Write as _;
    let mut out = String::with_capacity(entries.len() * 48);

    for (k, v) in entries {
        writeln!(out, "{:<32} {}", k, v).ok();
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

    use std::fmt::Write as _;
    let mut out = String::with_capacity(vars.len() * 80);

    for v in vars {
        let count_str = if v.count > 1 {
            format!("[{}]", v.count)
        } else {
            String::new()
        };

        writeln!(
            out,
            "{:<32} {}{:<12} {:<16} {}",
            v.name, v.type_name, count_str, v.unit, v.desc
        )
        .ok();
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
