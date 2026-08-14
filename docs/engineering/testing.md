# Testing handbook

## Default gate

```bash
cargo test --workspace
```

Run from the repository root. Prefer fixing failures in the canonical Rust crates before adjusting site demos.

## Conformance

```bash
dold conformance --vectors spec/vectors
```

Golden vectors under `spec/vectors` are the protocol truth for DSK/1 behavior.

## Vision / benches

```bash
dold bench vision --samples 32
```

Archive the output with host details when publishing numbers. Do **not** invent metrics for docs or the site.

```bash
cargo bench -p doldskrift
```

Use Criterion reports only when the bench target exists and the run completed on a named environment.

## Site / JS

The static GitHub Pages site includes a demo-aligned codec (`site/src/doldskrift.js`, `site/src/mge2.js`). Treat it as a presentation surface; protocol-critical behavior must match Rust. Differential Rust↔WASM tests are the long-term bridge ([ADR-0002](../adr/ADR-0002-wasm-bindings.md)).

Manual smoke:

1. `npm run site:preview` (or `npm run site` while editing)
2. Lens direct + vision modes — SHA-256 evidence
3. `site/examples/visual-roundtrip.html` — ten-step walkthrough

## Link check

```bash
node tools/check-links.mjs
```

CI runs the internal link checker; keep new docs and site pages linked from existing indexes when practical.

## Reporting failures

- Codec / CLI / font: bug template
- Glyph fingerprints / recognition: glyph template
- Vision misses: `.github/ISSUE_TEMPLATE/vision-failure.yml`
- Docs gaps: `.github/ISSUE_TEMPLATE/documentation.yml`

## Related

- [Build](build.md)
- [CLI reference](../reference/cli.md)
