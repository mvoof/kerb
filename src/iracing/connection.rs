use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, HANDLE, WAIT_OBJECT_0};
use windows_sys::Win32::System::Threading::{OpenEventW, WaitForSingleObject};

use crate::connection::ReadResult;
use crate::iracing::structs::{IRSDK_MAX_BUFS, VarType, irsdk_header, irsdk_varHeader};
use crate::types::TelemetryValue;

const SYNCHRONIZE: u32 = 0x00100000;

const SHM_MEM_MAP_FILE: &str = "Local\\IRSDKMemMapFileName";
const SHM_DATA_VALID_EVENT: &str = "Local\\IRSDKDataValidEventName";

/// Decode a null-terminated cp1252 byte array into a Rust `String`.
/// iRacing stores all strings in shared memory as fixed-size, null-padded cp1252 buffers.
fn parse_c_str(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&x| x == 0).unwrap_or(bytes.len());
    crate::decode_cp1252(&bytes[..len])
}

/// Live connection to the iRacing telemetry service via Win32 shared memory.
///
/// Holds the shared-memory region, an optional data-ready event used for
/// efficient frame-synchronised waiting, and the parsed variable header table.
///
/// # Threading
///
/// Not [`Send`] — and this is a deliberate API contract, not an oversight:
/// the struct holds raw shared-memory pointers, a Win32 event `HANDLE`, and a
/// `RefCell` session cache. Create and use the connection on a single thread;
/// for GUI apps spawn a dedicated telemetry `std::thread` that owns the
/// connection and sends normalized data out through channels or events.
pub struct IRsdkConnection {
    shm: crate::shm::SharedMemRegion,
    h_event: HANDLE,
    pub(crate) vars: HashMap<String, irsdk_varHeader>,
    cached_session: std::cell::RefCell<Option<(i32, crate::iracing::session::IracingSession)>>,
    pub(crate) offsets: crate::iracing::types::IracingOffsets,
}

impl IRsdkConnection {
    /// Create a mock connection for unit testing and benchmarking.
    #[doc(hidden)]
    pub unsafe fn new_mock(
        view_address: *mut std::ffi::c_void,
        vars: HashMap<String, irsdk_varHeader>,
    ) -> Self {
        let offsets = crate::iracing::types::IracingOffsets::resolve(&vars);
        Self {
            shm: unsafe { crate::shm::SharedMemRegion::new_mock(view_address) },
            h_event: 0 as _,
            vars,
            cached_session: std::cell::RefCell::new(None),
            offsets,
        }
    }

    /// Open the iRacing shared-memory region and parse the variable header table.
    /// Returns `Err` if the sim is not running or the header is invalid.
    #[doc(hidden)]
    pub fn connect() -> Result<Self, crate::error::SimError> {
        let shm = crate::shm::SharedMemRegion::open(SHM_MEM_MAP_FILE)
            .map_err(crate::error::SimError::NotConnected)?;

        let shared_mem = shm.as_ptr();

        unsafe {
            let header = std::ptr::read_unaligned(shared_mem as *const irsdk_header);

            let event_name: Vec<u16> = SHM_DATA_VALID_EVENT
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let h_event = OpenEventW(SYNCHRONIZE, FALSE, event_name.as_ptr());

            if header.ver <= 0 || header.ver > 10 || header.num_vars <= 0 {
                if !h_event.is_null() {
                    CloseHandle(h_event);
                }

                return Err(crate::error::SimError::InvalidHeader(format!(
                    "Invalid iRacing telemetry header (ver={}, num_vars={})",
                    header.ver, header.num_vars
                )));
            }

            let shm_size = 32 * 1024 * 1024usize; // 32 MB — upper bound for iRacing SHM
            let var_offset = header.var_header_offset as usize;
            let element_size = std::mem::size_of::<irsdk_varHeader>();
            let num = header.num_vars as usize;
            if var_offset.saturating_add(num.saturating_mul(element_size)) > shm_size {
                if !h_event.is_null() {
                    CloseHandle(h_event);
                }
                return Err(crate::error::SimError::InvalidHeader(
                    "var_header_offset out of SHM bounds".into(),
                ));
            }

            let mut vars = HashMap::new();

            for i in 0..num {
                let offset = var_offset + i * element_size;
                let var_header_ptr = shared_mem.add(offset) as *const irsdk_varHeader;
                let var_header = std::ptr::read_unaligned(var_header_ptr);
                let name_str = parse_c_str(&var_header.name);
                vars.insert(name_str, var_header);
            }

            let offsets = crate::iracing::types::IracingOffsets::resolve(&vars);

            Ok(Self {
                shm,
                h_event,
                vars,
                cached_session: std::cell::RefCell::new(None),
                offsets,
            })
        }
    }

