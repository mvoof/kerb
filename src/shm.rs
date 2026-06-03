use windows_sys::Win32::Foundation::{CloseHandle, FALSE, GetLastError, HANDLE};
use windows_sys::Win32::System::Memory::{
    FILE_MAP_READ, MEM_COMMIT, MEMORY_BASIC_INFORMATION, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile,
    OpenFileMappingW, UnmapViewOfFile, VirtualQuery,
};

/// RAII wrapper around a Windows shared-memory file mapping.
///
/// Calls `OpenFileMappingW` + `MapViewOfFile` on construction and automatically
/// unmaps the view and closes the handle when dropped.
pub struct SharedMemRegion {
    h_map: HANDLE,
    pub view: MEMORY_MAPPED_VIEW_ADDRESS,
}

impl SharedMemRegion {
    /// Open an existing named shared-memory region for reading.
    ///
    /// `name` is the kernel object name the simulator created (e.g.
    /// `"Local\\acpmf_physics"`). Returns an error string if the mapping
    /// cannot be opened — usually because the sim is not running.
    pub fn open(name: &str) -> Result<Self, String> {
        unsafe {
            let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let h_map = OpenFileMappingW(FILE_MAP_READ, FALSE, wide.as_ptr());

            if h_map.is_null() {
                let err = GetLastError();

                return Err(format!(
                    "OpenFileMappingW failed for '{}' (error {})",
                    name, err
                ));
            }

            let view = MapViewOfFile(h_map, FILE_MAP_READ, 0, 0, 0);

            if view.Value.is_null() {
                let err = GetLastError();

                CloseHandle(h_map);

                return Err(format!("MapViewOfFile failed (error {})", err));
            }

            Ok(Self { h_map, view })
        }
    }

    /// Return a raw pointer to the start of the mapped view.
    pub fn as_ptr(&self) -> *const u8 {
        self.view.Value as *const u8
    }

    /// Return the size of the mapped region in bytes via `VirtualQuery`.
    pub fn len(&self) -> usize {
        unsafe {
            let mut info: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
            let ret = VirtualQuery(
                self.view.Value,
                &mut info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );
            if ret == 0 || info.State != MEM_COMMIT {
                return 0;
            }
            info.RegionSize
        }
    }

    /// Create a mock `SharedMemRegion` pointing to an externally-owned buffer (for tests only).
    ///
    /// # Safety
    /// The caller must ensure `ptr` is valid for the lifetime of the returned value.
    #[doc(hidden)]
    pub unsafe fn new_mock(ptr: *mut std::ffi::c_void) -> Self {
        Self {
            h_map: 0 as _,
            view: MEMORY_MAPPED_VIEW_ADDRESS { Value: ptr },
        }
    }
}

/// Unmaps the view and closes the file-mapping handle on drop.
impl Drop for SharedMemRegion {
    fn drop(&mut self) {
        unsafe {
            if !self.view.Value.is_null() {
                UnmapViewOfFile(self.view);
            }

            if !self.h_map.is_null() {
                CloseHandle(self.h_map);
            }
        }
    }
}
