use crate::error::SimError;

/// Identifies a specific simulator for use with [`SimConnection::connect_to`].
#[non_exhaustive]
pub enum SimType {
    #[cfg(feature = "iracing")]
    IRacing,
    #[cfg(feature = "ac")]
    Ac,
    #[cfg(feature = "lmu")]
    Lmu,
}

/// A connected simulator. Match on the variant to access its API.
///
/// # Example
///
/// ```ignore
/// use kerb::{SimConnection, Connection, ac::connection::AcFrame};
///
/// let conn = SimConnection::connect().expect("no sim running");
/// match conn {
///     Connection::IRacing(c) => {
///         c.wait_for_data(16);
///         let frame = c.frame();
///         println!("rpm={}", frame.rpm);
///     }
///     Connection::Ac(c) => {
///         let frame = c.frame();
///         match &frame {
///             AcFrame::Classic(f) => {
///                 println!("{:.0} rpm  gear {}", f.physics.rpms, f.physics.gear);
///             }
///             AcFrame::Evo(f) => {
///                 println!("{:.0} rpm  gear {}", f.physics.rpms, f.physics.gear);
///                 println!("pad_life: {:?}", f.physics.pad_life);
///             }
///         }
///     }
///     Connection::Lmu(c) => {
///         let frame = c.frame();
///         let player = frame.player_telemetry();
///         let rpm = player.engine_rpm;
///         println!("rpm={}", rpm);
///     }
/// }
/// ```
#[non_exhaustive]
pub enum Connection {
    #[cfg(feature = "iracing")]
    IRacing(crate::iracing::connection::IRsdkConnection),
    #[cfg(feature = "ac")]
    Ac(crate::ac::connection::AcConnection),
    #[cfg(feature = "lmu")]
    Lmu(crate::lmu::connection::LmuConnection),
}

/// Entry point for connecting to a running simulator.
pub struct SimConnection;

impl SimConnection {
    /// Auto-detect and connect to the first running simulator.
    ///
    /// Tries enabled features in order: iRacing → AC/AC Evo → LMU.
    /// For the `ac` feature, tries AC Evo SHM first, then classic AC.
    /// Returns [`SimError::NoSimRunning`] if nothing is running.
    pub fn connect() -> Result<Connection, SimError> {
        #[cfg(feature = "iracing")]
        {
            if let Ok(c) = crate::iracing::connection::IRsdkConnection::connect() {
                return Ok(Connection::IRacing(c));
            }
        }
        #[cfg(feature = "ac")]
        if let Ok(c) = crate::ac::connection::AcConnection::connect() {
            return Ok(Connection::Ac(c));
        }
        #[cfg(feature = "lmu")]
        if let Ok(c) = crate::lmu::connection::LmuConnection::connect() {
            return Ok(Connection::Lmu(c));
        }
        Err(SimError::NoSimRunning)
    }

    /// Connect to a specific simulator explicitly.
    pub fn connect_to(sim: SimType) -> Result<Connection, SimError> {
        match sim {
            #[cfg(feature = "iracing")]
            SimType::IRacing => {
                crate::iracing::connection::IRsdkConnection::connect().map(Connection::IRacing)
            }
            #[cfg(feature = "ac")]
            SimType::Ac => crate::ac::connection::AcConnection::connect().map(Connection::Ac),
            #[cfg(feature = "lmu")]
            SimType::Lmu => crate::lmu::connection::LmuConnection::connect().map(Connection::Lmu),
        }
    }
}