    /// Returns `true` when the iRacing sim is actively broadcasting telemetry (status bit 0).
    pub(crate) fn is_connected(&self) -> bool {
        unsafe {
            let offset = std::mem::offset_of!(irsdk_header, status);
            let status = std::ptr::read_unaligned(self.shm.as_ptr().add(offset) as *const i32);
            (status & 1) != 0
        }
    }

    /// Block until the sim signals new data, or until `timeout_ms` elapses.
    ///
    /// Returns `true` when data is (likely) available, `false` on timeout or disconnect.
    /// Falls back to a 16 ms sleep when the event handle is unavailable, then checks
    /// `is_connected()` — callers receive `false` as soon as the sim closes even without
    /// a Win32 event to wake them.
    pub(crate) fn wait_for_data(&self, timeout_ms: u32) -> bool {
        unsafe {
            if self.h_event.is_null() {
                sleep(Duration::from_millis(16));

                self.is_connected()
            } else {
                let wait_result = WaitForSingleObject(self.h_event, timeout_ms);

                wait_result == WAIT_OBJECT_0
            }
        }
    }

    /// Return the session info update version counter.
    /// This counter increments whenever the session info YAML block changes,
    /// so callers can cheaply detect session changes without re-reading the YAML.
    pub fn session_info_update(&self) -> i32 {
        unsafe {
            let shared_mem = self.shm.as_ptr();
            if shared_mem.is_null() {
                return -1;
            }
            let offset = std::mem::offset_of!(irsdk_header, session_info_update);
            std::ptr::read_unaligned(shared_mem.add(offset) as *const i32)
        }
    }

    // Find the double-buffer with the highest tick count and return a pointer to its data.
    fn get_latest_data_ptr(&self) -> Option<*const u8> {
        unsafe {
            let shared_mem = self.shm.as_ptr();

            let header = std::ptr::read_unaligned(shared_mem as *const irsdk_header);

            if header.num_buf <= 0 || header.num_buf as usize > IRSDK_MAX_BUFS {
                return None;
            }

            let mut latest_buf_idx = 0;
            let mut max_tick_count = -1;

            for i in 0..header.num_buf as usize {
                let tick_count = header.var_buf[i].tick_count;

                if tick_count > max_tick_count {
                    max_tick_count = tick_count;

                    latest_buf_idx = i;
                }
            }

            if max_tick_count < 0 {
                return None;
            }

            let buf_offset = header.var_buf[latest_buf_idx].buf_offset as usize;

            Some(shared_mem.add(buf_offset))
        }
    }

