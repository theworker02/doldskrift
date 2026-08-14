//! Generate golden test vectors into `spec/vectors/`.

use doldskrift::{checksum_bytes, encode, Session};
use std::fs;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../spec/vectors");
    fs::create_dir_all(&root).unwrap();

    let cases = [
        ("hello-world", "Hello world"),
        ("hello-doldskrift", "Hello from Doldskrift"),
        ("unicode", "café 日本語 🎉"),
        ("empty", ""),
        ("controls", "line1\nline2\ttab"),
    ];

    for (name, input) in cases {
        let encoded = encode(input).unwrap();
        let checksum = format!("{:08x}", checksum_bytes(input.as_bytes()));
        let enc_json = serde_json::json!({
            "version": 1,
            "mode": "encoded",
            "input": input,
            "encoded": encoded,
            "checksum": checksum,
        });
        fs::write(
            root.join(format!("{name}.json")),
            serde_json::to_string_pretty(&enc_json).unwrap(),
        )
        .unwrap();

        let session = Session::from_seed(b"vector-seed", "v1").unwrap();
        let sess_enc = session.encode(input).unwrap();
        let sess_json = serde_json::json!({
            "version": 1,
            "mode": "session",
            "seed": "vector-seed",
            "input": input,
            "encoded": sess_enc,
            "mapping_id": session.mapping_id().to_string(),
            "checksum": checksum,
        });
        fs::write(
            root.join(format!("{name}-session.json")),
            serde_json::to_string_pretty(&sess_json).unwrap(),
        )
        .unwrap();
    }

    let visual = serde_json::json!({
        "version": 1,
        "mode": "visual",
        "input": "DEPLOY READY",
        "encoded": "DEPLOY READY",
    });
    fs::write(
        root.join("visual-identity.json"),
        serde_json::to_string_pretty(&visual).unwrap(),
    )
    .unwrap();

    println!("Wrote golden vectors to {}", root.display());
}
