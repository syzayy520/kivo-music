use super::types::TrackHit;

pub fn search_prefix(tracks: &[TrackHit], q: &str, limit: u32) -> Vec<TrackHit> {
    let q = q.trim();
    if q.is_empty() || limit == 0 {
        return Vec::new();
    }
    let qn = q.to_ascii_lowercase();
    let lim = limit as usize;

    let mut out: Vec<TrackHit> = Vec::new();
    for t in tracks {
        if out.len() >= lim {
            break;
        }
        let title = t.title.to_ascii_lowercase();
        if title.starts_with(&qn) {
            out.push(t.clone());
            continue;
        }
        // Fallback: prefix match on filename (path tail)
        if let Some(name) = t
            .path
            .rsplit(|c| c == '/' || c == '\\')
            .next()
            .map(|s| s.to_ascii_lowercase())
        {
            if name.starts_with(&qn) {
                out.push(t.clone());
            }
        }
    }
    out
}
