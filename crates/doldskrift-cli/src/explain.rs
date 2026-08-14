//! `dold explain` — protocol education without dumping payload by default.

use doldskrift::DskDocument;
use doldskrift::{FONT_VERSION, PUA_BASE};
use doldskrift_font::{generate_alphabet, generate_glyph};

pub fn explain_document(
    doc: &DskDocument,
    reveal_symbols: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let report = doc.inspect();
    let mut out = report.display();
    out.push_str("\n\nExplain\n───────\n");
    out.push_str(&format!(
        "This container uses protocol {} with glyph engine MGE/{}.\n",
        report.protocol, report.glyph_engine
    ));
    out.push_str(&format!(
        "Mode `{}` selects how payload bytes relate to semantic text.\n",
        report.mode
    ));
    out.push_str("Checksum covers header JSON ∥ payload (CRC32C) for corruption detection only.\n");
    out.push_str("Decoded semantic text is withheld unless `--reveal` is passed.\n");

    if reveal_symbols && !doc.is_binary_payload() {
        if let Ok(text) = std::str::from_utf8(&doc.payload) {
            let catalog = generate_alphabet(FONT_VERSION, 1)?;
            out.push_str("\nSymbol sample (first 8 payload chars)\n");
            for (i, ch) in text.chars().take(8).enumerate() {
                let cp = ch as u32;
                if let Some(g) = catalog.iter().find(|g| g.codepoint == cp) {
                    out.push_str(&format!(
                        "\nSymbol #{i}\nFingerprint:\n  {}\nEncoding:\n  Payload symbol U+{cp:04X} (byte 0x{:02X})\nRedundancy:\n  Level {}\nIntegrity:\n  {}\n",
                        g.fingerprint(),
                        cp.saturating_sub(PUA_BASE),
                        g.features.redundancy,
                        if g.features.integrity == g.fingerprint.integrity {
                            "Valid"
                        } else {
                            "Mismatch"
                        }
                    ));
                } else if let Ok(g) = generate_glyph(cp, FONT_VERSION, 1) {
                    out.push_str(&format!(
                        "\nSymbol #{i}\nFingerprint:\n  {}\n",
                        g.fingerprint()
                    ));
                }
            }
        }
    }
    Ok(out)
}
