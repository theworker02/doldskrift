# Font architecture (MGE/2)

```text
                    ┌──────────────┐
   codepoint ──────►│  zone_map    │◄── density (GRE/1 R)
   seed            │  N E S W C   │
   family          └──────┬───────┘
   font_version           │
                          ▼
                   integrity I (nibble)
                          │
                          ▼
              fingerprint DSKG-1:…-R#-I#-O#
                          │
                          ▼
                 primitive geometry
                     │         │
                     ▼         ▼
                   SVG/TTF   feature map
                     │         │
                     ▼         ▼
                   fonts/    vision/scan
```

## Modules (`doldskrift-font`)

| Module | Role |
|--------|------|
| `zones` | Zone + primitive enums, `zone_map_for_symbol` |
| `fingerprint` | `DSKG-1`, integrity |
| `density` | Density + `FontFamily` |
| `geometry` | Strokes / primitives |
| `distance` | `glyph_distance`, `MINIMUM_DISTANCE` |
| `metrics` | Human-readability vs machine-separability scores |
| `recognition` | Classify glyph/fingerprint against catalog |
| `vision` | Experimental PNG / feature-map scan |
| `svg` / `ttf` | Emit font sources |

## Families

`DoldskriftText`, `DoldskriftMono`, `DoldskriftMicro` — technical variants (advance width, default density), not fashion weights.

## Project mark

`generate_project_mark` builds `U+E1F0` through the same engine. Brand assets must stay consistent with that geometry.

## Specs

- [DEP-0002](../../deps/DEP-0002-machine-glyph-engine.md)
- [DEP-0003](../../deps/DEP-0003-glyph-redundancy.md)
- [generation](generation.md) · [recognition](recognition.md) · [specimen](specimen.md)
