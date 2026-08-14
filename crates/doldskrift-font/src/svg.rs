//! SVG path serialization for glyphs.

use crate::geometry::{GlyphGeometry, Point, Primitive, FONT_UNITS};

/// Convert a glyph to a standalone SVG document string.
pub fn glyph_to_svg(glyph: &GlyphGeometry) -> String {
    let mut paths = String::new();
    for p in &glyph.primitives {
        paths.push_str(&primitive_to_svg(p));
        paths.push('\n');
    }
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {FONT_UNITS} {FONT_UNITS}" width="64" height="64" fill="none" stroke="#1a1a1a" stroke-width="36" stroke-linecap="round" stroke-linejoin="round">
{paths}</svg>"##
    )
}

/// Emit SVG elements for a primitive.
pub fn primitive_to_svg(p: &Primitive) -> String {
    match p {
        Primitive::Line { from, to } => {
            format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
                from.x,
                flip(from.y),
                to.x,
                flip(to.y)
            )
        }
        Primitive::Arc {
            center,
            radius,
            start_deg,
            sweep_deg,
        } => {
            let (x1, y1) = polar(center, *radius, *start_deg);
            let (x2, y2) = polar(center, *radius, *start_deg + *sweep_deg);
            let large = if sweep_deg.abs() > 180 { 1 } else { 0 };
            let sweep = if *sweep_deg >= 0 { 1 } else { 0 };
            format!(
                r#"<path d="M {x1} {} A {radius} {radius} 0 {large} {sweep} {x2} {}"/>"#,
                flip_f(y1),
                flip_f(y2)
            )
        }
        Primitive::Junction { at } => format!(
            r##"<circle cx="{}" cy="{}" r="12" fill="#1a1a1a" stroke="none"/>"##,
            at.x,
            flip(at.y)
        ),
        Primitive::Dot { at, radius } => format!(
            r##"<circle cx="{}" cy="{}" r="{radius}" fill="#1a1a1a" stroke="none"/>"##,
            at.x,
            flip(at.y)
        ),
        Primitive::Ring { center, radius } => format!(
            r#"<circle cx="{}" cy="{}" r="{radius}"/>"#,
            center.x,
            flip(center.y)
        ),
        Primitive::Branch { origin, a, b } => format!(
            r#"<path d="M {} {} L {} {} M {} {} L {} {}"/>"#,
            origin.x,
            flip(origin.y),
            a.x,
            flip(a.y),
            origin.x,
            flip(origin.y),
            b.x,
            flip(b.y)
        ),
        Primitive::Terminal {
            at,
            orientation_deg,
        } => {
            let (dx, dy) = approx_dir(*orientation_deg);
            format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke-width="48"/>"#,
                at.x - dx / 5,
                flip(at.y - dy / 5),
                at.x + dx / 5,
                flip(at.y + dy / 5)
            )
        }
        Primitive::Crossing {
            at,
            arm,
            rotation_deg,
        } => {
            let (dx, dy) = approx_dir(*rotation_deg);
            let (px, py) = approx_dir(*rotation_deg + 90);
            format!(
                r#"<path d="M {} {} L {} {} M {} {} L {} {}"/>"#,
                at.x - dx * arm / 100,
                flip(at.y - dy * arm / 100),
                at.x + dx * arm / 100,
                flip(at.y + dy * arm / 100),
                at.x - px * arm / 100,
                flip(at.y - py * arm / 100),
                at.x + px * arm / 100,
                flip(at.y + py * arm / 100)
            )
        }
        Primitive::Enclosure {
            origin,
            width,
            height,
        } => format!(
            r#"<rect x="{}" y="{}" width="{width}" height="{height}" rx="40"/>"#,
            origin.x,
            flip(origin.y + height)
        ),
    }
}

fn flip(y: i32) -> i32 {
    FONT_UNITS - y
}

fn flip_f(y: f64) -> f64 {
    f64::from(FONT_UNITS) - y
}

fn polar(center: &Point, radius: i32, deg: i32) -> (f64, f64) {
    let rad = (f64::from(deg)).to_radians();
    (
        f64::from(center.x) + f64::from(radius) * rad.cos(),
        f64::from(center.y) + f64::from(radius) * rad.sin(),
    )
}

fn approx_dir(deg: i32) -> (i32, i32) {
    let d = deg.rem_euclid(360);
    match d / 45 {
        0 => (100, 0),
        1 => (70, 70),
        2 => (0, 100),
        3 => (-70, 70),
        4 => (-100, 0),
        5 => (-70, -70),
        6 => (0, -100),
        _ => (70, -70),
    }
}
