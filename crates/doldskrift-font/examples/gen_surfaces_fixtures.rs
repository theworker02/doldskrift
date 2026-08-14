//! Regenerate small SVG fixtures under `site/public/surfaces/` from the CLI renderers.
//!
//! ```text
//! cargo run -p doldskrift-font --example gen_surfaces_fixtures
//! ```
//!
//! Keep fixtures tiny (short payloads) for GH Pages.

use doldskrift::encode;
use doldskrift_font::{
    render_ambient_constellation_ex, render_duet_svg, render_flicker_svg, render_kaleidoscope_svg,
    render_notarize_svg, render_postcard_svg, render_seal_svg, render_spectrogram_svg,
    render_timeline_svg,
};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = repo_surfaces_dir();
    fs::create_dir_all(&out).expect("create surfaces dir");

    let write = |name: &str, svg: String| {
        let path = out.join(name);
        fs::write(&path, svg).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        println!("wrote {}", path.display());
    };

    let ambient_text = "stars";
    let enc = encode(ambient_text).unwrap();
    let digest = hex_sha(ambient_text);
    write(
        "ambient-sample.svg",
        render_ambient_constellation_ex(&enc, 1, Some(&digest), false),
    );
    write(
        "ambient-animated-sample.svg",
        render_ambient_constellation_ex(&enc, 1, Some(&digest), true),
    );

    let flick = encode("tick").unwrap();
    write("flicker-sample.svg", render_flicker_svg(&flick, 1).unwrap());

    let post = encode("hi").unwrap();
    write(
        "postcard-sample.svg",
        render_postcard_svg(&post, 1, &hex_sha("hi")),
    );

    let a = encode("a").unwrap();
    let b = encode("b").unwrap();
    write("duet-sample.svg", render_duet_svg(&a, &b, 1).unwrap());
    // Echo fixture: notarize-style integrity carrier (CLI `dold echo` builds a richer strip)
    let echo_enc = encode("echo").unwrap();
    let echo_digest = hex_sha("echo");
    write(
        "echo-sample.svg",
        render_notarize_svg(&echo_enc, &echo_digest, 1)
            .unwrap()
            .replacen("notarize-margin", "echo-integrity", 1),
    );
    // Mesh gallery fixture: ambient stand-in (full LSG mesh via `dold mesh`)
    write(
        "mesh-sample.svg",
        render_ambient_constellation_ex(&encode("mesh").unwrap(), 2, None, false),
    );
    write(
        "spectrogram-sample.svg",
        render_spectrogram_svg(&encode("spec").unwrap(), 1),
    );

    write(
        "kaleidoscope-sample.svg",
        render_kaleidoscope_svg(&encode("kal").unwrap(), 1).unwrap(),
    );
    write(
        "timeline-sample.svg",
        render_timeline_svg(&encode("time").unwrap(), &[1, 2, 3]).unwrap(),
    );
    write(
        "seal-sample.svg",
        render_seal_svg(&encode("seal").unwrap(), &hex_sha("seal"), 1).unwrap(),
    );

    println!("OK — fixtures in {}", out.display());
}

fn hex_sha(text: &str) -> String {
    hex::encode(Sha256::digest(text.as_bytes()))
}

fn repo_surfaces_dir() -> PathBuf {
    // examples run with CARGO_MANIFEST_DIR = crates/doldskrift-font
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../..")
        .join("site/public/surfaces")
        .canonicalize()
        .unwrap_or_else(|_| manifest.join("../../site/public/surfaces"))
}
