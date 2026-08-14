# Discovery

Agents and browsers need a structured way to notice Doldskrift content.

## HTML meta

```html
<meta name="doldskrift" content="version=1;mode=encoded">
```

Parse `version=` and `mode=` semicolon-separated parameters. Empty content → not detected.

## Data attributes

```html
<div data-doldskrift="1" data-doldskrift-mode="session"></div>
```

Presence of `data-doldskrift` yields `detected: true` with optional mode.

## Container magic

Files / buffers starting with `DSK1` are containers (`DetectionSource::ContainerMagic` when probed).

## PUA heuristic

If a high fraction of characters lie in `U+E000`–`U+E0FF`, treat as likely encoded text. Empty strings MUST NOT match. Thresholds are implementation-defined; reference JS/Rust expose a configurable threshold.

## Structured result

Prefer:

```json
{
  "detected": true,
  "version": 1,
  "mode": "encoded",
  "source": "html_meta",
  "raw": "version=1;mode=encoded"
}
```

over a bare boolean — callers need mode to choose a decoder.

## APIs

- Rust: `detect_from_html_meta`, `detect_from_attrs`, `detect_pua_heuristic`
- JS: `detectDoldskrift`, `detectFromHtmlMeta`, `detectFromAttrs`, `detectPuaHeuristic`
