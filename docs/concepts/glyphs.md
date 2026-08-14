# Glyphs (MGE/2)

Phase II glyphs are produced by the **Machine Glyph Engine v2**. A glyph is not only a path outline — it is a structured object with zone anatomy, a fingerprint, redundancy, and an integrity marker.

## Generation inputs

```text
generate(codepoint, font_version, seed, density, family) → GlyphGeometry
```

Equal inputs MUST yield identical geometry and `DSKG-1` fingerprints on all platforms.

## Zone anatomy

```text
      N
   W  C  E
      S
```

Each zone holds a primitive code `0..=7` (Empty, Terminal, Dot, Arc, Branch, Ring, Junction, Cross). The alphabet derives zones from the symbol id (`codepoint - U+E000`); GRE/1 may force secondary non-empty zones at higher density.

## Fingerprint

```text
DSKG-1:N2-E0-S1-W3-C4-R1-I9-O3
```

| Field | Meaning |
|-------|---------|
| N E S W C | Zone codes |
| R | GRE/1 redundancy level |
| I | 4-bit integrity / parity |
| O | Orientation bucket 0..7 |

## Families

| Family | CSS / name | Role |
|--------|------------|------|
| Text | `DoldskriftText` | General UI / specimens |
| Mono | `DoldskriftMono` | Terminals, aligned columns |
| Micro | `DoldskriftMicro` | Small sizes; default density `low` |

## Density

`low` / `normal` / `high` trade stroke weight and GRE level (1 / 2 / 3). See [DEP-0003](../../deps/DEP-0003-glyph-redundancy.md).

## Distance

`glyph_distance` mixes fingerprint distance with topology counts. Alphabet builders aim for pairwise distance ≥ `MINIMUM_DISTANCE` (1.5).

## Project mark

`U+E1F0` (`DSK_PROJECT_MARK`) is a first-class glyph and the brand logo. Details: [brand.md](../brand.md).

## Pipeline sketch

```text
symbol id → zones (+ GRE) → integrity I → fingerprint
         → primitives / strokes → SVG or font source
         → optional feature map for vision
```

Deeper: [font/architecture.md](../font/architecture.md), [DEP-0002](../../deps/DEP-0002-machine-glyph-engine.md).
