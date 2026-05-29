use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(feature = "ac")]
use kerb::ac::{
    connection::{AcClassicFrame, AcFrame},
    snapshot::build_snapshot as build_ac_snapshot,
};

#[cfg(feature = "lmu")]
use kerb::lmu::{connection::LmuFrame, snapshot::build_snapshot as build_lmu_snapshot};

#[cfg(feature = "iracing")]
use kerb::iracing::{
    connection::IRsdkConnection,
    types::{irsdk_header, irsdk_varHeader},
};

use kerb::decode_cp1252;

/// Benchmark for string decoding (Windows-1252 to Rust String).
fn bench_string_decoders(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Decoders");

    // Simulate a typical CP-1252 telemetry string (e.g. driver name, track name)
    let ascii_bytes = b"Spa-Francorchamps";
    let accented_bytes = b"N\xfcrburgring Nordschleife"; // 'ü' in CP-1252 is 0xFC

    group.bench_function("decode_cp1252 (ASCII)", |b| {
        b.iter(|| {
            let res = decode_cp1252(black_box(ascii_bytes));
            black_box(res);
        })
    });

    group.bench_function("decode_cp1252 (Accented/CP-1252)", |b| {
        b.iter(|| {
            let res = decode_cp1252(black_box(accented_bytes));
            black_box(res);
        })
    });

    group.finish();
}

/// Benchmarks for Assetto Corsa (AC) telemetry performance.
#[cfg(feature = "ac")]
fn bench_assetto_corsa(c: &mut Criterion) {
    let mut group = c.benchmark_group("Assetto Corsa");

    // Initialize an empty frame (simulates shared memory state)
    let inner: AcClassicFrame = unsafe { std::mem::zeroed() };
    let frame = AcFrame::Classic(inner);

    group.bench_function("Read Single Field (Physics RPM)", |b| {
        b.iter(|| {
            let rpms = black_box(&inner).physics.rpms;
            black_box(rpms);
        })
    });

    group.bench_function("Stack Copy Full Frame", |b| {
        b.iter(|| {
            let cloned_frame = *black_box(&inner);
            black_box(cloned_frame);
        })
    });

    group.bench_function("Build Snapshot HashMap (Allocating)", |b| {
        b.iter(|| {
            let snap = build_ac_snapshot(black_box(&frame));
            black_box(snap);
        })
    });

    group.finish();
}

/// Benchmarks for Le Mans Ultimate (LMU) telemetry performance.
#[cfg(feature = "lmu")]
fn bench_le_mans_ultimate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Le Mans Ultimate");

    // Initialize an empty frame
    let frame: LmuFrame = unsafe { std::mem::zeroed() };

    group.bench_function("Read Single Field (Gear)", |b| {
        b.iter(|| {
            let gear = black_box(&frame).telemetry.vehicles[0].gear;
            black_box(gear);
        })
    });

    group.bench_function("Stack Copy Full Frame", |b| {
        b.iter(|| {
            let cloned_frame = *black_box(&frame);
            black_box(cloned_frame);
        })
    });

    group.bench_function("Build Snapshot HashMap (Allocating)", |b| {
        b.iter(|| {
            let snap = build_lmu_snapshot(black_box(&frame));
            black_box(snap);
        })
    });

    group.finish();
}

