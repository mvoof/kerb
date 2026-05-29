/// Errors returned by simulator backends and utility helpers.
#[derive(Debug)]
pub enum SimError {
    /// The simulator's shared memory region could not be opened (sim not running).
    NotConnected(String),
    /// No enabled simulator is currently running.
    NoSimRunning,
    /// The shared memory header contains unexpected or corrupt data.
    InvalidHeader(String),
    /// A standard I/O error (e.g. writing a snapshot file to disk).
    Io(std::io::Error),
}

/// Human-readable error messages for logging / display.
impl std::fmt::Display for SimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimError::NotConnected(msg) => write!(f, "Not connected: {}", msg),
            SimError::NoSimRunning => write!(f, "No simulator is currently running"),
            SimError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            SimError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for SimError {}

/// Allows `?` to convert [`std::io::Error`] into [`SimError::Io`] automatically.
impl From<std::io::Error> for SimError {
    fn from(e: std::io::Error) -> Self {
        SimError::Io(e)
    }
}
