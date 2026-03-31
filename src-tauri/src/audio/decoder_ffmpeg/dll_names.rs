/// Candidate DLL base names for common FFmpeg builds.
///
/// We try in order to maximize compatibility with different FFmpeg distributions.
pub fn avformat_candidates() -> &'static [&'static str] {
    &[
        "avformat-60.dll",
        "avformat-61.dll",
        "avformat-59.dll",
        "avformat-58.dll",
        "avformat.dll",
    ]
}

pub fn avcodec_candidates() -> &'static [&'static str] {
    &[
        "avcodec-60.dll",
        "avcodec-61.dll",
        "avcodec-59.dll",
        "avcodec-58.dll",
        "avcodec.dll",
    ]
}

pub fn avutil_candidates() -> &'static [&'static str] {
    &[
        "avutil-58.dll",
        "avutil-59.dll",
        "avutil-57.dll",
        "avutil-56.dll",
        "avutil.dll",
    ]
}

pub fn swresample_candidates() -> &'static [&'static str] {
    &[
        "swresample-4.dll",
        "swresample-5.dll",
        "swresample-3.dll",
        "swresample.dll",
    ]
}
