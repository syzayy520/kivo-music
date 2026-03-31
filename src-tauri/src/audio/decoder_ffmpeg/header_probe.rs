use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

/// Best-effort sample-rate probe by inspecting file headers.
///
/// This is used for the DEC-002 WAV writer demo. In later integration,
/// sample rate should come from the decoded stream itself.
pub fn probe_sample_rate(path: &Path) -> Option<u32> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "mp3" => probe_mp3_sample_rate(path),
        "flac" => probe_flac_sample_rate(path),
        "wav" => probe_wav_sample_rate(path),
        _ => None,
    }
}

fn probe_mp3_sample_rate(path: &Path) -> Option<u32> {
    let mut f = File::open(path).ok()?;
    let mut buf = [0u8; 4096];
    let n = f.read(&mut buf).ok()?;
    let b = &buf[..n];

    let mut i = 0usize;
    if n >= 10 && &b[..3] == b"ID3" {
        let size = ((b[6] as usize & 0x7f) << 21)
            | ((b[7] as usize & 0x7f) << 14)
            | ((b[8] as usize & 0x7f) << 7)
            | (b[9] as usize & 0x7f);
        i = 10 + size;
    }

    while i + 4 <= n {
        if b[i] == 0xff && (b[i + 1] & 0xe0) == 0xe0 {
            let ver = (b[i + 1] >> 3) & 0x03;
            let sr = (b[i + 2] >> 2) & 0x03;
            let sample_rate = match (ver, sr) {
                // MPEG1
                (0b11, 0) => 44100,
                (0b11, 1) => 48000,
                (0b11, 2) => 32000,
                // MPEG2
                (0b10, 0) => 22050,
                (0b10, 1) => 24000,
                (0b10, 2) => 16000,
                // MPEG2.5
                (0b00, 0) => 11025,
                (0b00, 1) => 12000,
                (0b00, 2) => 8000,
                _ => 0,
            };
            if sample_rate != 0 {
                return Some(sample_rate);
            }
        }
        i += 1;
    }
    None
}

fn probe_flac_sample_rate(path: &Path) -> Option<u32> {
    let mut f = File::open(path).ok()?;
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic).ok()?;
    if &magic != b"fLaC" {
        return None;
    }

    let mut header = [0u8; 4];
    f.read_exact(&mut header).ok()?;
    let block_type = header[0] & 0x7f;
    let length = ((header[1] as usize) << 16) | ((header[2] as usize) << 8) | (header[3] as usize);
    if block_type != 0 || length < 18 {
        return None;
    }

    // Skip min/max blocksize + min/max framesize (10 bytes)
    f.seek(SeekFrom::Current(10)).ok()?;
    let mut sr_bytes = [0u8; 4];
    f.read_exact(&mut sr_bytes).ok()?;
    let sample_rate =
        ((sr_bytes[0] as u32) << 12) | ((sr_bytes[1] as u32) << 4) | ((sr_bytes[2] as u32) >> 4);
    if sample_rate == 0 {
        None
    } else {
        Some(sample_rate)
    }
}

fn probe_wav_sample_rate(path: &Path) -> Option<u32> {
    let mut f = File::open(path).ok()?;
    let mut hdr = [0u8; 12];
    f.read_exact(&mut hdr).ok()?;
    if &hdr[0..4] != b"RIFF" || &hdr[8..12] != b"WAVE" {
        return None;
    }

    loop {
        let mut chunk_hdr = [0u8; 8];
        if f.read_exact(&mut chunk_hdr).is_err() {
            break;
        }
        let id = &chunk_hdr[0..4];
        let size = u32::from_le_bytes(chunk_hdr[4..8].try_into().ok()?) as u64;

        if id == b"fmt " {
            let mut buf = vec![0u8; size.min(32) as usize];
            f.read_exact(&mut buf).ok()?;
            if buf.len() < 12 {
                return None;
            }
            return Some(u32::from_le_bytes(buf[4..8].try_into().ok()?));
        } else {
            let skip = size + (size % 2);
            f.seek(SeekFrom::Current(skip as i64)).ok()?;
        }
    }
    None
}