    /// Read a single telemetry variable by name from the latest data buffer.
    /// Uses `read_unaligned` because shared-memory offsets are not guaranteed
    /// to satisfy Rust's alignment requirements.
    #[doc(hidden)]
    pub fn read_variable(&self, name: &str) -> Option<TelemetryValue> {
        let var = self.vars.get(name)?;
        let data_ptr = self.get_latest_data_ptr()?;
        let offset = var.offset as usize;

        unsafe {
            let ptr = data_ptr.add(offset);
            let count = var.count as usize;

            match VarType::from_i32(var.type_)? {
                VarType::Char => {
                    if var.count_as_char != 0 {
                        let slice = std::slice::from_raw_parts(ptr, count);

                        Some(TelemetryValue::String(parse_c_str(slice)))
                    } else if count == 1 {
                        Some(TelemetryValue::Char(std::ptr::read_unaligned(ptr)))
                    } else {
                        let mut vec = Vec::with_capacity(count);

                        for idx in 0..count {
                            vec.push(std::ptr::read_unaligned(ptr.add(idx)));
                        }

                        Some(TelemetryValue::String(crate::decode_cp1252(&vec)))
                    }
                }

                VarType::Bool => {
                    if count == 1 {
                        Some(TelemetryValue::Bool(std::ptr::read_unaligned(ptr) != 0))
                    } else {
                        let mut vec = Vec::with_capacity(count);

                        for idx in 0..count {
                            vec.push(std::ptr::read_unaligned(ptr.add(idx)) != 0);
                        }

                        Some(TelemetryValue::BoolArray(vec))
                    }
                }

                VarType::Int => {
                    let int_ptr = ptr as *const i32;

                    if count == 1 {
                        Some(TelemetryValue::Int(std::ptr::read_unaligned(int_ptr)))
                    } else {
                        let mut vec = Vec::with_capacity(count);

                        for idx in 0..count {
                            vec.push(std::ptr::read_unaligned(int_ptr.add(idx)));
                        }

                        Some(TelemetryValue::IntArray(vec))
                    }
                }

                VarType::BitField => {
                    let uint_ptr = ptr as *const u32;

                    if count == 1 {
                        Some(TelemetryValue::BitField(std::ptr::read_unaligned(uint_ptr)))
                    } else {
                        let mut vec = Vec::with_capacity(count);

                        for idx in 0..count {
                            let u32_val = std::ptr::read_unaligned(uint_ptr.add(idx));

                            vec.push(u32_val as i32);
                        }

                        Some(TelemetryValue::IntArray(vec))
                    }
                }

                VarType::Float => {
                    let float_ptr = ptr as *const f32;

                    if count == 1 {
                        Some(TelemetryValue::Float(std::ptr::read_unaligned(float_ptr)))
                    } else {
                        let mut vec = Vec::with_capacity(count);

                        for idx in 0..count {
                            vec.push(std::ptr::read_unaligned(float_ptr.add(idx)));
                        }

                        Some(TelemetryValue::FloatArray(vec))
                    }
                }

                VarType::Double => {
                    let double_ptr = ptr as *const f64;

                    if count == 1 {
                        Some(TelemetryValue::Double(std::ptr::read_unaligned(double_ptr)))
                    } else {
                        let mut vec = Vec::with_capacity(count);

                        for idx in 0..count {
                            vec.push(std::ptr::read_unaligned(double_ptr.add(idx)));
                        }

                        Some(TelemetryValue::DoubleArray(vec))
                    }
                }
            }
        }
    }

    /// Snapshot every known variable from the latest data buffer into a map.
    pub(crate) fn read_all_variables(&self) -> HashMap<String, TelemetryValue> {
        let mut map = HashMap::with_capacity(self.vars.len());

        for name in self.vars.keys() {
            if let Some(val) = self.read_variable(name) {
                map.insert(name.clone(), val);
            }
        }
        map
    }

    /// Raw YAML session string from iRacing shared memory.
    pub fn session_yaml(&self) -> Option<String> {
        unsafe {
            let shared_mem = self.shm.as_ptr();
            let header = std::ptr::read_unaligned(shared_mem as *const irsdk_header);

            if header.session_info_len <= 0 {
                return None;
            }

            let info_ptr = shared_mem.add(header.session_info_offset as usize);
            let bytes = std::slice::from_raw_parts(info_ptr, header.session_info_len as usize);

            let len = bytes.iter().position(|&x| x == 0).unwrap_or(bytes.len());

            Some(crate::decode_cp1252(&bytes[..len]))
        }
    }

    /// All current telemetry variables as a map with sim-native names and units.
    pub fn telemetry_snapshot(
        &self,
    ) -> std::collections::HashMap<String, crate::types::TelemetryValue> {
        self.read_all_variables()
    }

    /// Metadata for every telemetry variable iRacing currently exposes.
    pub fn var_list_snapshot(&self) -> Vec<crate::types::VarMeta> {
        use crate::iracing::structs::VarType;

        self.vars
            .iter()
            .map(|(name, hdr)| {
                let type_name = match VarType::from_i32(hdr.type_) {
                    Some(VarType::Char) => "char",
                    Some(VarType::Bool) => "bool",
                    Some(VarType::Int) => "int",
                    Some(VarType::BitField) => "bitfield",
                    Some(VarType::Float) => "float",
                    Some(VarType::Double) => "double",
                    None => "unknown",
                };

                let unit = {
                    let len = hdr
                        .unit
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(hdr.unit.len());

                    crate::decode_cp1252(&hdr.unit[..len])
                };

                let desc = {
                    let len = hdr
                        .desc
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(hdr.desc.len());

                    crate::decode_cp1252(&hdr.desc[..len])
                };

                crate::types::VarMeta {
                    name: name.clone(),
                    type_name,
                    unit,
                    desc,
                    count: hdr.count as u32,
                }
            })
            .collect()
    }

