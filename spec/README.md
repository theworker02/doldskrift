# Spec index

Normative and supporting materials for Doldskrift wire formats and engines.

| Document | Role |
|----------|------|
| [`DOLDSKRIFT-1.md`](DOLDSKRIFT-1.md) | **Normative** DSK/1 protocol (mapping, errors, containers overview) |
| [`encoding.md`](encoding.md) | Encoding details |
| [`containers.md`](containers.md) | `.dsk` binary layout |
| [`glyphs.md`](glyphs.md) | Glyph / PUA / MGE pointers |
| [`discovery.md`](discovery.md) | Handshake / discovery |
| [`protocol-constants.json`](protocol-constants.json) | **Single source** for cross-language constants |
| [`vectors/`](vectors/) | Golden conformance vectors (authority when prose disagrees) |

## Engines & research (pointers into code + docs)

| Label | Code / docs |
|-------|-------------|
| **DSK** | `crates/doldskrift` — codec, document, protocol |
| **MGE** | `crates/doldskrift-font` — glyph engine; DEP-0002 |
| **DVE** | `crates/doldskrift-vision` — recognition |
| **LSG / Neural** | `crates/doldskrift/src/neural/` — [ADR-0007](../docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md) |
| **Protected / Gate** | `protected` module + refuse stubs — [ADR-0006](../docs/adr/ADR-0006-gate-protected-visual-objects.md) |

## Syncing constants

1. Edit [`protocol-constants.json`](protocol-constants.json).
2. Mirror into `packages/doldskrift-js/src/protocol-constants.json`.
3. Align Rust `errors` / version constants and `packages/doldskrift-js/src/types.ts`.
4. Run `cargo test --workspace` and `npm test`.

`honesty.protectedAeadShipping` must remain `false` until real AEAD ships behind an explicit feature and tests.
