# Generated Doldskrift Font Artifacts

These files are produced by `dold font build` from the procedural glyph grammar.

- `Doldskrift-Regular.svg` — SVG font source (checked into `fonts/source` after promotion)
- `glyphs.json` — full geometry catalog
- `doldskrift.css` — `@font-face` helper

To produce OTF/TTF/WOFF2, run the converter in `tools/fontconvert` (fonttools),
or use any SVG-font → OpenType pipeline. The geometry is fully determined by
`(font_version, seed)` recorded in `build-manifest.json`.
