<p align="center">
  <img src="../../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# Neural overview

**Status:** Foundation scaffolding. Mock / DeterministicBaseline paths compile and test. Trained readers are **not** shipping.

Neural is a **first-class platform profile** — a learned Latent Structure Grammar (LSG) with a compatible reader — not an appendix bolted onto Open mode.

## Research question

Can Doldskrift evolve from a fixed symbol alphabet into a **Latent Structure Grammar (LSG)** whose visual forms are optimized for machine perception and reconstructed by a **compatible trained reader (DSK-R)**?

## Honest framing

| Claim | OK? |
|-------|-----|
| Learned visuals may require a matching reader / grammar epoch | Yes |
| Wrong reader fails closed (`Incompatible` / `UnknownGrammar`) | Yes |
| Neural is encryption / unbreakable / AI-only secrecy | **No** |
| Humans cannot reverse-engineer Neural artifacts | **No** |

**Neural ≠ encryption.** Confidentiality is the **Protected** profile (AEAD + Gate). Neural and Protected solve different problems; they may compose later.

## Profiles

| Profile | Protocol | Recovery |
|---------|----------|----------|
| Open | DSK/1–2 | Public codec |
| Neural | DSK/3 Neural | Compatible reader |
| Protected | DSK/3 Protected | Gate + capability K |

## Pipeline

```text
semantic → SemanticComposer → LatentGraph (LSG/1-E####)
  → MGE/5 plan/render → visual medium
  → DVE/2 → ObservationGraph → ReaderBackend → status / value
```

See [architecture](architecture.md), [LSG](lsg.md), [reader](reader.md), [security](security.md).

## What runs today

- `compose_deterministic` → carrier `ObservationGraph` → `DeterministicBaseline` / `MockReader` exact round-trip (SHA-256 match in tests)
- CLI: `dold neural encode|render|reconstruct|inspect`, `dold dataset generate`, reader stubs
- MGE/5 and DVE/2 are **honest stubs** with TODOs

## Deferred

Real Gemma/LoRA training, full MGE/5 geometry, full DVE/2 camera path, multi-GB checkpoints, published adversarial OCR numbers.
