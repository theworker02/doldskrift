# Architecture

Doldskrift is a **machine-native visual information platform**. One stack, three profiles — not a timeline of bolted-on phases.

## Product definition

> A machine-native visual information system for representing, rendering, transmitting, and reconstructing structured information through glyphs designed primarily for machine interpretation.

Swedish *dold* + *skrift* → concealed script. Hidden ≠ encrypted in Open or Neural modes.

## Four layers

```text
DOLDSKRIFT
┌──────────────────────────────────────┐
│ Semantic Layer                       │  structured machine information
├──────────────────────────────────────┤
│ Protocol Layer                       │  frames, streams, schemas, sessions
├──────────────────────────────────────┤
│ Visual Layer                         │  glyphs, alphabets, rendering
├──────────────────────────────────────┤
│ Recognition Layer                    │  detection, vision, reconstruction
└──────────────────────────────────────┘
```

Unified Open story:

```text
DATA → DSK → MGE → VISUAL MEDIUM → DVE → DSK → DATA
```

Protected story (target — AEAD not shipping):

```text
DATA → serialize → AEAD → DSK/3 → MGE → VISUAL → DVE → ciphertext → Gate(K) → DATA
```

Neural story (foundation stubs):

```text
semantic → LSG (LatentGraph) → MGE/5 plan → visual → DVE/2 → ObservationGraph → DSK-R
```

## Platform names

| Name | Meaning |
|------|---------|
| **DSK** | Protocol family (DSK/1, experimental DSK/2, DSK/3 Neural + Protected) |
| **MGE** | Machine Glyph Engine (glyphs, Forge, unexpected surfaces) |
| **DVE** | Doldskrift Vision Engine |
| **LSG** | Latent Structure Grammar (Neural intermediate) |
| **DSK-R** | Neural readers (DeterministicBaseline / Mock today) |
| **Gate** | Authorized runtime for Protected objects |
| **Forge** | Alphabet optimization |
| **Lens** | Flagship visual demo (Open) |
| **Lab** | CLI and developer environment |

## Profiles (present tense)

| Profile | What it is | Shipping? |
|---------|------------|-----------|
| **Open** | Deterministic codec + glyphs | Yes (prototype) |
| **Neural** | Learned LSG + compatible reader | Foundation stubs |
| **Protected** | AEAD + Gate | Refuse stubs only |

## Crate layout (≤5)

```text
crates/
├── doldskrift/           # semantic + protocol (+ neural/, protected)
├── doldskrift-cli/       # dold Lab CLI
├── doldskrift-vision/    # DVE
├── doldskrift-font/      # MGE + Forge + surfaces
└── doldskrift-bindings/  # WASM + C ABI
```

**Rule:** Rust owns the truth. JS/WASM adapters speak to Rust constants ([`spec/protocol-constants.json`](spec/protocol-constants.json)).

See [`docs/adr/`](docs/adr/) for historical decisions and [`docs/engineering/dependencies.md`](docs/engineering/dependencies.md) for dependency policy.

## Security boundary

**Open / Neural:** Checksums detect accidental corruption. Visual illegibility and learned grammars are **not** encryption.

**Protected:** AEAD + capability-held Gate *will be* the confidentiality boundary; DVE still only recovers ciphertext. Honest limits: [SECURITY.md](SECURITY.md).
