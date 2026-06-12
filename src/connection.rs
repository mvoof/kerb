use crate::error::SimError;

/// Identifies a specific simulator for use with [`SimConnection::connect_to`].
#[non_exhaustive]
pub enum SimType {
    #[cfg(feature = "iracing")]
    IRacing,
    #[cfg(feature = "ac-evo")]
    AcEvo,
    #[cfg(feature = "lmu")]
    Lmu,
}

/// A connected simulator. Match on the variant to access its API.
///
/// # Threading
///
/// `Connection` (and every per-sim connection inside it) is **not [`Send`]** —
/// this is a deliberate API contract: the connections hold raw shared-memory
/// pointers, Win32 handles, and `RefCell` caches. Create the connection on the
/// thread that will use it. The standard pattern for GUI apps is a dedicated
/// `std::thread` that owns the connection and forwards data via channels/events:
///
/// ```ignore
/// std::thread::spawn(move || {
///     let conn = kerb::SimConnection::connect().unwrap(); // created IN the thread
///     // … read loop, send normalized frames out …
/// });
/// ```
///
/// # Example
///
/// ```ignore
/// use kerb::{SimConnection, Connection, ac_evo::connection::AcEvoFrame};
///
/// let conn = SimConnection::connect().expect("no sim running");
/// match conn {
///     Connection::IRacing(c) => {
///         c.wait_for_data(16);
///         let frame = c.frame();
///         println!("rpm={}", frame.rpm);
///     }
///     Connection::AcEvo(c) => {
///         let frame = c.frame().unwrap();
///         println!("{:.0} rpm  gear {}", frame.physics.rpms, frame.physics.gear);
///         println!("abs={} tc={}", frame.graphics.electronics.abs_level, frame.graphics.electronics.tc_level);
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
    IRacing(Box<crate::iracing::connection::IRsdkConnection>),
    #[cfg(feature = "ac-evo")]
    AcEvo(crate::ac_evo::connection::AcEvoConnection),
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
                return Ok(Connection::IRacing(Box::new(c)));
            }
        }
        #[cfg(feature = "ac-evo")]
        if let Ok(c) = crate::ac_evo::connection::AcEvoConnection::connect() {
            return Ok(Connection::AcEvo(c));
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
            SimType::IRacing => crate::iracing::connection::IRsdkConnection::connect()
                .map(|c| Connection::IRacing(Box::new(c))),
            #[cfg(feature = "ac-evo")]
            SimType::AcEvo => {
                crate::ac_evo::connection::AcEvoConnection::connect().map(Connection::AcEvo)
            }
            #[cfg(feature = "lmu")]
            SimType::Lmu => crate::lmu::connection::LmuConnection::connect().map(Connection::Lmu),
        }
    }
}
