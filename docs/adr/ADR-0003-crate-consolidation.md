# ADR-0003: Five-crate workspace

## Status

Accepted

## Context

Crate sprawl (core/codec/document/protocol/…) slowed navigation and confused consumers.

## Decision

Target:

```text
doldskrift
doldskrift-cli
doldskrift-vision
doldskrift-font
doldskrift-bindings
```

Protocol lives in `doldskrift` modules. Vision and font stay separate for dependency isolation.

## Consequences

- Users depend on `doldskrift = "..."`
- Temporary thin shims may exist during migration
