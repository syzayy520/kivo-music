// output_thread_boundary/format_fields.rs
//
// Format fields extraction from WAVEFORMATEX pointer.

use windows::Win32::Media::Audio::WAVEFORMATEX;

/// Extracted format fields from WAVEFORMATEX.
#[derive(Clone, Copy, Debug)]
pub struct FormatFields {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub avg_bytes_per_sec: u32,
    pub format_tag: u16,
    pub cb_size: u16,
}

/// Extract format fields from a WAVEFORMATEX pointer.
///
/// # Safety
/// The pointer must be valid and point to a properly initialized WAVEFORMATEX structure.
pub unsafe fn extract_format_fields(ptr: *const WAVEFORMATEX) -> FormatFields {
    let format = &*ptr;
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
