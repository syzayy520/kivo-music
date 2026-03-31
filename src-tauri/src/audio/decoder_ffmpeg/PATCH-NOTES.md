# Ticket: AUD-P0-SHELL-027

## Scope
- ✅ Modified only: `src-tauri/src/audio/decoder_ffmpeg/**`
- ✅ No new dependencies
- ✅ External API unchanged (`FfmpegLoadOptions`, `FfmpegApi::load`, etc.)

## What changed

### `dynlib.rs`
- Refactored Windows implementation into a separate submodule (`dynlib_windows.rs`) to keep files small and single-purpose.
- Public API of `DynLib` unchanged.

### `dynlib_windows.rs`
- **No global side effects:** when an FFmpeg directory is provided, we now:
  1) capture the existing DLL directory (if any),
  2) call `SetDllDirectoryW(temp_dir)` just for `LoadLibraryW`,
  3) **restore the prior DLL directory** on scope exit (success or failure).
- **Serialized temporary overrides:** `SetDllDirectoryW` is process-global, so we guard the critical section with a process-wide `Mutex` to avoid cross-thread races during temporary changes.
- **Better diagnostics:** on failure we return an `AudioError::Io` whose `source` is a `std::io::Error` containing a rich message with:
  - the library name,
  - the attempted directory (if provided),
  - the underlying OS error (`std::io::Error::last_os_error()`).

## How to verify (Windows)

### A) Missing DLL case
Run the existing smoke entry (whichever your repo provides):
```bat
cd src-tauri
cargo run --bin ffmpeg_smoke -- <any_input>
```
Expected: error contains `library=...` + `dir=...` (if provided) + `os_error=...`.

### B) DLL present case
Run the same smoke with FFmpeg DLLs available; expected: prints `version_info` successfully.

## Gates
```bat
cd src-tauri
cargo fmt
cargo test
cargo clippy -- -D warnings
```