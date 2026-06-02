//! Real-time telemetry from sim-racing titles: iRacing, Assetto Corsa, AC Evo, Le Mans Ultimate.
//!
//! # Quick start
//!
//! ```no_run
//! use kerb::{Connection, SimConnection};
//!
//! let conn = SimConnection::connect().expect("no sim running");
//! match conn {
//!     #[cfg(feature = "iracing")]
//!     Connection::IRacing(c) => {
//!         c.wait_for_data(16);
//!         let frame = c.frame().expect("failed to read frame");
//!         println!("rpm={} gear={}", frame.rpm, frame.gear);
//!     }
//!     #[cfg(feature = "ac")]
//!     Connection::Ac(c) => {
//!         let frame = c.frame().expect("failed to read frame");
//!         println!("{:.0} rpm  gear {}", frame.rpms(), frame.gear());
//!     }
//!     #[cfg(feature = "lmu")]
//!     Connection::Lmu(c) => {
//!         let frame = c.frame().expect("failed to read frame");
//!         let _ = frame;
//!     }
//!     _ => {}
//! }
//! ```
//!
//! # Feature flags
//!
//! | Feature   | Sim                            | `Connection` variant  |
//! |-----------|--------------------------------|-----------------------|
//! | `iracing` | iRacing                        | `Connection::IRacing` |
//! | `ac`      | Assetto Corsa + AC Evo (auto)  | `Connection::Ac`      |
//! | `lmu`     | Le Mans Ultimate               | `Connection::Lmu`     |
//!
//! Enable multiple features if your overlay supports several sims.
//! [`SimConnection::connect`] tries each enabled sim in order and returns the
//! first one running.

#[cfg(not(any(feature = "iracing", feature = "ac", feature = "lmu")))]
compile_error!(
    "kerb: no simulator features enabled. \
     Add at least one of: \"iracing\", \"ac\", \"lmu\""
);

#[cfg(not(target_os = "windows"))]
compile_error!("kerb only supports Windows targets (iRacing/AC/LMU use Windows Shared Memory API)");

pub(crate) mod error;
pub(crate) mod types;
pub(crate) mod utils;

pub mod connection;
pub mod sim_string;

#[cfg(feature = "iracing")]
pub mod iracing;

#[cfg(feature = "ac")]
pub mod ac;

#[cfg(feature = "lmu")]
pub mod lmu;

#[cfg(any(feature = "ac", feature = "lmu"))]
pub(crate) mod shm;

pub use connection::{Connection, SimConnection, SimType};
pub use error::SimError;
pub use sim_string::{SimString, SimStringU16};
pub use types::{TelemetryValue, VarMeta, VarType};
pub use utils::decode_cp1252;

#[cfg(feature = "iracing")]
pub use utils::save_session;

#[cfg(any(feature = "iracing", feature = "ac", feature = "lmu"))]
pub use utils::{HasSnapshot, save_telemetry_snapshot, save_var_list};
