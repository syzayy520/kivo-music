use crate::audio::errors::{AudioError, AudioResult};

use std::ffi::{c_void, CString};
use std::path::Path;

/// Minimal dynamic library loader (Windows only) used by the FFmpeg decoder.
///
/// Notes:
/// - No external dependencies.
/// - On Windows, we may temporarily adjust the DLL search directory while
///   calling `LoadLibraryW`, then restore it to avoid process-global pollution.
#[derive(Debug)]
pub struct DynLib {
    name: String,
    search: String,
    handle: *mut c_void,
}

impl DynLib {
    pub fn load(name: &str, dir: Option<&Path>, search: &str) -> AudioResult<Self> {
        let handle = load_library(name, dir, search)?;
        Ok(Self {
            name: name.to_string(),
            search: search.to_string(),
            handle,
        })
    }

    /// Lookup a symbol and cast it to the requested function-pointer type.
    pub unsafe fn sym<T: Copy>(&self, symbol: &str) -> AudioResult<T> {
        self.get(symbol)
    }

    pub unsafe fn get<T: Copy>(&self, symbol: &str) -> AudioResult<T> {
        let cs = CString::new(symbol).map_err(|_| {
            AudioError::io(
                "ffmpeg.dynlib.symbol",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "symbol contains NUL"),
            )
        })?;

        let ptr = get_symbol(self.handle, cs.as_ptr());
        if ptr.is_null() {
            let os_error = {
                #[cfg(windows)]
                {
                    let e = std::io::Error::last_os_error();
                    match e.raw_os_error() {
                        Some(code) if code != 0 => Some(format!("{} (win32={code})", e)),
                        _ => None,
                    }
                }
                #[cfg(not(windows))]
                {
                    None
                }
            };

            return Err(AudioError::FfmpegSymbolMissing {
                library: self.name.clone(),
                symbol: symbol.to_string(),
                search: self.search.clone(),
                os_error,
            });
        }

        // SAFETY: caller only requests function-pointer types.
        Ok(std::mem::transmute_copy(&ptr))
    }
}

impl Drop for DynLib {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.handle.is_null() {
                free_library(self.handle);
            }
        }
    }
}

#[cfg(windows)]
mod dynlib_windows;
#[cfg(windows)]
#[path = "win_dll_dir.rs"]
mod win_dll_dir;

#[cfg(windows)]
use dynlib_windows::{free_library, get_symbol, load_library};

#[cfg(not(windows))]
fn load_library(_name: &str, _dir: Option<&Path>, _search: &str) -> AudioResult<*mut c_void> {
    Err(AudioError::io(
        "ffmpeg.dynlib.load",
        std::io::Error::new(std::io::ErrorKind::Unsupported, "DynLib is Windows-only"),
    ))
}

#[cfg(not(windows))]
unsafe fn get_symbol(_handle: *mut c_void, _name: *const std::ffi::c_char) -> *mut c_void {
    std::ptr::null_mut()
}

#[cfg(not(windows))]
unsafe fn free_library(_handle: *mut c_void) {}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn load_nonexistent_dir_includes_dll_and_searched_dir() {
        // Use a path that is extremely unlikely to exist.
        let dir = std::env::temp_dir().join(format!(
            "kivo_no_such_ffmpeg_dir_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        ));

        // Ensure it doesn't exist.
        if dir.exists() {
            let _ = std::fs::remove_dir_all(&dir);
        }

        let err = DynLib::load("avutil-59.dll", Some(dir.as_path()), "unit-test").unwrap_err();
        let s = err.to_string();

        // Requirement: error text should contain dll name + searched dir (for定位).
        assert!(s.contains("dll=avutil-59.dll") || s.contains("library=avutil-59.dll"));
        assert!(s.contains("searched_dir=") || s.contains(&dir.display().to_string()));
    }
}
