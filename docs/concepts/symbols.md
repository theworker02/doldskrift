# Symbols

## Payload alphabet

DSK/1 encoded/session payloads use **256 symbols**:

| Property | Value |
|----------|-------|
| Range | `U+E000` … `U+E0FF` |
| Meaning | Offset `o` ↔ byte via Mapping |
| Identity | byte `b` ↔ `U+E000 + b` |

Helpers in Rust: `byte_to_symbol`, `symbol_to_byte`, `is_symbol`.  
In JS: same names from `@doldskrift/core`.

These codepoints live in the BMP Private Use Area. Without a Doldskrift font, OS fallback glyphs look empty or boxed; with the font, MGE/2 shapes appear.

## Control symbols (`U+E100+`)

Reserved for an experimental semantic / framing channel (not part of the byte alphabet):

| Constant | Offset | Codepoint |
|----------|--------|-----------|
| `DSK_START` | 0 | `U+E100` |
| `DSK_END` | 1 | `U+E101` |
| `DSK_TOKEN` | 2 | `U+E102` |
| `DSK_BREAK` | 3 | `U+E103` |
| `DSK_META` | 4 | `U+E104` |
| `DSK_ESCAPE` | 5 | `U+E105` |

`is_control_symbol` treats `U+E100` … `U+E10F` as the control window. Payload decoders MUST reject these when expecting alphabet symbols (`InvalidCodepoint`).

## Project mark

| Constant | Codepoint |
|----------|-----------|
| `DSK_PROJECT_MARK` | `U+E1F0` |

Valid MGE/2 glyph; official logo. Not a data-byte symbol.

## Mental model

```text
U+E000–E0FF   data alphabet (256)
U+E100–E10F   control window (experimental)
U+E1F0        project mark / logo
```

Everything else is ordinary Unicode (visual mode) or invalid for encoded payloads.
