# Future work

Tracked research and engineering beyond stable DSK/1 + Phase II MGE/2.

## Protocol

- [ ] DSK/2 sketch: compression flag semantics, ECC payloads, chunked containers
- [ ] DSK/3 Protected framing + Gate (design: [ADR-0006](../adr/ADR-0006-gate-protected-visual-objects.md))
- [ ] Stabilize experimental MIME registrations if the format sees external use
- [ ] Formalize control-symbol semantic channel (`U+E100+`) or drop it

## Glyphs / vision

- [ ] Richer vision/OCR pipeline with published bench numbers
- [ ] Optional ECC for lossy capture
- [ ] Machine-optimized visual bit alphabet behind an experimental flag
- [ ] Stronger camera-skew invariance
- [ ] MGE/5 target path for Protected visuals (variants, compound groups, layout)

## Tooling

- [ ] Broader TTF/OTF distribution of Text/Mono/Micro
- [ ] `dold explain` / `pack` / `unpack` UX polish
- [x] `dold open` + Protected `inspect` fields (stub refuse paths; no AEAD)
- [ ] Language SDKs beyond Rust/JS (Python, Go) locked to vectors
- [ ] Finish Phase IV Lens WASM / pixel-true vision leftover

## Product / ethics

- [ ] Operator “semantic reveal” patterns in the specimen site
- [ ] Clearer in-UI security disclaimers for third-party apps (Open vs Protected)
- [ ] Accessibility audits on published demos

## Process

New normative behavior → DEP per [`deps/DEP-0001-process.md`](../../deps/DEP-0001-process.md).
