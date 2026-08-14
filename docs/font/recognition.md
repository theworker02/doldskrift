# Recognition

MGE/2 is designed so machines can recover symbol identity from **structure**, not from Latin OCR.

## Fingerprint match

Given an observed `DSKG-1:…` string or `GlyphFingerprint`:

```rust
classify_fingerprint(fp, catalog)?;
classify_glyph(&geometry, catalog)?;
```

Returns `RecognitionResult { symbol, confidence, … }`.

Integrity: prefer candidates where `verify_integrity(symbol_id, zones, I)` holds.

## Distance

`fingerprint.distance` weights zone Hamming, redundancy delta, integrity delta, and circular orientation delta. `glyph_distance` adds topology feature deltas.

## Vision (experimental)

```text
PNG / feature map
  → zone estimates
  → candidate fingerprint
  → classify against catalog
  → ScanReport
```

APIs: `scan_image`, `scan_feature_map`, `bench_vision`, `render_glyph_png_feature_map`.

CLI:

```bash
dold vision path/to/image.png
dold bench vision   # Phase II controlled benches
```

Failures surface as `VisionError`. Vision is **not** required for DSK/1 conformance.

## Limits

- Skew, blur, and exotic cameras will degrade recovery — GRE/1 helps mildly; true ECC is future work.
- Encoded **text** in the DOM remains the reliable channel; pixels are a research channel.
- Never claim vision defeats determined inspection of source bytes.
