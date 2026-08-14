# Font conversion

`dold font build` emits SVG font + `glyphs.json` into `fonts/generated`.

To produce OTF/TTF/WOFF2 with [fonttools](https://github.com/fonttools/fonttools):

```bash
pip install fonttools brotli
# Convert using your preferred SVG/UFO → OpenType pipeline.
# Keep fonts/source as the regenerable source of truth.
```

The geometry is fully determined by `(font_version, seed)` in `build-manifest.json`.
