//! DVE/2 scaffold — perception pipeline toward [`ObservationGraph`].
//!
//! Stages (foundation stubs): detect → normalize → segment → topology → relationships.
//! Perception only — does **not** reconstruct semantics.
//!
//! Beyond latent carriers, [`perceive_svg_geometry`] builds an observation from
//! rendered SVG path / primitive geometry (still **no camera**).

use doldskrift::neural::{
    LatentGraph, ObservationEdge, ObservationGraph, ObservationNode, ObservationNodeKind,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// DVE/2 engine label.
pub const DVE2_ENGINE: &str = "DVE/2";

/// Pipeline stage status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dve2Stage {
    /// Region detection.
    Detect,
    /// Geometric normalize.
    Normalize,
    /// Segment marks.
    Segment,
    /// Topology extraction.
    Topology,
    /// Relationship graph build.
    Relationships,
}

/// Report from a DVE/2 run (stub).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dve2Report {
    /// Engine label.
    pub engine: String,
    /// Stages that were executed (stubs may skip real work).
    pub stages: Vec<String>,
    /// Resulting observation graph.
    pub observation: ObservationGraph,
    /// Explicit TODOs.
    pub todos: Vec<String>,
}

/// Lightweight geometry hint extracted from SVG markup (no raster / no camera).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SvgGeometryHint {
    /// Approximate mark count (circles, paths, glyph groups).
    pub mark_count: usize,
    /// Path / polyline element count.
    pub path_count: usize,
    /// Circle elements.
    pub circle_count: usize,
    /// Line elements.
    pub line_count: usize,
    /// Whether a `dsk-carrier` desc is present.
    pub has_carrier: bool,
    /// Bounding-box stub from viewBox if present `[x,y,w,h]`.
    pub view_box: Option<[f64; 4]>,
}

/// Run the synthetic DVE/2 path that wraps a latent carrier (CI / foundation).
///
/// This is **not** a camera pipeline. Real detect/normalize/segment remain TODO.
pub fn perceive_latent_carrier(graph: LatentGraph) -> Dve2Report {
    let observation = ObservationGraph::from_latent_carrier(graph);
    Dve2Report {
        engine: DVE2_ENGINE.into(),
        stages: vec![
            "detect:stub".into(),
            "normalize:stub".into(),
            "segment:stub".into(),
            "topology:stub".into(),
            "relationships:stub".into(),
        ],
        observation,
        todos: vec![
            "Implement detect → normalize → segment on raster / camera frames".into(),
            "Build topology and relationship edges from segmented marks".into(),
            "Remove dependence on latent_carrier for production perception".into(),
        ],
    }
}

/// Classify a random / non-Doldskrift observation.
pub fn perceive_unknown() -> Dve2Report {
    Dve2Report {
        engine: DVE2_ENGINE.into(),
        stages: vec!["detect:empty".into()],
        observation: ObservationGraph::not_doldskrift(),
        todos: vec!["Wire real negative detectors for camera noise".into()],
    }
}

/// Observe SVG path / shape geometry without a camera.
///
/// Counts structural elements and builds an [`ObservationGraph`] with geometry attrs.
/// If a latent carrier is supplied, DeterministicBaseline can still reconstruct.
/// This is **not** pixel vision and **not** a security boundary.
pub fn perceive_svg_geometry(svg: &str, latent: Option<LatentGraph>) -> Dve2Report {
    let hint = analyze_svg_geometry(svg);
    let mut nodes = Vec::new();
    let mut attrs = BTreeMap::new();
    attrs.insert("mark_count".into(), hint.mark_count.to_string());
    attrs.insert("path_count".into(), hint.path_count.to_string());
    attrs.insert("circle_count".into(), hint.circle_count.to_string());
    attrs.insert("line_count".into(), hint.line_count.to_string());
    attrs.insert("has_carrier".into(), hint.has_carrier.to_string());
    attrs.insert("source".into(), "svg-geometry".into());

    nodes.push(ObservationNode {
        id: "geom0".into(),
        kind: if hint.mark_count > 0 || hint.has_carrier {
            ObservationNodeKind::Group
        } else {
            ObservationNodeKind::Unknown
        },
        fingerprint: Some("DSK-SVG-GEOM".into()),
        bbox: hint.view_box,
        attrs: attrs.clone(),
    });

    // Segment-like: one observation node per path element (capped)
    let path_cap = hint.path_count.min(16);
    for i in 0..path_cap {
        let mut a = BTreeMap::new();
        a.insert("primitive".into(), "path".into());
        a.insert("index".into(), i.to_string());
        nodes.push(ObservationNode {
            id: format!("path{i}"),
            kind: ObservationNodeKind::Mark,
            fingerprint: None,
            bbox: None,
            attrs: a,
        });
    }
    let circle_cap = hint.circle_count.min(12);
    for i in 0..circle_cap {
        let mut a = BTreeMap::new();
        a.insert("primitive".into(), "circle".into());
        a.insert("index".into(), i.to_string());
        nodes.push(ObservationNode {
            id: format!("circ{i}"),
            kind: ObservationNodeKind::Mark,
            fingerprint: None,
            bbox: None,
            attrs: a,
        });
    }

    let edges = stub_relationships(&nodes);
    let is_dsk = hint.has_carrier || hint.mark_count > 0;
    let epoch = latent.as_ref().map(|g| g.meta.epoch);
    let observation = ObservationGraph {
        epoch,
        is_doldskrift: is_dsk,
        nodes,
        edges,
        latent_carrier: latent,
        notes: Some(
            "SVG geometry observation — path/circle/line counts; not camera perception".into(),
        ),
    };

    Dve2Report {
        engine: DVE2_ENGINE.into(),
        stages: vec![
            "detect:svg-tags".into(),
            "normalize:viewBox".into(),
            "segment:path-circle".into(),
            "topology:adjacent-stub".into(),
            "relationships:stub".into(),
        ],
        observation,
        todos: vec![
            "Parse path `d` commands into stroke topology (not just tag counts)".into(),
            "Camera / raster DVE/2 remains separate and unfinished".into(),
        ],
    }
}

