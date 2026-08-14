//! Embed build metadata for `dold version --verbose`.

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=PROFILE");
    if let Ok(t) = std::env::var("TARGET") {
        println!("cargo:rustc-env=DOLDSKRIFT_TARGET={t}");
    }
    if let Ok(p) = std::env::var("PROFILE") {
        println!("cargo:rustc-env=DOLDSKRIFT_PROFILE={p}");
    }
    let git = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=DOLDSKRIFT_GIT={git}");
    let rustc = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=DOLDSKRIFT_RUSTC={rustc}");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
}
