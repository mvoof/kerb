use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(feature = "ac-evo")]
use kerb::ac_evo::snapshot::build_snapshot as build_ac_snapshot;

#[cfg(feature = "lmu")]
use kerb::lmu::{LmuFrame, snapshot::build_snapshot as build_lmu_snapshot};

#[cfg(feature = "iracing")]
use kerb::iracing::{
    connection::IRsdkConnection,
    structs::{irsdk_header, irsdk_varHeader},
};

use kerb::decode_cp1252;

/// CP-1252 string decoding — called on every string field read from iRacing SHM.
fn bench_string_decoders(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Decoders");

    let ascii_bytes = b"Spa-Francorchamps";
    let accented_bytes = b"N\xfcrburgring Nordschleife"; // 'ü' = 0xFC in CP-1252

    group.bench_function("decode_cp1252 ASCII", |b| {
        b.iter(|| black_box(decode_cp1252(black_box(ascii_bytes))))
    });

    group.bench_function("decode_cp1252 CP-1252 accented", |b| {
        b.iter(|| black_box(decode_cp1252(black_box(accented_bytes))))
    });

    group.finish();
}

/// Assetto Corsa Evo — frame construction and field access.
#[cfg(feature = "ac-evo")]
fn bench_assetto_corsa(c: &mut Criterion) {
    use kerb::ac_evo::connection::AcEvoFrame;

    let mut group = c.benchmark_group("Assetto Corsa Evo");

    let frame: AcEvoFrame = unsafe { std::mem::zeroed() };

    group.bench_function("Read single field (physics.rpms)", |b| {
        b.iter(|| black_box(black_box(&frame).physics.rpms))
    });

    group.bench_function("Clone frame", |b| {
        b.iter(|| black_box(black_box(&frame).clone()))
    });

    group.bench_function("Build snapshot HashMap", |b| {
        b.iter(|| black_box(build_ac_snapshot(black_box(&frame))))
    });

    group.finish();
}

/// Le Mans Ultimate — frame access with the new mirror types.
#[cfg(feature = "lmu")]
fn bench_le_mans_ultimate(c: &mut Criterion) {
    let mut group = c.benchmark_group("Le Mans Ultimate");

    let frame = LmuFrame::default();

    group.bench_function(
        "Read single field (vehicles_telemetry[0].engine_rpm)",
        |b| b.iter(|| black_box(black_box(&frame).vehicles_telemetry[0].engine_rpm)),
    );

    group.bench_function("vehicles_telemetry() slice (0 vehicles)", |b| {
        b.iter(|| black_box(black_box(&frame).vehicles_telemetry()))
    });

    group.bench_function("player_telemetry() lookup", |b| {
        b.iter(|| black_box(black_box(&frame).player_telemetry()))
    });

    group.bench_function("Build snapshot HashMap", |b| {
        b.iter(|| black_box(build_lmu_snapshot(black_box(&frame))))
    });

    group.finish();
}

