# ADR-0007: DSK/3 Neural — Learned Machine Grammar (Phase V centerpiece)

## Status

Accepted for foundation scaffolding (types + docs + Mock/Deterministic path). Trained readers and full MGE/5 / DVE/2 are **not** shipping.

## Context

ADR-0006 established **Protected Visual Objects** and the **Gate** as a Phase V product surface: glyphs encode AEAD ciphertext; semantic recovery requires capability **K**. That work remains valid.

Separately, the research question for Phase V has shifted toward a **learned visual representation**:

> Can Doldskrift evolve from a fixed symbol alphabet into a **Latent Structure Grammar (LSG)** whose visual forms are optimized for machine perception, reconstructed by a compatible trained reader (DSK-R), while remaining honest that this is representation — not encryption?

Without reconciliation, docs would treat Phase V as “Gate only” or invent a conflicting DSK/3. We need one Phase V umbrella with **sibling profiles**.

## Decision

### 1. Phase V has three coexisting profiles

| Profile | Wire / product name | What glyphs carry | Recovery |
|---------|---------------------|-------------------|----------|
| **Open** | DSK/1–2 | Symbols → plaintext (or session permutation) | Public codec |
| **Neural** | DSK/3 Neural | Latent structure (LSG) → machine-optimized visuals | Compatible trained reader (DSK-R); Mock/Deterministic for CI |
| **Protected** | DSK/3 Protected | AEAD ciphertext of semantics | Gate + capability **K** |

**Neural ≠ encryption.** A learned reader may make casual human reading harder and may refuse incompatible grammars, but that is **not** confidentiality. Confidentiality remains Protected/AEAD (+ Gate). Neural and Protected may combine later (e.g. Neural visual of ciphertext) but solve different problems.

### 2. Protocol naming

- **DSK/3** remains the major protocol family for Phase V extensions.
- Sub-profiles: `DSK/3-NEURAL`, `DSK/3-PROTECTED` (and Open stays on DSK/1–2).
- **LSG/1** versions grammar epochs as `LSG/1-E0001`, `LSG/1-E0002`, …
- Reader manifests declare supported epochs; incompatible decode fails cleanly (`Incompatible` / `UnknownGrammar`) — **never** fabricate convincing false plaintext.

### 3. Pipeline (Neural)

```text
DskValue / semantic
  → SemanticComposer → LatentGraph (LSG)
  → MGE/5 (neural-aware render; stub today)
  → visual medium
  → DVE/2 → ObservationGraph (perception only)
  → ReaderBackend → ReaderOutput / status
```

Perception (DVE/2) does **not** reconstruct semantics. Reconstruction is the reader’s job.

### 4. Implementation posture (foundation)

- Prefer **modules inside existing crates** over new crates.
- Ship **real compiling stubs** + a **DeterministicBaseline** / **MockReader** path that proves exact-match round-trips in CI without multi-GB weights.
- `GemmaReader` may exist behind `cfg`/feature but must not pull huge deps by default and must not claim trained inference until wired.
- Training experiments may use Python under `training/`; production codec / protocol / vision / runtime / CLI stay Rust-first. No second protocol implementation.

### 5. Relationship to ADR-0006

ADR-0006 stands for Protected + Gate. This ADR does not supersede it. ROADMAP Phase V centers the **Neural research question** while retaining Protected milestones as a sibling track.

## Consequences

### Positive

- Single source of truth for Neural architecture and honest security language.
- CI can validate pipeline shape without model downloads.
- Protected remains a first-class product path without blocking Neural research.

### Negative / deferred

- Full MGE/5 spatial grammar, DVE/2 camera pipeline, Gemma/LoRA training, checkpoint packaging, and adversarial OCR bake-offs are deferred (see ROADMAP).
- Risk of profile confusion in UX — mitigate with explicit `profile` / `Readable` / reader-status fields.

## References

- [ADR-0006](ADR-0006-gate-protected-visual-objects.md) — Gate + Protected
- [Neural overview](../neural/overview.md)
- [`ROADMAP.md`](../../ROADMAP.md)
- [`SECURITY.md`](../../SECURITY.md)
