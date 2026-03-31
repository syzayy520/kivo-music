use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::sync::{Mutex, OnceLock};

#[link(name = "kernel32")]
extern "system" {
    fn SetDllDirectoryW(lp_path_name: *const u16) -> i32;
    fn GetDllDirectoryW(n_buffer_length: u32, lp_buffer: *mut u16) -> u32;
}

pub(crate) struct ScopedDllDirectory {
    prev: Option<Vec<u16>>,
}

impl ScopedDllDirectory {
    pub(crate) fn set(temp_dir: &Path) -> std::io::Result<Self> {
        let prev = get_dll_directory_utf16()?;
        set_dll_directory_utf16(Some(path_to_utf16_nul(temp_dir)))?;
        Ok(Self { prev })
    }
}

impl Drop for ScopedDllDirectory {
    fn drop(&mut self) {
        let _ = set_dll_directory_utf16(self.prev.take());
    }
}

pub(crate) fn dll_dir_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn path_to_utf16_nul(p: &Path) -> Vec<u16> {
    let mut wide: Vec<u16> = OsStr::new(p).encode_wide().collect();
    wide.push(0);
    wide
}

fn set_dll_directory_utf16(path: Option<Vec<u16>>) -> std::io::Result<()> {
    let ptr = match path.as_ref() {
        Some(v) => v.as_ptr(),
        None => ptr::null(),
    };
    let ok = unsafe { SetDllDirectoryW(ptr) };
    if ok == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

fn get_dll_directory_utf16() -> std::io::Result<Option<Vec<u16>>> {
    let mut buf: Vec<u16> = vec![0u16; 260];
    let n = unsafe { GetDllDirectoryW(buf.len() as u32, buf.as_mut_ptr()) };
    if n == 0 {
        return Ok(None);
    }
    if (n as usize) >= buf.len() {
        buf = vec![0u16; (n as usize) + 1];
        let n2 = unsafe { GetDllDirectoryW(buf.len() as u32, buf.as_mut_ptr()) };
        if n2 == 0 {
            return Ok(None);
        }
        buf.truncate((n2 as usize) + 1);
        if *buf.last().unwrap_or(&0) != 0 {
            buf.push(0);
        }
        return Ok(Some(buf));
    }

    buf.truncate((n as usize) + 1);
    if *buf.last().unwrap_or(&0) != 0 {
        buf.push(0);
    }
    Ok(Some(buf))
}
