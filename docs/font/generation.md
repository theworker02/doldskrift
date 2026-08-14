# Glyph generation

## Entry points

```rust
generate_glyph(codepoint, font_version, seed)?;
generate_glyph_ex(codepoint, &GenerateOptions { density, family, .. })?;
generate_alphabet_ex(&opts)?;  // 256 symbols + distance search
generate_project_mark(seed)?;
generate_visual_ascii(font_version, seed)?; // U+0020..=U+007E remaps
```

CLI:

```bash
dold font build --out fonts/generated --seed 1
```

## Determinism

Same `(codepoint, font_version, seed, density, family)` → identical `GlyphGeometry` and fingerprint string.

Internal attempt loop may spin the seed when human-readability score stays too high (≥ 0.55), except the project mark which accepts immediately — still deterministic for a fixed starting seed.

## Alphabet construction

For each offset `0..256`:

1. Generate candidate glyph.
2. Measure `min glyph_distance` to existing glyphs.
3. If `< MINIMUM_DISTANCE` (1.5), spin seed up to 64 times keeping the best separation.
4. Soft-fail allowed after best effort; codepoint identity remains unique.

## GRE hooks

Density selects `R`:

| Density | R | Zone echoes |
|---------|---|-------------|
| low | 1 | base only |
| normal | 2 | force W≠Empty if needed + SE integrity dots |
| high | 3 | also force S≠Empty + high-redundancy markers |

## Outputs

- SVG font / per-glyph SVG
- `glyphs.json` catalog (codepoints, fingerprints, metrics)
- Build manifest for reproducibility

## Metrics during generation

- `human_readability_score` — lower is more “alien”
- `machine_separability` — higher is better for recognition

These guide regeneration heuristics; they are not security proofs.
