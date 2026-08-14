# Protocol overview (DSK/1)

DOLDSKRIFT/1 defines how agents encode, frame, discover, and negotiate **representations** of UTF-8 text. It does not define a secure transport.

## Layers

```text
┌─────────────────────────────────────────┐
│ Discovery (HTML meta, data-*, magic)    │
├─────────────────────────────────────────┤
│ Negotiation (optional DSK? / DSK!)      │
├─────────────────────────────────────────┤
│ Modes: visual | encoded | session       │
├─────────────────────────────────────────┤
│ Mapping (identity or seeded permute)    │
├─────────────────────────────────────────┤
│ Streaming symbols  and/or  .dsk container│
└─────────────────────────────────────────┘
         optional MGE/2 display / vision
```

## Normative sources

| Document | Role |
|----------|------|
| [`spec/DOLDSKRIFT-1.md`](../../spec/DOLDSKRIFT-1.md) | Wire + mapping algorithms |
| [`spec/vectors/`](../../spec/vectors/) | Golden tests |
| This `docs/protocol/*` set | Explanatory guides |

## MIME (experimental)

- Binary container: `application/vnd.doldskrift`
- Textual representation: `text/doldskrift`

## Phase II relationship

MGE/2 / GRE/1 / vision attach **below and beside** DSK/1. A decoder that never renders glyphs remains DSK/1-conformant if it passes vector tests.

## Next

- [Framing](framing.md) · [Containers](containers.md) · [Discovery](discovery.md) · [Negotiation](negotiation.md) · [Streaming](streaming.md)
