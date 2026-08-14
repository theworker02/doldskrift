# Font installation

## Build artifacts

```bash
dold font build --out fonts/generated --seed 1
```

Typical outputs:

| Path | Role |
|------|------|
| `fonts/generated/Doldskrift-Regular.svg` | SVG font source |
| `fonts/generated/glyphs.json` | Catalog + fingerprints |
| `fonts/generated/doldskrift.css` | `@font-face` helper |
| `fonts/generated/build-manifest.json` | Build metadata |

Families targeted by MGE/2: **DoldskriftText**, **DoldskriftMono**, **DoldskriftMicro**.

Optional conversion tools: `tools/fontconvert/` (see its README) for TTF/OTF pipelines when needed.

## CSS

```css
@font-face {
  font-family: "DoldskriftText";
  src: url("./fonts/generated/Doldskrift-Regular.svg") format("svg");
  font-weight: normal;
  font-style: normal;
  font-display: block;
}

.doldskrift.agent-text {
  font-family: "DoldskriftText", ui-monospace, monospace;
}
```

For Mono/Micro, point additional `@font-face` rules at the corresponding generated files when present.

## System install (developers)

- **Windows:** copy TTF/OTF (if converted) into `C:\Windows\Fonts` or per-user fonts settings.
- **macOS:** Font Book → Add.
- **Linux:** `~/.local/share/fonts/` then `fc-cache -f`.

CLI specimens do not require system install; the specimen site loads webfonts.

## Density

When rendering specimens:

```bash
dold render --density high --debug
```

Micro family defaults to `low` density (GRE level 1) for small sizes.

## Project mark

To show the logo as text:

```html
<span class="doldskrift" aria-label="Doldskrift">&#xE1F0;</span>
```

Or embed `assets/brand/logo-mark.svg`. See [brand.md](../brand.md).

## Troubleshooting

| Symptom | Check |
|---------|-------|
| Boxes / empty glyphs | Font not applied; encoded PUA needs the face |
| Visual mode looks Latin | Wrong family or browser ignored `@font-face` |
| Fingerprints mismatch | Rebuild with the release `--seed` |
| AT reads garbage | Encoded mode without `aria-label` |