/// Count SVG geometry tags and parse a simple viewBox.
pub fn analyze_svg_geometry(svg: &str) -> SvgGeometryHint {
    let lower = svg.to_ascii_lowercase();
    let path_count = count_tag(&lower, "<path");
    let circle_count = count_tag(&lower, "<circle");
    let line_count = count_tag(&lower, "<line");
    let g_count = count_tag(&lower, "<g");
    let has_carrier = svg.contains("id=\"dsk-carrier\"") || svg.contains("id='dsk-carrier'");
    let mark_count = path_count + circle_count + line_count + g_count.saturating_div(2);
    let view_box = parse_view_box(svg);
    SvgGeometryHint {
        mark_count,
        path_count,
        circle_count,
        line_count,
        has_carrier,
        view_box,
    }
}

fn count_tag(hay: &str, tag: &str) -> usize {
    hay.matches(tag).count()
}

fn parse_view_box(svg: &str) -> Option<[f64; 4]> {
    let key = "viewBox=\"";
    let idx = svg.find(key).or_else(|| svg.find("viewbox=\""))?;
    let after = &svg[idx + key.len()..];
    let end = after.find('"')?;
    let parts: Vec<f64> = after[..end]
        .split_whitespace()
        .filter_map(|p| p.parse().ok())
        .collect();
    if parts.len() == 4 {
        Some([parts[0], parts[1], parts[2], parts[3]])
    } else {
        None
    }
}

/// Build a minimal relationship graph from observation nodes (stub helper).
pub fn stub_relationships(nodes: &[ObservationNode]) -> Vec<ObservationEdge> {
    let mut edges = Vec::new();
    for w in nodes.windows(2) {
        edges.push(ObservationEdge {
            from: w[0].id.clone(),
            to: w[1].id.clone(),
            rel: "adjacent".into(),
            confidence: 0.5,
        });
    }
    let _ = ObservationNodeKind::Mark; // keep kind in API surface
    edges
}

#[cfg(test)]
mod tests {
    use super::*;
    use doldskrift::neural::{compose_deterministic, DeterministicBaseline, ReaderBackend};
    use doldskrift::semantic::Value;

    #[test]
    fn dve2_carrier_round_trip() {
        let value = Value::String("dve2".into());
        let latent = compose_deterministic(&value).unwrap();
        let report = perceive_latent_carrier(latent);
        assert_eq!(report.engine, DVE2_ENGINE);
        let out = DeterministicBaseline::new()
            .reconstruct(&report.observation)
            .unwrap();
        assert!(out.ok());
        assert_eq!(out.value, Some(value));
    }

    #[test]
    fn dve2_unknown() {
        let report = perceive_unknown();
        assert!(!report.observation.is_doldskrift);
    }

    #[test]
    fn dve2_svg_geometry_counts() {
        let svg = r##"<svg viewBox="0 0 100 80" xmlns="http://www.w3.org/2000/svg">
  <desc id="dsk-carrier">{"kind":"test","encoded":"x","note":"n"}</desc>
  <circle cx="10" cy="10" r="2"/>
  <circle cx="20" cy="20" r="2"/>
  <path d="M0 0 L10 10"/>
  <line x1="0" y1="0" x2="1" y2="1"/>
  <g transform="translate(1,1)"></g>
</svg>"##;
        let report = perceive_svg_geometry(svg, None);
        assert!(report.observation.is_doldskrift);
        let hint = analyze_svg_geometry(svg);
        assert_eq!(hint.circle_count, 2);
        assert_eq!(hint.path_count, 1);
        assert_eq!(hint.line_count, 1);
        assert!(hint.has_carrier);
        assert_eq!(hint.view_box, Some([0.0, 0.0, 100.0, 80.0]));
        assert!(report.stages.iter().any(|s| s.contains("svg")));
    }

    #[test]
    fn dve2_svg_with_latent_reconstructs() {
        let value = Value::String("geom".into());
        let latent = compose_deterministic(&value).unwrap();
        let svg = r#"<svg viewBox="0 0 10 10"><circle cx="1" cy="1" r="1"/></svg>"#;
        let report = perceive_svg_geometry(svg, Some(latent));
        let out = DeterministicBaseline::new()
            .reconstruct(&report.observation)
            .unwrap();
        assert!(out.ok());
        assert_eq!(out.value, Some(value));
    }
}
