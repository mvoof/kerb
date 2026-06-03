#[cfg(feature = "ac")]
use kerb::ac::types::{AcPhysicsData, AcStaticData};

#[cfg(feature = "lmu")]
use kerb::lmu::types::LmuFrame;

#[cfg(feature = "iracing")]
use kerb::iracing::structs::irsdk_header;

#[cfg(feature = "ac")]
#[test]
fn physics_struct_size() {
    let size = std::mem::size_of::<AcPhysicsData>();
    assert!(
        size > 100,
        "AcPhysicsData is unexpectedly small: {} bytes",
        size
    );
}

#[cfg(feature = "ac")]
#[test]
fn static_struct_has_track_field() {
    let s: AcStaticData = unsafe { std::mem::zeroed() };
    let _ = s.track;
}

#[cfg(feature = "lmu")]
#[test]
fn lmu_frame_size() {
    let size = std::mem::size_of::<LmuFrame>();
    assert!(
        size > 1000,
        "LmuFrame is unexpectedly small: {} bytes",
        size
    );
}

#[cfg(feature = "iracing")]
#[test]
fn iracing_header_struct_size() {
    let size = std::mem::size_of::<irsdk_header>();
    assert_eq!(
        size, 112,
        "irsdk_header size must be exactly 112 bytes for binary compatibility"
    );
}
