//! Differential vision — compare two observation / scan graphs.

use crate::ScanReport;
use serde::{Deserialize, Serialize};

/// Diff between two structural scan reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDiffReport {
    /// Left path label.
    pub left: String,
    /// Right path label.
    pub right: String,
    /// Glyph count left.
    pub left_glyphs: usize,
    /// Glyph count right.
    pub right_glyphs: usize,
    /// Positions where symbol ids differ (index, left_cp, right_cp).
    pub symbol_mismatches: Vec<(usize, u32, u32)>,
    /// Absolute difference in mean confidence.
    pub confidence_delta: f64,
    /// Whether reconstructed text matches.
    pub text_equal: bool,
    /// Human summary.
    pub summary: String,
}

/// Compare two scan reports (order-sensitive glyph alignment).
pub fn diff_scan_reports(
    left: &ScanReport,
    right: &ScanReport,
    left_label: &str,
    right_label: &str,
) -> ScanDiffReport {
    let n = left.glyphs.len().max(right.glyphs.len());
    let mut mismatches = Vec::new();
    for i in 0..n {
        let a = left.glyphs.get(i).map(|g| g.symbol);
        let b = right.glyphs.get(i).map(|g| g.symbol);
        match (a, b) {
            (Some(la), Some(rb)) if la != rb => mismatches.push((i, la, rb)),
            (Some(la), None) => mismatches.push((i, la, 0)),
            (None, Some(rb)) => mismatches.push((i, 0, rb)),
            _ => {}
        }
    }
    let text_equal = left.text == right.text;
    let confidence_delta = (left.mean_confidence - right.mean_confidence).abs();
    let summary = format!(
        "Doldskrift Vision Diff\nLeft:             {left_label}\nRight:            {right_label}\nGlyphs:           {} vs {}\nMismatches:       {}\nConfidence Δ:     {confidence_delta:.3}\nText equal:       {text_equal}\nLeft text:        {}\nRight text:       {}",
        left.glyphs.len(),
        right.glyphs.len(),
        mismatches.len(),
        left.text.as_deref().unwrap_or("<none>"),
        right.text.as_deref().unwrap_or("<none>"),
    );
    ScanDiffReport {
        left: left_label.into(),
        right: right_label.into(),
        left_glyphs: left.glyphs.len(),
        right_glyphs: right.glyphs.len(),
        symbol_mismatches: mismatches,
        confidence_delta,
        text_equal,
        summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ScanReport;
    use doldskrift_font::RecognitionResult;

    #[test]
    fn detects_mismatch() {
        let left = ScanReport {
            engine: "structural".into(),
            glyphs: vec![RecognitionResult {
                symbol: 0xE100,
                confidence: 1.0,
                alternatives: vec![],
            }],
            text: Some("a".into()),
            mean_confidence: 1.0,
            checksum_ok: None,
            confidence_buckets: vec![0; 10],
            summary: String::new(),
        };
        let right = ScanReport {
            engine: "structural".into(),
            glyphs: vec![RecognitionResult {
                symbol: 0xE101,
                confidence: 0.9,
                alternatives: vec![],
            }],
            text: Some("b".into()),
            mean_confidence: 0.9,
            checksum_ok: None,
            confidence_buckets: vec![0; 10],
            summary: String::new(),
        };
        let d = diff_scan_reports(&left, &right, "L", "R");
        assert_eq!(d.symbol_mismatches.len(), 1);
        assert!(!d.text_equal);
    }
}
