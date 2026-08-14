//! DSK Beacon — fast “I contain Doldskrift” visual marker.

/// Canonical beacon pattern bytes (logical; rendered by vision/font layers).
pub const BEACON_PATTERN: &[u8] = b"DSKB\x01\xE1\xF0";

/// Detected beacon.
#[derive(Debug, Clone, PartialEq)]
pub struct Beacon {
    /// Center (x, y) in image pixels.
    pub center: (u32, u32),
    /// Confidence 0.0–1.0.
    pub confidence: f32,
}

/// Scan a feature map / byte grid for the beacon pattern (platform-independent).
///
/// `rows` is a row-major grid of quantized cell tags. Returns centers of matches.
pub fn detect_beacon(rows: &[Vec<u8>], cell_w: u32, cell_h: u32) -> Vec<Beacon> {
    let mut found = Vec::new();
    if rows.is_empty() {
        return found;
    }
    let h = rows.len();
    let w = rows[0].len();
    let pat = BEACON_PATTERN;
    if w < pat.len() {
        return found;
    }
    for (y, row) in rows.iter().enumerate().take(h) {
        if row.len() < pat.len() {
            continue;
        }
        for x in 0..=row.len() - pat.len() {
            if &row[x..x + pat.len()] == pat {
                found.push(Beacon {
                    center: (
                        (x as u32 + pat.len() as u32 / 2) * cell_w,
                        y as u32 * cell_h + cell_h / 2,
                    ),
                    confidence: 1.0,
                });
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_beacon() {
        let mut row = vec![0u8; 20];
        row[5..5 + BEACON_PATTERN.len()].copy_from_slice(BEACON_PATTERN);
        let hits = detect_beacon(&[row], 8, 8);
        assert_eq!(hits.len(), 1);
        assert!(hits[0].confidence > 0.9);
    }
}
