# Doldskrift: Machine-Native Visual Information

**Status:** Living outline · Version aligned with repository 0.4.x  
**License:** Apache-2.0 OR MIT (implementation)  
**Note:** Evaluation numbers are deliberately omitted until filled from reproducible `dold bench` / Criterion runs.

---

## Abstract

Software agents exchange meaning largely through human writing systems or opaque binary markers. Doldskrift proposes a **machine-native visual information platform**: a protocol (DSK), a structured glyph engine (MGE), a visual medium, and a vision engine (DVE) that together support deterministic round-trips of the form:

```text
DATA → DSK → MGE → VISUAL → DVE → DSK → DATA
```

This whitepaper outlines motivation, architecture, writing-system design, recognition approach, security non-goals, and an evaluation plan. It does **not** claim measured throughput or recognition rates here; those belong in a future Evaluation section populated from named, reproducible benches.

---

## 1. Introduction

### 1.1 Motivation

Agents increasingly need channels that:

1. Survive screenshots, print, and cameras
2. Remain deterministically reconstructable
3. Can be visually composed (font-like) rather than only rigid modules
4. Do not pretend that illegibility equals secrecy

### 1.2 Contributions (claimed scope)

- Layered architecture separating protocol, glyphs, medium, and recognition
- MGE/2 zone-structured glyphs with fingerprints (DSKG-1) and GRE redundancy
- Canonical Rust implementation with Lab CLI and experimental vision crate
- Honest product framing and open evaluation methodology

### 1.3 Non-contributions

- Encryption, authentication, or DRM
- A finished commodity scanner ecosystem
- Fabricated performance figures

---

## 2. Background and related work

- 2D barcodes and QR — mature public scanning; different design goals ([positioning](../docs/product/positioning.md))
- Steganography and “security fonts” — often conflate obscurity with crypto; Doldskrift refuses that framing
- Machine typography / symbolic HCI — related research threads; see [`docs/research/`](../docs/research/)
- Optical camera communication — adjacent; DVE is early-stage

---

## 3. Design goals

| Goal | Intent |
|------|--------|
| Determinism | Same seed/mode → same symbols and reconstructable decode |
| Machine-first glyphs | Geometry optimized for separability, not Latin familiarity |
| Medium survival | Graceful degradation research (blur, noise, print) |
| Protocol clarity | Containers, sessions, checksums; experimental semantics in DSK/2 |
| Honest security | Explicit non-goals in SECURITY.md |

### 3.1 Non-goals

Accessibility-as-sole-UI, drop-in QR replacement, confidentiality from visual form alone.

---

## 4. Architecture

Four conceptual layers:

1. **Semantic** — structured machine values (DSK/2 experimental)
2. **Protocol** — DSK framing, streams, containers, sessions
3. **Visual** — MGE glyphs, fonts, Forge research
4. **Recognition** — DVE detection and reconstruction

Platform names: DSK, MGE, DVE, Forge, Lens, Lab.

Implementation: Rust canonical core; WASM/C ABI bindings; vision and font crates isolated for dependency hygiene ([ADRs](../docs/adr/)).

```text
crates/
  doldskrift           # protocol core
  doldskrift-cli       # Lab (`dold`)
  doldskrift-vision    # DVE
  doldskrift-font      # MGE / Forge surfaces
  doldskrift-bindings  # WASM + C ABI
```

---

## 5. Protocol (DSK)

### 5.1 DSK/1

Byte↔PUA symbol mapping, modes (visual / encoded / session), containers (`.dsk`), checksums. Most mature surface.

### 5.2 DSK/2 (experimental)

Semantic value modules and typed streams — evolving; see [ADR-0004](../docs/adr/ADR-0004-dsk2-semantics.md).

### 5.3 Sessions and mappings

Seed-derived permutations for session opacity relative to the static table — still not encryption in Open mode.

### 5.4 DSK/3 Protected (proposed)

AEAD of semantic payloads; glyphs encode ciphertext; Gate + capability for semantic recovery. Design: [ADR-0006](../docs/adr/ADR-0006-gate-protected-visual-objects.md). Not shipping. Sibling to Neural under Phase V.

### 5.5 DSK/3 Neural (foundation)

Latent Structure Grammar (LSG), SemanticComposer, ObservationGraph, ReaderBackend. Design: [ADR-0007](../docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md). **Neural ≠ encryption.** Mock/DeterministicBaseline paths exist for CI; trained readers deferred.

---

## 6. Machine Glyph Engine (MGE)

### 6.1 MGE/2 anatomy

Zones N/E/S/W/C, primitive codes, orientation, GRE redundancy levels, integrity markers.

### 6.2 Fingerprints (DSKG-1)

