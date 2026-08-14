//! `dold conformance` — run golden vectors.

use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn run(vectors_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !vectors_dir.is_dir() {
        return Err(format!("vectors directory not found: {}", vectors_dir.display()).into());
    }
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut entries: Vec<_> = fs::read_dir(vectors_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x == "json")
        })
        .collect();
    entries.sort();
    if entries.is_empty() {
        return Err("no vector JSON files found".into());
    }
    for path in entries {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        match run_one(&path) {
            Ok(()) => {
                println!("PASS  {name}");
                passed += 1;
            }
            Err(e) => {
                println!("FAIL  {name}: {e}");
                failed += 1;
            }
        }
    }
    println!();
    println!("Conformance: {passed} passed, {failed} failed");
    if failed > 0 {
        return Err("conformance failures".into());
    }
    Ok(())
}

fn run_one(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let v: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let mode = v["mode"].as_str().unwrap_or("encoded");
    let input = v["input"].as_str().ok_or("missing input")?;
    match mode {
        "encoded" => {
            let encoded = doldskrift::encode(input)?;
            if let Some(expected) = v["encoded"].as_str() {
                if encoded != expected {
                    return Err("encoded mismatch".into());
                }
            }
            let decoded = doldskrift::decode(&encoded)?;
            if decoded != input {
                return Err("roundtrip mismatch".into());
            }
            if let Some(cs) = v["checksum"].as_str() {
                let actual = format!("{:08x}", doldskrift::checksum_bytes(input.as_bytes()));
                if actual != cs.to_ascii_lowercase() {
                    return Err(format!("checksum mismatch: {actual} != {cs}").into());
                }
            }
        }
        "session" => {
            let seed = v["seed"].as_str().unwrap_or("");
            let session = doldskrift::Session::from_seed(seed.as_bytes(), "vector")?;
            let encoded = session.encode(input)?;
            if let Some(expected) = v["encoded"].as_str() {
                if encoded != expected {
                    return Err("session encoded mismatch".into());
                }
            }
            if session.decode(&encoded)? != input {
                return Err("session roundtrip mismatch".into());
            }
        }
        "visual" => {
            if doldskrift::encode_with_mode(input, doldskrift::Mode::Visual)? != input {
                return Err("visual should be identity".into());
            }
        }
        other => return Err(format!("unknown mode {other}").into()),
    }
    Ok(())
}