    /// Capture a full telemetry frame. Reads all variables from shared memory in one pass.
    pub(crate) fn frame(
        &self,
    ) -> Result<crate::iracing::types::IracingFrame, crate::error::SimError> {
        let data_ptr = self
            .get_latest_data_ptr()
            .ok_or_else(|| crate::error::SimError::InvalidHeader("No valid data buffer".into()))?;
        Ok(crate::iracing::types::IracingFrame::from_raw(
            data_ptr,
            &self.offsets,
        ))
    }

    /// Read the next telemetry frame, blocking up to `timeout_ms`.
    ///
    /// iRacing is **event-driven**: the call blocks on a Win32 data-ready
    /// event and returns as soon as new data arrives (often <1 ms at 60 Hz).
    /// Pass `0` for a non-blocking read.
    ///
    /// - [`ReadResult::Frame`] — new data arrived and was read successfully.
    /// - [`ReadResult::NotReady`] — `timeout_ms` expired without new data
    ///   (sim may be paused or loading).
    /// - [`ReadResult::Disconnected`] — iRacing stopped broadcasting.
    pub fn read_frame(&self, timeout_ms: u32) -> ReadResult<crate::iracing::types::IracingFrame> {
        if !self.wait_for_data(timeout_ms) {
            if !self.is_connected() {
                return ReadResult::Disconnected;
            }

            return ReadResult::NotReady;
        }

        if !self.is_connected() {
            return ReadResult::Disconnected;
        }

        match self.frame() {
            Ok(frame) => ReadResult::Frame(frame),
            Err(_) => ReadResult::NotReady,
        }
    }

    /// Parse the current session-info YAML into an `IracingSession`.
    ///
    /// Automatically caches the parsed representation and only re-parses the large
    /// YAML block if iRacing reports that the session info has changed.
    pub fn session_info(&self) -> Option<crate::iracing::session::IracingSession> {
        let current_version = self.session_info_update();

        if let Some((_, session)) = self
            .cached_session
            .borrow()
            .as_ref()
            .filter(|(v, _)| *v == current_version)
        {
            return Some(session.clone());
        }

        let yaml = self.session_yaml()?;

        let session = crate::iracing::session::IracingSession::from_yaml(&yaml)?;

        *self.cached_session.borrow_mut() = Some((current_version, session.clone()));

        Some(session)
    }
}

/// Closes the data-valid event handle on drop. The shared-memory region is
/// managed by `SharedMemRegion` and cleaned up automatically.
impl Drop for IRsdkConnection {
    fn drop(&mut self) {
        unsafe {
            if !self.h_event.is_null() {
                CloseHandle(self.h_event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iracing::structs::irsdk_header;
    use std::collections::HashMap;
    use std::mem;

    fn make_header(status: i32) -> Vec<u8> {
        let mut hdr = unsafe { mem::zeroed::<irsdk_header>() };
        hdr.ver = 2;
        hdr.status = status;
        hdr.num_vars = 0;
        hdr.var_header_offset = mem::size_of::<irsdk_header>() as i32;
        let ptr = &hdr as *const irsdk_header as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, mem::size_of::<irsdk_header>()) }.to_vec()
    }

    /// When status bit 0 is set (sim active), `wait_for_data` with no event handle
    /// must sleep ~16 ms and return `true`.
    #[test]
    fn wait_for_data_no_event_connected_returns_true() {
        let mut buf = make_header(1);
        let conn = unsafe { IRsdkConnection::new_mock(buf.as_mut_ptr() as _, HashMap::new()) };
        assert!(conn.wait_for_data(100));
    }

    /// When status bit 0 is clear (sim closed), `wait_for_data` with no event handle
    /// must return `false` — not `true` as the old code did unconditionally.
    #[test]
    fn wait_for_data_no_event_disconnected_returns_false() {
        let mut buf = make_header(0);
        let conn = unsafe { IRsdkConnection::new_mock(buf.as_mut_ptr() as _, HashMap::new()) };
        assert!(!conn.wait_for_data(100));
    }

    /// `is_connected` reflects the status bit independently of the event handle.
    #[test]
    fn is_connected_reads_status_bit() {
        let mut buf_on = make_header(1);
        let conn_on =
            unsafe { IRsdkConnection::new_mock(buf_on.as_mut_ptr() as _, HashMap::new()) };
        assert!(conn_on.is_connected());

        let mut buf_off = make_header(0);
        let conn_off =
            unsafe { IRsdkConnection::new_mock(buf_off.as_mut_ptr() as _, HashMap::new()) };
        assert!(!conn_off.is_connected());
    }
}
