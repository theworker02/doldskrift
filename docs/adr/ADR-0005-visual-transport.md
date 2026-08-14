# ADR-0005: Visual transport model

## Status

Accepted

## Context

Doldskrift’s value proposition is a round-trip through a visual medium:

```text
DATA → DSK → MGE → VISUAL → DVE → DSK → DATA
```

Browser demos (Lens, examples) need a fast local story, while production recognition (cameras, screenshots, print) needs a real vision pipeline. Conflating the two leads to fake claims (“browser vision = DVE”) or under-explaining demos.

## Decision

1. Treat the **visual medium** as an explicit transport hop — lossy, physical, not a crypto channel.
2. Keep **pixel / camera DVE** in Rust (`doldskrift-vision`, `dold vision` / `dold bench vision`).
3. Allow **browser demos** to reconstruct via a **structural map** bound to rendered MGE glyphs after discarding plaintext, provided UX copy states that this is not full pixel DVE.
4. Publish numeric recognition / throughput only from measured CLI/Criterion runs — never invent site metrics.
5. ECC, beacons, and degradation curves remain research until benches say otherwise.

## Consequences

- Clear split: site codec + structural demos vs Lab vision crate
- Security docs stay aligned: illegibility ≠ secrecy
- WASM DVE can land later without rewriting the transport story
- Partners and papers can cite the same DATA→…→DATA diagram without overclaiming maturity
