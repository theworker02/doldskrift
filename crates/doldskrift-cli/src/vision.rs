//! `dold vision` / `dold scan` / `dold bench vision`.

use doldskrift::FONT_VERSION;
use doldskrift_font::generate_alphabet;
use doldskrift_vision::{bench_vision, diff_scan_reports, scan_image, ScanEngine};
use std::path::Path;

pub fn analyze_image(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    scan_path(path, "structural", false)
}

pub fn scan_path(
    path: &Path,
    engine: &str,
    json: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let eng = ScanEngine::parse(engine)?;
    let catalog = generate_alphabet(FONT_VERSION, 1)?;
    let report = scan_image(path, eng, &catalog)?;
    if json {
        Ok(serde_json::to_string_pretty(&report)?)
    } else {
        Ok(report.summary)
    }
}

pub fn diff_paths(
    left: &Path,
    right: &Path,
    engine: &str,
    json: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let eng = ScanEngine::parse(engine)?;
    let catalog = generate_alphabet(FONT_VERSION, 1)?;
    let a = scan_image(left, eng, &catalog)?;
    let b = scan_image(right, eng, &catalog)?;
    let diff = diff_scan_reports(
        &a,
        &b,
        &left.display().to_string(),
        &right.display().to_string(),
    );
    if json {
        Ok(serde_json::to_string_pretty(&diff)?)
    } else {
        Ok(diff.summary)
    }
}

pub fn run_bench(samples: usize) -> Result<String, Box<dyn std::error::Error>> {
    let report = bench_vision(1, samples)?;
    Ok(report.display())
}
