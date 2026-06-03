// format_fields.rs
//
// Format field extraction from WAVEFORMATEX.
//
// This file contains logic to safely read format fields from a
// WAVEFORMATEX pointer, handling packed struct alignment issues.

use windows::Win32::Media::Audio::WAVEFORMATEX;

/// Extract basic format fields from a WAVEFORMATEX pointer.
///
/// WAVEFORMATEX is a packed struct, so fields are copied to local
/// variables before being returned to avoid unaligned reference issues.
///
/// # Safety
/// The caller must ensure the pointer is valid and non-null.
pub unsafe fn extract_format_fields(ptr: *mut WAVEFORMATEX) -> (u32, u16, u16, u16, u32, u16, u16) {
    let format = unsafe { &*ptr };
    let sample_rate_hz = format.nSamplesPerSec;
    let channels = format.nChannels;
    let bits_per_sample = format.wBitsPerSample;
    let format_tag = format.wFormatTag;
    let cb_size = format.cbSize;
    let block_align = format.nBlockAlign;
    let avg_bytes_per_sec = format.nAvgBytesPerSec;

    (
        sample_rate_hz,
        channels,
        bits_per_sample,
        block_align,
        avg_bytes_per_sec,
        format_tag,
        cb_size,
    )
}