/// Benchmarks for iRacing telemetry performance using a memory-mocked connection.
#[cfg(feature = "iracing")]
fn bench_iracing(c: &mut Criterion) {
    let mut group = c.benchmark_group("iRacing");

    // Construct mock irsdk_header
    let mut header: irsdk_header = unsafe { std::mem::zeroed() };
    header.ver = 1;
    header.status = 1; // connected
    header.num_buf = 1;
    header.var_buf[0].tick_count = 100;
    header.var_buf[0].buf_offset = std::mem::size_of::<irsdk_header>() as i32;

    // Define a realistic mock iRacing session-info YAML string
    let yaml_str = "WeekendInfo:\n  TrackName: spa\n  TrackDisplayName: Spa-Francorchamps\n  TrackLength: 7.00 km\n\0";
    let yaml_bytes = yaml_str.as_bytes();

    let header_sz = std::mem::size_of::<irsdk_header>();
    header.session_info_len = yaml_bytes.len() as i32;
    header.session_info_offset = (header_sz + 4) as i32;
    header.session_info_update = 1;

    // We store the header + 4 bytes float value + YAML bytes in a contiguous buffer
    let mut buffer = vec![0u8; header_sz + 4 + yaml_bytes.len()];

    // Copy header, float, and YAML string into the memory buffer
    unsafe {
        std::ptr::copy_nonoverlapping(
            &header as *const irsdk_header as *const u8,
            buffer.as_mut_ptr(),
            header_sz,
        );
        // Write the RPM value (5500.0) at offset 112
        let rpm: f32 = 5500.0;
        std::ptr::copy_nonoverlapping(
            &rpm as *const f32 as *const u8,
            buffer.as_mut_ptr().add(header_sz),
            4,
        );
        // Write the YAML string at the offset
        std::ptr::copy_nonoverlapping(
            yaml_bytes.as_ptr(),
            buffer.as_mut_ptr().add(header_sz + 4),
            yaml_bytes.len(),
        );
    }

    // Populate variable mapping
    let mut vars = std::collections::HashMap::new();
    let mut rpm_var = irsdk_varHeader {
        type_: 4, // Float
        offset: 0,
        count: 1,
        count_as_char: 0,
        pad: [0; 3],
        name: [0; 32],
        desc: [0; 64],
        unit: [0; 32],
    };
    rpm_var.name[0] = b'R';
    rpm_var.name[1] = b'P';
    rpm_var.name[2] = b'M';
    vars.insert("RPM".to_string(), rpm_var);

    // Create the mock connection
    let conn = IRsdkConnection::new_mock(buffer.as_mut_ptr() as *mut _, vars);

    group.bench_function("Read Single Field (rpm struct field)", |b| {
        b.iter(|| {
            let frame = black_box(&conn).frame();
            let rpm = frame.rpm;
            black_box(rpm);
        })
    });

    group.bench_function("Read Full Frame (all vars from SHM)", |b| {
        b.iter(|| {
            let frame = black_box(&conn).frame();
            black_box(frame);
        })
    });

    group.bench_function("Telemetry Snapshot HashMap", |b| {
        b.iter(|| {
            let snap = black_box(&conn).telemetry_snapshot();
            black_box(snap);
        })
    });

    group.bench_function("Session Info - Cached O(1) Hot Path", |b| {
        b.iter(|| {
            let session = black_box(&conn).session_info();
            black_box(session);
        })
    });

    let shared_mem = buffer.as_mut_ptr();
    let header_ptr = shared_mem as *mut irsdk_header;

    group.bench_function(
        "Session Info - Parsing Cold Path (with YAML parsing)",
        |b| {
            b.iter(|| {
                // Force cache invalidation by incrementing update version on every iteration
                unsafe {
                    (*header_ptr).session_info_update += 1;
                }
                let session = black_box(&conn).session_info();
                black_box(session);
            })
        },
    );

    group.finish();
}

criterion_group!(benches, run_all_benches);
criterion_main!(benches);

// Helper to run all enabled benchmarks.
// This allows clean conditional compilation inside standard Rust function scope,
// avoiding macro parsing errors with #[cfg] inside the criterion_group! macro.
fn run_all_benches(c: &mut Criterion) {
    bench_string_decoders(c);

    #[cfg(feature = "ac")]
    bench_assetto_corsa(c);

    #[cfg(feature = "lmu")]
    bench_le_mans_ultimate(c);

    #[cfg(feature = "iracing")]
    bench_iracing(c);
}
