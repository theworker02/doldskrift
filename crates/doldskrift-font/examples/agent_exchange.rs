//! Minimal agent exchange using postcard SVG carriers (Open mode).
//!
//! ```bash
//! cargo run -p doldskrift-font --example agent_exchange
//! ```
//!
//! Day-to-day: `echo "hi" | dold postcard -o msg.svg && dold postcard --read msg.svg`

use doldskrift::{decode, encode};
use doldskrift_font::{extract_carrier_from_svg, render_postcard_svg};
use sha2::{Digest, Sha256};

fn main() {
    let plaintext = "Deploy worker seven.";
    // Agent α
    let encoded = encode(plaintext).expect("encode");
    let digest = hex::encode(Sha256::digest(plaintext.as_bytes()));
    let svg = render_postcard_svg(&encoded, 1, &digest);

    // Agent β (carrier metadata — not camera vision)
    let carrier = extract_carrier_from_svg(&svg).expect("carrier");
    assert_eq!(carrier.kind, "machine-postcard");
    let recovered = decode(&carrier.encoded).expect("decode");
    assert_eq!(recovered, plaintext);
    assert_eq!(carrier.sha256.as_deref(), Some(digest.as_str()));

    println!("OK — agent exchange round-trip");
    println!("payload: {recovered}");
    println!("sha256:  {}… (integrity ≠ signature)", &digest[..12]);
}