/// iRacing — mock connection benchmarks for frame reading and session parsing.
#[cfg(feature = "iracing")]
fn bench_iracing(c: &mut Criterion) {
    let mut group = c.benchmark_group("iRacing");

    let mut header: irsdk_header = unsafe { std::mem::zeroed() };
    header.ver = 1;
    header.status = 1;
    header.num_buf = 1;
    header.var_buf[0].tick_count = 100;
    header.var_buf[0].buf_offset = std::mem::size_of::<irsdk_header>() as i32;

    let yaml = "WeekendInfo:\n  TrackDisplayName: Spa-Francorchamps\n\0";
    let hz = std::mem::size_of::<irsdk_header>();
    header.session_info_len = yaml.len() as i32;
    header.session_info_offset = (hz + 4) as i32;
    header.session_info_update = 1;

    let mut buf = vec![0u8; hz + 4 + yaml.len()];
    unsafe {
        std::ptr::copy_nonoverlapping(
            &header as *const irsdk_header as *const u8,
            buf.as_mut_ptr(),
            hz,
        );
        let rpm: f32 = 7500.0;
        std::ptr::copy_nonoverlapping(&rpm as *const f32 as *const u8, buf.as_mut_ptr().add(hz), 4);
        std::ptr::copy_nonoverlapping(yaml.as_ptr(), buf.as_mut_ptr().add(hz + 4), yaml.len());
    }

    let mut vars = std::collections::HashMap::new();
    let mut rpm_hdr = irsdk_varHeader {
        type_: 4,
        offset: 0,
        count: 1,
        count_as_char: 0,
        pad: [0; 3],
        name: [0; 32],
        desc: [0; 64],
        unit: [0; 32],
    };
    rpm_hdr.name[..3].copy_from_slice(b"RPM");
    vars.insert("RPM".to_string(), rpm_hdr);

    let conn = unsafe { IRsdkConnection::new_mock(buf.as_mut_ptr() as *mut _, vars) };

    group.bench_function("read_frame() — typed IracingFrame", |b| {
        b.iter(|| black_box(black_box(&conn).read_frame(0)))
    });

    group.bench_function("read_variable(\"RPM\") — dynamic lookup", |b| {
        b.iter(|| black_box(black_box(&conn).read_variable("RPM")))
    });

    group.bench_function("telemetry_snapshot() — full HashMap", |b| {
        b.iter(|| black_box(black_box(&conn).telemetry_snapshot()))
    });

    group.bench_function("session_info() — cached hot path", |b| {
        b.iter(|| black_box(black_box(&conn).session_info()))
    });

    let header_ptr = buf.as_mut_ptr() as *mut irsdk_header;
    group.bench_function(
        "session_info() — cold parse (YAML re-parse each call)",
        |b| {
            b.iter(|| {
                unsafe {
                    (*header_ptr).session_info_update += 1;
                }
                black_box(black_box(&conn).session_info())
            })
        },
    );

    group.finish();

    // Benchmark with array fields (car_idx_* count=64) — exercises
    // the copy_nonoverlapping path that replaced per-element read_unaligned loops.
    let mut group = c.benchmark_group("iRacing array fields");

    let array_count: usize = 64;
    let mut array_buf = vec![0u8; hz + array_count * 4 * 6];
    unsafe {
        std::ptr::copy_nonoverlapping(
            &header as *const irsdk_header as *const u8,
            array_buf.as_mut_ptr(),
            hz,
        );
        for i in 0..array_count {
            let v: f32 = 1000.0 + i as f32;
            std::ptr::copy_nonoverlapping(
                &v as *const f32 as *const u8,
                array_buf.as_mut_ptr().add(hz + i * 4),
                4,
            );
        }
    }

    let mut offset_cursor = 0i32;
    let mut array_vars: std::collections::HashMap<String, irsdk_varHeader> =
        std::collections::HashMap::new();

    for (name, type_) in &[
        ("CarIdxLapDistPct", 4i32),
        ("CarIdxEstTime", 4),
        ("CarIdxLap", 2),
        ("CarIdxPosition", 2),
    ] {
        let mut h = irsdk_varHeader {
            type_: *type_,
            offset: offset_cursor,
            count: array_count as i32,
            count_as_char: 0,
            pad: [0; 3],
            name: [0; 32],
            desc: [0; 64],
            unit: [0; 32],
        };
        let nb = name.len().min(31);
        h.name[..nb].copy_from_slice(name.as_bytes());
        array_vars.insert(name.to_string(), h);
        offset_cursor += array_count as i32 * 4;
    }

    let mut bool_var = irsdk_varHeader {
        type_: 1,
        offset: offset_cursor,
        count: array_count as i32,
        count_as_char: 0,
        pad: [0; 3],
        name: [0; 32],
        desc: [0; 64],
        unit: [0; 32],
    };
    bool_var.name[..15].copy_from_slice(b"CarIdxOnPitRoad");
    array_vars.insert("CarIdxOnPitRoad".to_string(), bool_var);

    let array_conn =
        unsafe { IRsdkConnection::new_mock(array_buf.as_mut_ptr() as *mut _, array_vars) };

    group.bench_function("read_frame() — 4xf32[64] + 2xi32[64] + 1xbool[64]", |b| {
        b.iter(|| black_box(black_box(&array_conn).read_frame(0)))
    });

    group.finish();
}

fn run_all_benches(c: &mut Criterion) {
    bench_string_decoders(c);

    #[cfg(feature = "ac-evo")]
    bench_assetto_corsa(c);

    #[cfg(feature = "lmu")]
    bench_le_mans_ultimate(c);

    #[cfg(feature = "iracing")]
    bench_iracing(c);
}

criterion_group!(benches, run_all_benches);
criterion_main!(benches);
