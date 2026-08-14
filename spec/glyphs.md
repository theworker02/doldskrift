# Glyph Grammar

Doldskrift glyphs are procedurally constructed from primitives:

- line
- arc
- junction
- dot
- ring
- branch
- terminal
- crossing
- orientation
- enclosure

## Determinism

```text
glyph = generate(codepoint, font_version, seed)
```

Same inputs MUST yield identical geometry on all platforms.

## Design tension

| Goal | Metric |
|------|--------|
| Low human familiarity | Heuristic Latin-resemblance score |
| High machine separability | Structural feature distance / unique bitfield |

Glyphs that score too similar to Latin letters/digits SHOULD be regenerated.
Near-duplicate glyphs SHOULD be regenerated until machine bitfields remain unique.

## Machine recognition features

Structural features (terminals, intersections, dots, orientation, enclosures) are embedded so a future vision pipeline can recover symbol IDs from pixels without ML in the base design.