Stable structural descriptors for recognition and debugging.

### 6.3 Project mark

`DSK_PROJECT_MARK` (`U+E1F0`) is a real glyph in the writing system, not a detached logo.

### 6.4 Forge

Research into alphabet / glyph optimization under multi-objective criteria — experimental.

### 6.5 Learned Machine Typography (outline — results TBD)

**Question:** Can glyph/layout generation be driven by latent structure (LSG) such that machine separability rises while Latin/digit familiarity falls, under a declared grammar epoch?

**Sketch:**

1. Compose semantics → `LatentGraph`
2. MGE/5 (`MGE5-NEURAL`) plans compounds, contextual variants, repetition suppression
3. Render (TODO: full geometry)
4. DVE/2 perceives → `ObservationGraph`
5. Compatible reader reconstructs or fails closed

**Non-claims:** Not encryption; no published recognition rates until measured; no “humans cannot decode” language.

**Pointers:** [`docs/neural/`](../docs/neural/), Forge 3 objective stubs in `doldskrift-font::mge5`.

---

## 7. Vision Engine (DVE)

### 7.1 Structural pipeline

Rust `doldskrift-vision` performs structural scan/recognition. Maturity: **experimental**.

### 7.2 Browser demos

Lens and site examples may recover via a **structural map** bound to rendered glyphs. That proves the writing-system story in-browser; it is **not** equivalent to shipping pixel DVE in WASM.

### 7.3 Transport model

See [ADR-0005](../docs/adr/ADR-0005-visual-transport.md).

---

## 8. Security model

Summary only — normative text: [`SECURITY.md`](../SECURITY.md).

- Visual illegibility ≠ secrecy
- CRC / checksums ≠ authenticity
- Threat model assumes determined analysts, DOM inspection, copy/paste

---

## 9. Implementation status

Qualitative maturity (from repository README):

| Component | Maturity |
|-----------|----------|
| DSK/1 codec + `.dsk` | Stable prototype |
| DSK/2 semantics | Experimental |
| DSK/3 Protected + Gate | Design only |
| MGE glyph engine | Candidate (evolving) |
| DVE vision | Experimental |
| Forge | Experimental |
| WASM bindings | Beta / in progress |
| CLI | Beta |
| Lens / Black Page | Shipping in `site/` (Open mode) |

---

## 10. Evaluation

> **To be filled from reproducible `dold bench` runs.**  
> Do not invent numbers for this section.

### 10.1 Planned methodology

1. Record host OS, CPU, commit SHA, sample counts
2. Run `dold bench vision --samples N` and archive stdout
3. Run `cargo bench -p doldskrift` (Criterion) for encode/decode/stream/container/mapping/glyph targets when configured
4. Publish tables with environment footnotes in release notes or a dated appendix

### 10.2 Metrics of interest (empty until measured)

| Metric | Source | Value |
|--------|--------|-------|
| Encode throughput | Criterion / CLI | *TBD* |
| Decode throughput | Criterion / CLI | *TBD* |
| Vision recognition (synthetic) | `dold bench vision` | *TBD* |
| Degradation curves | Controlled Lab runs | *TBD* |

### 10.3 Demo evidence (non-benchmark)

Browser Lens / visual-roundtrip SHA-256 exact match under the structural-map path — correctness demo, not a camera benchmark.

---

## 11. Applications

See [`docs/product/use-cases.md`](../docs/product/use-cases.md). Emphasize research, agent tooling, and optical *demonstrations* without overclaiming production readiness.

---

## 12. Limitations

- DVE and Forge are early
- Browser ≠ full pixel recognition
- No universal scanner ecosystem
- Human accessibility requires a parallel semantic channel

---

## 13. Future work

- Hierarchical / graph recognition (DVE/1+)
- ECC and fountain strategies under camera noise
- MGE contextual variants; Forge search loops
- WASM DVE bindings when stable enough
- Live visual state / runtime (Phase V direction)
- Gate + Protected Visual Objects (DSK/3) — see [ADR-0006](../docs/adr/ADR-0006-gate-protected-visual-objects.md)
- Standards collaboration readiness

See [`ROADMAP.md`](../ROADMAP.md) and [`docs/research/future-work.md`](../docs/research/future-work.md).

---

## 14. Conclusion

Doldskrift frames a coherent stack for machine-native visual information. Progress should be judged by architecture clarity, conformance tests, and **measured** benches — not marketing metrics.

---

## References (internal)

- [`spec/DOLDSKRIFT-1.md`](../spec/DOLDSKRIFT-1.md)
- [`ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`docs/adr/`](../docs/adr/)
- [`docs/product/`](../docs/product/)
- [`SECURITY.md`](../SECURITY.md)
