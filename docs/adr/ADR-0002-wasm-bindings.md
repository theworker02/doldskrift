# ADR-0002: WASM as primary foreign-language bridge

## Status

Accepted

## Context

Maintaining full JS codec duplicates creates drift.

## Decision

Expose `doldskrift-bindings` (WASM + C ABI). Browser/Node TypeScript wrappers call WASM.

## Consequences

- Larger initial WASM engineering cost
- Deterministic cross-language behavior
- Differential tests: Rust ↔ WASM
