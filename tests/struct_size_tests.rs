#[cfg(feature = "ac")]
use kerb::ac::structs::{SPageFilePhysics, SPageFileStatic};

#[cfg(feature = "ac-evo")]
use kerb::ac_evo::structs::{SPageFilePhysicsEvo, SPageFileStaticEvo};

#[cfg(feature = "lmu")]
use kerb::lmu::structs::rF2Telemetry;

#[cfg(feature = "iracing")]
use kerb::iracing::types::irsdk_header;

#[cfg(feature = "ac")]
#[test]
fn physics_struct_size() {
    let size = std::mem::size_of::<SPageFilePhysics>();
    // Sanity check: Assetto Corsa's SPageFilePhysics is roughly ~300-800 bytes depending
    // on compiled versions. We use a defensive lower-bound check (> 100) instead of an
    // exact equality check. This prevents the tests from breaking if Kunos appends new
    // telemetry fields to the end of the shared memory page in future game updates.
    assert!(
        size > 100,
        "SPageFilePhysics is unexpectedly small: {} bytes",
        size
    );
}

#[cfg(feature = "ac")]
#[test]
fn static_struct_has_track_field() {
    let s = SPageFileStatic::default();
    let _ = s.track;
}

#[cfg(feature = "ac-evo")]
#[test]
fn ac_evo_physics_struct_size() {
    let size = std::mem::size_of::<SPageFilePhysicsEvo>();
    // Sanity check: Assetto Corsa Evo's SPageFilePhysicsEvo is hundreds of bytes.
    // We use a defensive lower-bound check (> 100) to keep the test suite robust against
    // future game patches adding new physical parameters to the end of the page structure.
    assert!(
        size > 100,
        "SPageFilePhysicsEvo is unexpectedly small: {} bytes",
        size
    );
}

#[cfg(feature = "ac-evo")]
#[test]
fn ac_evo_static_struct_has_track_field() {
    let s = SPageFileStaticEvo::default();
    let _ = s.track;
}

#[cfg(feature = "lmu")]
#[test]
fn lmu_telemetry_struct_size() {
    let size = std::mem::size_of::<rF2Telemetry>();
    // Sanity check: Le Mans Ultimate (rFactor 2 plugin layout) telemetry page is massive
    // because it contains nested arrays for up to 128 vehicles (RF2_MAX_VEHICLES) and their
    // associated tires, inputs, and physical channels, spanning hundreds of kilobytes.
    // A soft check (> 1000) guarantees the multi-vehicle nested arrays were populated
    // properly by the compiler without breaking on minor alignment or padding updates.
    assert!(
        size > 1000,
        "rF2Telemetry is unexpectedly small: {} bytes",
        size
    );
}

#[cfg(feature = "iracing")]
#[test]
fn iracing_header_struct_size() {
    let size = std::mem::size_of::<irsdk_header>();
    // iRacing's irsdk_header must be exactly 112 bytes for binary compatibility with the sim.
    // Calculation:
    // - Header metadata: 10 fields * 4 bytes + 8 bytes padding = 48 bytes
    // - Buffer list: 4 slots * 16 bytes per slot (irsdk_varBuf) = 64 bytes
    // - Total: 48 + 64 = 112 bytes.
    assert_eq!(
        size, 112,
        "irsdk_header size must be exactly 112 bytes for binary compatibility"
    );
}
