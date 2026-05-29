use kerb::SimError;

#[test]
fn sim_error_display() {
    let e = SimError::NotConnected("iRacing shared memory not found".into());
    assert!(e.to_string().contains("iRacing shared memory not found"));
}

#[test]
fn sim_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let e: SimError = io_err.into();
    assert!(matches!(e, SimError::Io(_)));
}

#[test]
fn sim_error_no_sim_running_display() {
    let e = SimError::NoSimRunning;
    assert!(e.to_string().contains("No simulator"));
}
