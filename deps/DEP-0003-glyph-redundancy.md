# DEP-0003: Glyph Redundancy Encoding (GRE/1)

- **Status:** Accepted
- **Created:** 2026-08-12
- **Authors:** Doldskrift maintainers
- **Related:** [DEP-0002](DEP-0002-machine-glyph-engine.md)

## Abstract

Defines GRE/1 — a lightweight structural redundancy layer for MGE/2 glyphs so mild rendering or capture degradation does not immediately destroy symbol identity.

## Motivation

Machine reading of rendered glyphs (screenshots, downscaled UI, print-and-scan experiments) fails when a single distinctive mark is lost. Full ECC is deferred; GRE/1 adds **small, deterministic echo features** keyed by density without changing the DSK/1 byte alphabet.

## Specification

### Levels

Redundancy level `R` is carried in `DSKG-1` fingerprints and tied to density:

| Density | `R` | Intent |
|---------|-----|--------|
| `low` | 1 | Minimal markers; Micro / tiny sizes |
| `normal` | 2 | Secondary echo (default Text/Mono) |
| `high` | 3 | Maximum structural echo |

### Zone policy

After computing the base `zone_map_for_symbol(symbol_id, …)`:

1. If `R >= 2` and zone **W** is Empty → force **Dot**.
2. If `R >= 3` and zone **S** is Empty → force **Terminal**.

This preserves a non-empty secondary cue even when primary zone geometry is partially lost.

### Geometry policy (reference implementation)

Matching `doldskrift-font` geometry generation:

- `R >= 2`: place integrity-echo dots near the south-east of the cell (radius depends on density).
- `R >= 3`: add additional high-redundancy markers (extra ticks / echoes) derived from the integrity nibble.

Integrity nibble `I` remains the DEP-0002 parity function of `(symbol_id, zones)`. Verification: `verify_integrity(symbol_id, zones, I)`.

### Recognition use

Decoders MAY:

1. Match full fingerprint including `R` and `I`.
2. Fall back to zone Hamming distance if `R`/`I` are uncertain under noise.
3. Prefer candidates that satisfy `verify_integrity` when multiple are close in `glyph_distance`.

GRE/1 does **not** add Reed–Solomon or other block codes. Container flag `ECC` remains reserved for future protocol versions.

### CLI / render surface

Phase II tooling exposes density to render and bench paths:

```text
dold render … --density low|normal|high
dold render … --debug          # emit fingerprint / zone annotations
dold vision … / dold bench vision …
```

Exact flags follow the CLI reference; density MUST select GRE level as above.

## Rationale

Echoing identity into multiple zones is cheap, deterministic, and understandable in specimens. It improves separability metrics (`machine_separability`) without expanding the 256-symbol alphabet or breaking text codecs.

## Compatibility

- Textual DSK/1 encode/decode is unaffected.
- Older catalogs without `R`/`I` fields are incomplete for MGE/2 recognition; regenerate with `dold font build`.
- Lowering density reduces redundancy; Micro defaults to `low` intentionally.

## Security and accessibility

- Extra marks increase **machine** recoverability, not human secrecy.
- Higher density can worsen clutter for human viewers — acceptable for the research goal, but UIs MUST still expose semantic text to AT when content is for people.
- Do not claim GRE/1 provides tamper evidence; `I` is a parity nibble, not a MAC.

## Open questions / future work

- Quantify recovery rates under controlled downscale/blur benches (`bench_vision`).
- Optional true ECC payload in `.dsk` when `DocumentFlags::ECC` is defined for DSK/2.
- Whether session-mode visual specimens should pin density in the manifest `meta` map.
