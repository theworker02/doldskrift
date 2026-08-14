//! Programmatic and HTML discovery helpers.

use crate::{Mode, PROTOCOL_VERSION};
use serde::{Deserialize, Serialize};

/// Where Doldskrift metadata was found.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    /// `<meta name="doldskrift" content="...">`
    HtmlMeta,
    /// `data-doldskrift` / `data-doldskrift-mode` attributes.
    DataAttributes,
    /// Binary magic / container.
    ContainerMagic,
    /// Heuristic: high density of PUA alphabet symbols.
    PuaHeuristic,
}

/// Structured detection result (prefer this over a bare boolean).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Detection {
    /// True when Doldskrift was detected.
    pub detected: bool,
    /// Protocol version if known.
    pub version: Option<u8>,
    /// Mode if known.
    pub mode: Option<Mode>,
    /// Detection source.
    pub source: Option<DetectionSource>,
    /// Raw content / attribute payload for debugging.
    pub raw: Option<String>,
}

impl Detection {
    /// Negative result.
    pub fn none() -> Self {
        Self {
            detected: false,
            version: None,
            mode: None,
            source: None,
            raw: None,
        }
    }
}

/// Parse `content` from `<meta name="doldskrift" content="version=1;mode=encoded">`.
pub fn detect_from_html_meta(content: &str) -> Detection {
    let mut version = None;
    let mut mode = None;
    for part in content.split(';') {
        let part = part.trim();
        if let Some(v) = part.strip_prefix("version=") {
            version = v.trim().parse().ok();
        } else if let Some(m) = part.strip_prefix("mode=") {
            mode = m.trim().parse().ok();
        }
    }
    if version.is_none() && mode.is_none() && content.trim().is_empty() {
        return Detection::none();
    }
    Detection {
        detected: true,
        version: version.or(Some(PROTOCOL_VERSION)),
        mode,
        source: Some(DetectionSource::HtmlMeta),
        raw: Some(content.to_owned()),
    }
}

/// Detect from `data-doldskrift` and optional `data-doldskrift-mode`.
pub fn detect_from_attrs(version_attr: Option<&str>, mode_attr: Option<&str>) -> Detection {
    match version_attr {
        None => Detection::none(),
        Some(v) => {
            let version = v.parse().ok().or(Some(PROTOCOL_VERSION));
            let mode = mode_attr.and_then(|m| m.parse().ok());
            Detection {
                detected: true,
                version,
                mode,
                source: Some(DetectionSource::DataAttributes),
                raw: Some(format!(
                    "data-doldskrift={v};data-doldskrift-mode={}",
                    mode_attr.unwrap_or("")
                )),
            }
        }
    }
}

/// Heuristic: fraction of PUA alphabet characters above threshold.
pub fn detect_pua_heuristic(text: &str, threshold: f64) -> Detection {
    let total = text.chars().count();
    if total == 0 {
        return Detection::none();
    }
    let symbols = text.chars().filter(|c| crate::is_symbol(*c)).count();
    let ratio = symbols as f64 / total as f64;
    if ratio >= threshold {
        Detection {
            detected: true,
            version: Some(PROTOCOL_VERSION),
            mode: Some(Mode::Encoded),
            source: Some(DetectionSource::PuaHeuristic),
            raw: Some(format!("ratio={ratio:.3}")),
        }
    } else {
        Detection::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_parse() {
        let d = detect_from_html_meta("version=1;mode=encoded");
        assert!(d.detected);
        assert_eq!(d.mode, Some(Mode::Encoded));
    }
}
