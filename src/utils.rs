#[cfg(any(feature = "iracing", feature = "ac-evo", feature = "lmu"))]
use crate::error::SimError;

/// Decode a byte slice using the Windows system ACP (e.g. CP-1251 on Russian Windows, CP-1252 on Western).
pub fn decode_cp1252(bytes: &[u8]) -> String {
    if bytes.iter().all(|&b| b < 0x80) {
        // SAFETY: all bytes are valid ASCII, which is a subset of UTF-8
        return unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
    }
    let encoding = system_acp_encoding();
    let (decoded, _, _) = encoding.decode(bytes);
    decoded.into_owned()
}

fn system_acp_encoding() -> &'static encoding_rs::Encoding {
    #[cfg(all(windows, any(feature = "iracing", feature = "ac-evo", feature = "lmu")))]
    {
        // SAFETY: GetACP() is always safe to call and never fails.
        let acp = unsafe { windows_sys::Win32::Globalization::GetACP() };
        match acp {
            1251 => encoding_rs::WINDOWS_1251,
            1252 => encoding_rs::WINDOWS_1252,
            1250 => encoding_rs::WINDOWS_1250,
            1253 => encoding_rs::WINDOWS_1253,
            1254 => encoding_rs::WINDOWS_1254,
            1255 => encoding_rs::WINDOWS_1255,
            1256 => encoding_rs::WINDOWS_1256,
            1257 => encoding_rs::WINDOWS_1257,
            1258 => encoding_rs::WINDOWS_1258,
            874  => encoding_rs::WINDOWS_874,
            932  => encoding_rs::SHIFT_JIS,
            936  => encoding_rs::GBK,
            949  => encoding_rs::EUC_KR,
            950  => encoding_rs::BIG5,
            _    => encoding_rs::WINDOWS_1252,
        }
    }
    #[cfg(not(all(windows, any(feature = "iracing", feature = "ac-evo", feature = "lmu"))))]
    {
        encoding_rs::WINDOWS_1252
    }
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
