use super::*;

use std::ffi::{c_char, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::ptr;

use super::win_dll_dir::{dll_dir_lock, ScopedDllDirectory};

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
type HMODULE = *mut c_void;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryW(lp_lib_file_name: *const u16) -> HMODULE;
    fn GetProcAddress(h_module: HMODULE, lp_proc_name: *const c_char) -> *mut c_void;
    fn FreeLibrary(h_lib_module: HMODULE) -> i32;
}

/// Rich diagnostic for DLL load failure.
///
/// We keep it `Send + Sync` so it can be stored inside `std::io::Error`.
#[derive(Debug)]
struct LoadLibraryError {
    library: String,
    dir: Option<PathBuf>,
    search: String,
    source: std::io::Error,
}

impl std::fmt::Display for LoadLibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.dir {
            Some(d) => write!(
                f,
                "LoadLibraryW failed: library={} dir={} search={} os_error={}",
                self.library,
                d.display(),
                self.search,
                self.source
            ),
            None => write!(
                f,
                "LoadLibraryW failed: library={} search={} os_error={}",
                self.library, self.search, self.source
            ),
        }
    }
}

impl std::error::Error for LoadLibraryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

pub fn load_library(name: &str, dir: Option<&Path>, search: &str) -> AudioResult<HMODULE> {
    // If a dir is provided, temporarily add it to the DLL search path.
    // IMPORTANT: we MUST restore it after LoadLibrary returns.
    // NOTE: SetDllDirectoryW is process-global; serialize temporary overrides.
    let _lock = dir.map(|_| dll_dir_lock().lock().expect("dll dir mutex poisoned"));

    let _guard = dir.map(ScopedDllDirectory::set).transpose()?;

    let wide_name = {
        let mut w: Vec<u16> = OsStr::new(name).encode_wide().collect();
        w.push(0);
        w
    };

    let handle = unsafe { LoadLibraryW(wide_name.as_ptr()) };
    if handle.is_null() {
        let os_err = std::io::Error::last_os_error();
        let kind = os_err.kind();
        let boxed = LoadLibraryError {
            library: name.to_string(),
            dir: dir.map(|d| d.to_path_buf()),
            search: search.to_string(),
            source: os_err,
        };
        return Err(AudioError::io(
            "ffmpeg.dynlib.load",
            std::io::Error::new(kind, boxed),
        ));
    }

    Ok(handle)
}

pub unsafe fn get_symbol(handle: HMODULE, name: *const c_char) -> *mut c_void {
    if handle.is_null() || name.is_null() {
        return ptr::null_mut();
    }
    GetProcAddress(handle, name)
}

pub unsafe fn free_library(handle: HMODULE) {
    if !handle.is_null() {
        FreeLibrary(handle);
    }
}
