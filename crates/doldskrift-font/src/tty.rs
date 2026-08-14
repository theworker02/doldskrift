//! Terminal (TTY) fallback renderer — Unicode braille / block art from glyph zones.
//!
//! Not a substitute for SVG/PNG vision. Useful for demos without a browser.

use crate::geometry::GlyphGeometry;
use crate::zones::{Zone, ZonePrimitive};

/// Render a sequence of glyphs as braille cells (2×4 dots per glyph).
pub fn render_braille_line(glyphs: &[GlyphGeometry]) -> String {
    let mut out = String::new();
    for g in glyphs {
        out.push(glyph_to_braille(g));
        out.push(' ');
    }
    out
}

/// Render glyphs as a denser block-art strip (█ ▓ ▒ ░ ·).
pub fn render_block_line(glyphs: &[GlyphGeometry]) -> String {
    let mut out = String::new();
    for g in glyphs {
        out.push(glyph_to_block(g));
    }
    out
}

/// Multi-line TTY specimen with header.
pub fn render_tty_specimen(label: &str, glyphs: &[GlyphGeometry]) -> String {
    let mut out = String::new();
    out.push_str(&format!("Doldskrift TTY · {label}\n"));
    out.push_str(&format!(
        "braille: {}\n",
        render_braille_line(glyphs).trim_end()
    ));
    out.push_str(&format!("blocks:  {}\n", render_block_line(glyphs)));
    out.push_str(&format!("cells:   {}\n", glyphs.len()));
    out
}

fn glyph_to_braille(g: &GlyphGeometry) -> char {
    // Braille pattern bits (U+2800):
    // 0 3
    // 1 4
    // 2 5
    // 6 7
    let z = &g.zones;
    let mut bits: u32 = 0;
    if zone_ink(z.n) {
        bits |= 1 << 0;
    }
    if zone_ink(z.w) {
        bits |= 1 << 1;
    }
    if zone_ink(z.s) {
        bits |= 1 << 2;
    }
    if zone_ink(z.e) {
        bits |= 1 << 3;
    }
    if zone_ink(z.c) {
        bits |= 1 << 4;
        bits |= 1 << 5;
    }
    // Integrity / orientation as lower dots for uniqueness
    if g.fingerprint.integrity & 1 != 0 {
        bits |= 1 << 6;
    }
    if g.fingerprint.integrity & 2 != 0 {
        bits |= 1 << 7;
    }
    char::from_u32(0x2800 + bits).unwrap_or('⣿')
}

fn glyph_to_block(g: &GlyphGeometry) -> char {
    let occupied = Zone::ALL
        .iter()
        .filter(|z| zone_ink(g.zones.get(**z)))
        .count();
    match occupied {
        0 => '·',
        1 => '░',
        2 => '▒',
        3 => '▓',
        _ => '█',
    }
}

fn zone_ink(p: ZonePrimitive) -> bool {
    !matches!(p, ZonePrimitive::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate_glyph, FONT_UNITS};
    use doldskrift::{FONT_VERSION, PUA_BASE};

    #[test]
    fn braille_deterministic_and_nonempty() {
        let g = generate_glyph(PUA_BASE + 65, FONT_VERSION, 1).unwrap();
        let _ = FONT_UNITS;
        let line = render_braille_line(&[g.clone(), g]);
        assert!(line.chars().any(|c| c >= '\u{2800}'));
        assert_eq!(
            line,
            render_braille_line(&[
                generate_glyph(PUA_BASE + 65, FONT_VERSION, 1).unwrap(),
                generate_glyph(PUA_BASE + 65, FONT_VERSION, 1).unwrap(),
            ])
        );
    }
}
