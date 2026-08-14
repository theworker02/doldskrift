# Rust API reference

Crate `doldskrift` re-exports the stable surface.

## Codec

- `encode(text: &str) -> Result<String>`
- `decode(encoded: &str) -> Result<String>`
- `Encoder::new(Mode)` / `Decoder::new()`
- `StreamingEncoder` / `StreamingDecoder`
- `Session::builder().seed(...).build()?`

## Documents

- `DskDocument::encode_text` / `encode_session` / `pack_binary`
- `DskDocument::parse` / `open` / `write` / `to_bytes`
- `decode_text`, `inspect`, `verify`

## Font (feature `font`)

- `generate_glyph`, `generate_alphabet`, `generate_project_mark`
- `glyph_distance`, `GlyphFingerprint`, `classify_glyph`
- `Density`, `FontFamily`, `build_font_source`

See rustdoc: `cargo doc -p doldskrift --open`.
