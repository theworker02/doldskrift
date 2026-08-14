# Roadmap

Honest status. No artificial dates. The platform is one product today — future work deepens layers, it does not bolt on “phases.”

## Product today (unified platform)

Doldskrift is a **machine-native visual information platform** with one architecture and three profiles:

| Profile | Role | Status |
|---------|------|--------|
| **Open** | Deterministic DSK/1–2 codec, MGE glyphs, demos, Lab CLI | Stable prototype |
| **Neural** | LSG + compatible reader (DSK/3 Neural) | Foundation stubs + DeterministicBaseline |
| **Protected** | AEAD ciphertext + Gate authorization | Refuse-path stubs only — **no AEAD yet** |

**Neural ≠ encryption.** Confidentiality is Protected (when AEAD ships). ADRs remain historical decision records; product docs speak in the present tense about this architecture.

### Shipping surface

- Five-crate workspace: `doldskrift`, `cli`, `vision`, `font`, `bindings`
- DSK/1 codec + containers; DSK/2 semantic modules (experimental)
- MGE/2 glyphs + MGE/4 research; MGE/5 Neural SVG carrier stubs
- DVE/1 `VisionEngine` + SVG geometry observation + `dold scan --diff`
- Lab CLI: `guide` · `demo` · `convert` · `config` · `doctor` · `pipeline` · `self-test` · unexpected surfaces
- Lens / Black Page / Radio / Surfaces as **GitHub Pages** (`site/`)
- JS: `@doldskrift/core` (+ thin `@doldskrift/web` / `@doldskrift/react` adapters; generated WASM)
- Governance shell: LICENSE, CHANGELOG, CONTRIBUTING, CoC, SECURITY, SUPPORT, TRADEMARKS, GOVERNANCE, `deny.toml`

## Next (productization)

- Compound glyph groups / Mesh profile (Open MGE path)
- Hardened release: SBOM attestations, **cargo-deny enforced** (today: warn / continue-on-error in CI)
- Versioned docs portal + rustdoc publish (`cargo doc -p doldskrift --no-deps`)
- Demo hub polish + install page kept in sync with CLI
- Prefer WASM codec on the static site over the local JS subset (incremental)

## Neural track (deepen LSG / readers)

Docs: [Neural overview](docs/neural/overview.md), [ADR-0007](docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md).

1. ~~Docs + ADR reconciliation~~ foundation
2. ~~Core types: LatentGraph, ObservationGraph, ReaderBackend, epochs~~ foundation
3. ~~SemanticComposer + DeterministicBaseline exact-match CI path~~ foundation
4. ~~MGE/5 deterministic SVG carrier from plan~~ minimal visual
5. DVE/2 camera / normalize / segment / topology beyond stubs
6. Real adapter training (Gemma/LoRA or successor) under `training/` — no multi-GB weights in default CI
7. Evaluation harness with published adversarial OCR numbers (metrics TBD until measured)
8. Reader packaging (`dold reader install|verify`) without silent auto-download of huge checkpoints

## Protected track (Gate + AEAD)

Docs: [Protected mode](docs/product/protected-mode.md), [Gate stub](docs/engineering/gate.md), [ADR-0006](docs/adr/ADR-0006-gate-protected-visual-objects.md).

1. ~~Protected container metadata + `inspect` fields (`Readable: No`) without real crypto~~
2. ~~CLI refuse path: `decode` on Protected → clear error; stub `open` → `AccessDenied`~~
3. Real AEAD + capability checks behind Gate (**feature-gated**, e.g. `protected-crypto` — not started)
4. Session transform of ciphertext bytes + Protected visual path
5. Agent policy surface (scoped ops) and runtime key backends

Honest bounds: no “AI-only / human-impossible” claim; authorized owners can still extract via instrumentation; Neural difficulty ≠ confidentiality.

## Research

- Full Forge evolution across generations (Forge 3 neural-aware objectives)
- Hierarchical graph recognition at scale
- GPU (wgpu) experimental track
- Human-resistance study framework (consent-based)
- Grammar epoch evolution and reader compatibility matrices

## Deferred (honest — not “0.5.0 done” until these land or stay deferred)

- Trained Gemma / multi-GB readers and checkpoints in-repo
- Full camera DVE/2
- Real AEAD + Gate crypto — do **not** claim encryption without this
- Public GitHub org publish / crates.io / npm (prep only; badges may already point ahead)
- Fake benchmark / OCR bake-off numbers

## Later

- Live visual state / embedded optical channels (builds on Gate runtime)
- Neural + Protected composition (visual of ciphertext under LSG)
- Standards collaboration readiness
