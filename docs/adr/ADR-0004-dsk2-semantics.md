# ADR-0004: DSK/2 semantic layering

## Status

Accepted (experimental surface)

## Context

DSK/1 successfully maps bytes to PUA symbols and containers, but agents increasingly need typed semantic values (events, schemas, token streams) without forking a second incompatible protocol family. Early modules already exist under `doldskrift` (`semantic`, `schema`, textform helpers). Shipping them as “DSK/1” would freeze an unfinished design; ignoring them would encourage ad-hoc JSON-over-glyphs without shared framing.

## Decision

1. Keep **DSK/1** as the stable prototype codec and `.dsk` container path.
2. Develop **DSK/2** as an **experimental** semantic layer: typed glyphs / value tokens, schema hooks, and richer payloads that still bottom out in deterministic symbol streams.
3. Mark all DSK/2 modules and wire shapes as evolvable until a future DEP / version bump freezes them.
4. Document maturity honestly (Experimental) in README, site status, and product docs.

## Consequences

- Callers must feature-detect or pin versions; do not assume DSK/2 wire stability
- DSK/1 golden vectors remain the conformance baseline
- Product messaging separates “codec works today” from “semantic platform in progress”
- Reduces pressure to overload DSK/1 with breaking semantic meaning
