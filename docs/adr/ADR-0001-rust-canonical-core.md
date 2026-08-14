# ADR-0001: Rust is the canonical protocol implementation

## Status

Accepted

## Context

Multiple languages tempted duplicate codecs (Rust + JS). Divergent behavior breaks conformance.

## Decision

All protocol-critical behavior originates in Rust. Other languages use WASM or C ABI bindings.

## Consequences

- One test corpus
- JS packages become thin wrappers over time
- Slightly more complex packaging for browsers
