// format_fields.rs
//
// Format field extraction from WAVEFORMATEX.
//
// This file contains the FormatFields struct and logic to safely read
// format fields from a WAVEFORMATEX pointer, handling packed struct
// alignment issues.

use windows::Win32::Media::Audio::WAVEFORMATEX;

/// Format fields extracted from a WAVEFORMATEX pointer.
///
/// Uses named fields instead of positional tuple to avoid
/// passing 7+ separate parameters across function boundaries.
#[derive(Clone, Copy)]
pub struct FormatFields {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub avg_bytes_per_sec: u32,
    pub format_tag: u16,
    pub cb_size: u16,
}

/// Extract basic format fields from a WAVEFORMATEX pointer.
///
/// WAVEFORMATEX is a packed struct, so fields are copied to a
/// FormatFields struct before being returned to avoid unaligned
/// reference issues.
///
/// # Safety
/// The caller must ensure the pointer is valid and non-null.
pub unsafe fn extract_format_fields(ptr: *mut WAVEFORMATEX) -> FormatFields {
    let format = unsafe { &*ptr };
    FormatFields {
        sample_rate_hz: format.nSamplesPerSec,
        channels: format.nChannels,
        bits_per_sample: format.wBitsPerSample,
        block_align: format.nBlockAlign,
        avg_bytes_per_sec: format.nAvgBytesPerSec,
        format_tag: format.wFormatTag,
        cb_size: format.cbSize,
    }
}
