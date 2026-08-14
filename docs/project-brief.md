# Doldskrift — Project Brief

Source material for presentations, partnerships, and research conversations.

## Problem

Software agents still exchange meaning mostly through human writing systems and opaque binary blobs (QR-like codes). There is little room for a **font-like, structured, visually composable machine writing system** that survives pixels, screenshots, print, and cameras while remaining deterministically reconstructable.

## Technology

Doldskrift is a machine-native visual information platform:

```text
DATA → DSK (protocol) → MGE (glyphs) → VISUAL MEDIUM → DVE (vision) → DSK → DATA
```

Four layers: Semantic, Protocol, Visual, Recognition.

## Architecture

- **Rust** is the canonical implementation
- WASM / C ABI are bindings, not alternate protocols
- Vision and font tooling are isolated crates for dependency hygiene

## Demonstration

- **Lens** — human view vs machine decode with evidence
- **Black Page** — dense machine-illegible page → vision-only reconstruction → hash match
- CLI round-trips and conformance vectors

## Technical moat (earned difficulty)

- Machine glyph grammar and fingerprints
- Deterministic rendering + recognition
- Visual redundancy / ECC research
- Semantic framing (DSK/2)
- Cross-medium reconstruction (file ↔ pixels ↔ file)

## Open-source model

Apache-2.0 OR MIT core. No fake security claims. Open and Neural visual systems ≠ encryption; Protected mode (DSK/3 Protected / Gate) is the named encrypted surface when it ships.

## Possible applications (non-customer claims)

Machine-readable displays, camera-readable device coordination, agent debugging surfaces, visual telemetry, HCI research, air-gap *demonstrations*, printed machine media.

## Current status

See [`ROADMAP.md`](../ROADMAP.md). Prototype / experimental for advanced vision; DSK/1 codec is the most mature surface.

## Research questions

- How far can human-familiarity be minimized without harming machine separability?
- What ECC / fountain strategies win for camera channels?
- Can graph-based recognition scale with large alphabets?
- Can a Latent Structure Grammar + compatible reader outperform fixed alphabets without claiming encryption? ([Neural](neural/overview.md))

## Phase V profiles

| Profile | Claim |
|---------|-------|
| Open | Encoding — not encryption |
| Neural | Learned representation — not encryption |
| Protected | AEAD + Gate — encryption when shipping |
