use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActiveLyricLine {
    pub id: String,
    pub starts_at_ms: u64,
    pub text: String,
}

pub fn find_active_lyric_line(
    lines: &[ActiveLyricLine],
    position_ms: u64,
) -> Option<ActiveLyricLine> {
    lines
        .iter()
        .rfind(|line| line.starts_at_ms <= position_ms)
        .cloned()
}
