//! Generate a real MGE/2 glyph strip for the README demonstration.

use doldskrift::{encode, FONT_VERSION};
use doldskrift_font::{generate_alphabet, Primitive};

fn main() {
    let text = "Deploy worker seven.";
    let enc = encode(text).unwrap();
    let catalog = generate_alphabet(FONT_VERSION, 1).unwrap();
    let n = enc.chars().count();
    let mut body = String::new();
    body.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} 120" width="100%" role="img" aria-label="Doldskrift rendering of Deploy worker seven.">"##,
        n * 56
    ));
    body.push_str(r##"<rect width="100%" height="100%" fill="#EEF1F4"/>"##);
    for (i, ch) in enc.chars().enumerate() {
        let cp = ch as u32;
        let g = catalog.iter().find(|g| g.codepoint == cp).unwrap();
        body.push_str(&format!(
            r##"<g transform="translate({},8) scale(0.05)">"##,
            i * 56
        ));
        for p in &g.primitives {
            match p {
                Primitive::Line { from, to } => body.push_str(&format!(
                    r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#12161A" stroke-width="36" stroke-linecap="round"/>"##,
                    from.x,
                    1000 - from.y,
                    to.x,
                    1000 - to.y
                )),
                Primitive::Dot { at, radius } => body.push_str(&format!(
                    r##"<circle cx="{}" cy="{}" r="{radius}" fill="#12161A"/>"##,
                    at.x,
                    1000 - at.y
                )),
                Primitive::Ring { center, radius } => body.push_str(&format!(
                    r##"<circle cx="{}" cy="{}" r="{radius}" fill="none" stroke="#12161A" stroke-width="36"/>"##,
                    center.x,
                    1000 - center.y
                )),
                Primitive::Junction { at } => body.push_str(&format!(
                    r##"<circle cx="{}" cy="{}" r="14" fill="#0F6B5C"/>"##,
                    at.x,
                    1000 - at.y
                )),
                Primitive::Terminal { at, .. } => body.push_str(&format!(
                    r##"<circle cx="{}" cy="{}" r="20" fill="none" stroke="#12161A" stroke-width="28"/>"##,
                    at.x,
                    1000 - at.y
                )),
                Primitive::Branch { origin, a, b } => body.push_str(&format!(
                    r##"<path d="M{} {} L{} {} M{} {} L{} {}" stroke="#12161A" stroke-width="36" fill="none" stroke-linecap="round"/>"##,
                    origin.x,
                    1000 - origin.y,
                    a.x,
                    1000 - a.y,
                    origin.x,
                    1000 - origin.y,
                    b.x,
                    1000 - b.y
                )),
                Primitive::Crossing { at, arm, .. } => body.push_str(&format!(
                    r##"<path d="M{} {} L{} {} M{} {} L{} {}" stroke="#12161A" stroke-width="36" fill="none"/>"##,
                    at.x - arm,
                    1000 - at.y,
                    at.x + arm,
                    1000 - at.y,
                    at.x,
                    1000 - (at.y - arm),
                    at.x,
                    1000 - (at.y + arm)
                )),
                Primitive::Arc {
                    center, radius, ..
                } => body.push_str(&format!(
                    r##"<circle cx="{}" cy="{}" r="{radius}" fill="none" stroke="#12161A" stroke-width="36"/>"##,
                    center.x,
                    1000 - center.y
                )),
                Primitive::Enclosure {
                    origin,
                    width,
                    height,
                } => body.push_str(&format!(
                    r##"<rect x="{}" y="{}" width="{width}" height="{height}" fill="none" stroke="#12161A" stroke-width="36"/>"##,
                    origin.x,
                    1000 - origin.y - height
                )),
            }
        }
        body.push_str("</g>");
    }
    body.push_str("</svg>\n");
    std::fs::create_dir_all("assets/brand").ok();
    std::fs::write("assets/brand/readme-demo.svg", body).unwrap();
    println!("wrote assets/brand/readme-demo.svg ({n} glyphs for {text:?})");
}
