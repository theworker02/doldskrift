# DEP-0002: Machine Glyph Engine Version 2 (MGE/2)

- **Status:** Accepted
- **Created:** 2026-08-12
- **Authors:** Doldskrift maintainers
- **Related:** [DEP-0003](DEP-0003-glyph-redundancy.md)

## Abstract

Specifies MGE/2: a deterministic, zone-structured glyph generator and recognition model for Doldskrift symbols, including fingerprints (`DSKG-1`), integrity markers, structural distance, and font families.

## Motivation

Phase I glyphs were procedural but weakly structured for machine recovery from pixels. Agents and tooling need:

1. Stable structural identity independent of pixel rendering quirks.
2. Enough redundancy to survive mild capture noise (see GRE/1).
3. Families optimized for text, monospace, and micro sizes — not fashion variants.
4. The project logo to be a first-class glyph (`DSK_PROJECT_MARK`).

## Specification

### Engine identity

| Constant | Value |
|----------|-------|
| Glyph engine major | `2` (`GLYPH_ENGINE_VERSION`) |
| Font grammar version | `2` (`FONT_VERSION` / `FONT_VERSION_MGE2`) |
| Fingerprint format | `DSKG-1` |

Generation is deterministic for equal `(codepoint, font_version, seed, density, family)`.

### Zones

Each glyph cell is partitioned into five zones:

```text
        N
     W  C  E
        S
```

| Zone | Meaning |
|------|---------|
| N | North |
| E | East |
| S | South |
| W | West |
| C | Center |

Each zone holds a `ZonePrimitive` coded `0..=7`:

| Code | Primitive |
|------|-----------|
| 0 | Empty |
| 1 | Terminal |
| 2 | Dot |
| 3 | Arc |
| 4 | Branch |
| 5 | Ring |
| 6 | Junction |
| 7 | Cross |

Zone maps for alphabet symbols are derived from the symbol id (`codepoint - U+E000`) via `zone_map_for_symbol`, then adjusted by GRE/1 (DEP-0003).

### Fingerprints

Canonical string form:

```text
DSKG-1:N#-E#-S#-W#-C#-R#-I#-O#
```

| Field | Meaning |
|-------|---------|
| N…C | Zone primitive codes |
| R | Redundancy level (GRE/1) |
| I | Integrity nibble `0..=15` |
| O | Orientation bucket `0..=7` |

Integrity marker (reference algorithm, matching Rust):

```text
packed = zones.pack()   // 3 bits × 5 zones → u16
x = symbol_id as u32 XOR packed as u32 XOR (packed as u32 << 7)
x ^= x >> 4
x ^= x >> 2
I = x & 0x0F
```

### Distance

`glyph_distance` combines fingerprint distance with topology deltas (primitive count, terminals, intersections, dots). Alphabet generation SHOULD keep pairwise distance ≥ `MINIMUM_DISTANCE` (1.5). Soft failure is allowed after best-effort search; symbol identity remains unique by codepoint/bitfield.

### Families and density

| Family | Name string | Default density | Advance (font units) |
|--------|-------------|-----------------|----------------------|
| Text | `DoldskriftText` | normal | 620 |
| Mono | `DoldskriftMono` | normal | 600 |
| Micro | `DoldskriftMicro` | low | 520 |

Density levels: `low`, `normal`, `high` (see DEP-0003 for redundancy mapping).

### Project mark

`DSK_PROJECT_MARK = U+E1F0`. The brand logo **is** this glyph. Generators MUST treat it as a valid MGE/2 glyph (zones, fingerprint, integrity). Specimens and `assets/brand/` MUST remain geometrically consistent with the engine output for the official seed.

### Recognition

Classification matches observed fingerprints / feature maps against the catalog (`classify_glyph`, `classify_fingerprint`). Vision scanning (`scan_image`, feature maps) is **experimental** and not required for DSK/1 text conformance.

## Rationale

Zones give a compact, human-inspectable structural ID while remaining machine-primary. Fingerprints travel in JSON catalogs (`fonts/generated/glyphs.json`) without shipping full vector paths. Separating protocol (DSK/1) from glyph engine (MGE/2) lets fonts evolve without breaking `.dsk` parsers.

## Compatibility

- DSK/1 containers store `font_version` in header JSON; absence defaults to the implementation’s current font version.
- Parsers that ignore glyphs remain conforming.
- Font version `1` requests MAY be satisfied by generating MGE/2 geometry (forward fill); versions `> FONT_VERSION` MUST error with `UnsupportedVersion`.

## Security and accessibility

- Glyphs obfuscate **display**, not bytes in the DOM for encoded mode.
- Visual mode MUST keep semantic Unicode underneath.
- Vision pipelines MUST NOT be marketed as defeating determined inspection of source text.
- AT: see [`ACCESSIBILITY.md`](../ACCESSIBILITY.md).

## Open questions / future work

- Lossy-capture ECC as a container flag (`ECC`) for DSK/2+.
- Machine-optimized bit alphabets under an experimental flag.
- Stronger orientation invariance under camera skew.
