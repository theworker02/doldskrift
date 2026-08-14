//! Smoke / integration tests for workflow CLI surface.

use std::process::Command;

fn dold() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dold"))
}

#[test]
fn help_mentions_workflows() {
    let out = dold().arg("--help").output().expect("help");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("pipeline") || s.contains("Workflows"));
    assert!(s.contains("commands") || s.contains("self-test"));
}

#[test]
fn commands_catalog_lists_pipeline() {
    let out = dold().arg("commands").output().expect("commands");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("pipeline"));
    assert!(s.contains("self-test"));
    assert!(s.contains("Document lifecycle"));
}

#[test]
fn topics_alias_works() {
    let out = dold().arg("topics").output().expect("topics");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("pipeline"));
}

#[test]
fn self_test_passes() {
    let out = dold().arg("self-test").output().expect("self-test");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "self-test failed:\n{stdout}\n{stderr}"
    );
    assert!(stdout.contains("passed"));
}

#[test]
fn pipeline_with_text() {
    let tmp = std::env::temp_dir().join(format!("dold-pipe-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let dsk = tmp.join("msg.dsk");
    let out = dold()
        .args([
            "pipeline",
            "--text",
            "hi pipeline",
            "-o",
            dsk.to_str().unwrap(),
        ])
        .output()
        .expect("pipeline");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dsk.is_file());
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn surfaces_list() {
    let out = dold()
        .args(["surfaces", "list"])
        .output()
        .expect("surfaces list");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("postcard"));
}

#[test]
fn schema_prints_envelopes() {
    let out = dold().arg("schema").output().expect("schema");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("doldskrift.inspect/1"));
    assert!(s.contains("doldskrift.validate/1"));
}

#[test]
fn completion_still_builds() {
    let out = dold()
        .args(["completion", "powershell"])
        .output()
        .expect("completion");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("dold") || s.contains("Register-ArgumentCompleter") || !s.is_empty());
}

#[test]
fn guide_quickstart() {
    let out = dold().args(["guide", "quickstart"]).output().expect("guide");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("doctor") || s.contains("concealed"));
    assert!(s.contains("not encryption") || s.contains("NOT encryption") || s.contains("≠"));
}

#[test]
fn demo_pack() {
    let tmp = std::env::temp_dir().join(format!("dold-demo-it-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let out = dold()
        .args(["demo", "-o", tmp.to_str().unwrap(), "--text", "it demo"])
        .output()
        .expect("demo");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(tmp.join("message.dsk").is_file());
    assert!(tmp.join("postcard.svg").is_file());
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn convert_cli() {
    let tmp = std::env::temp_dir().join(format!("dold-conv-it-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let svg = tmp.join("c.svg");
    let out = dold()
        .args([
            "convert",
            "--from",
            "text",
            "--to",
            "postcard",
            "--text",
            "bridge",
            "-o",
            svg.to_str().unwrap(),
        ])
        .output()
        .expect("convert");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(svg.is_file());
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn json_errors_env() {
    let out = dold()
        .env("DOLD_JSON_ERRORS", "1")
        .args(["guide", "not-a-real-topic"])
        .output()
        .expect("json errors");
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("doldskrift.error/1") || err.contains("unknown guide"),
        "{err}"
    );
}
