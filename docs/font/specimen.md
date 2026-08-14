# Specimens

Specimens show what humans see when Doldskrift fonts render visual or encoded text.

## In-repo locations

| Path | Contents |
|------|----------|
| `fonts/specimens/logo.svg` | Project mark |
| `fonts/specimens/logo.png` | Raster logo |
| `fonts/generated/` | Built font + `glyphs.json` |
| `site/` | Interactive specimen / protocol explorer |

## Generating samples

```bash
echo "Hello agents" | dold render --mode encoded -o sample.txt
echo "Hello agents" | dold render --mode visual
dold render --mode session --seed alpha --density high --debug
```

`--debug` (Phase II) emits fingerprint/zone annotations suitable for papers and bug reports.

## Reading a specimen page

1. Note the **mode** — visual still copies as Latin; encoded copies as PUA.
2. Confirm font application (otherwise boxes).
3. Check accessible name / transcript for human operators.
4. Logo specimens should identify `U+E1F0` / `DSK_PROJECT_MARK`.

## Catalog inspection

```bash
dold font inspect --catalog fonts/generated/glyphs.json
```

Use fingerprints to compare regenerations after engine changes.

## Good specimen hygiene

- Label mode, seed, density, font version on the figure
- State Open-mode specimens are not encryption; label Protected specimens when DSK/3 exists
- Provide semantic transcript in captions when the audience is human (Open demos)
