# Neural architecture

## Layers

```text
┌─────────────────────────────────────────────┐
│ Semantic (DSK/2 Value / future DskValue)    │
├─────────────────────────────────────────────┤
│ SemanticComposer → LatentGraph (LSG)        │
├─────────────────────────────────────────────┤
│ MGE/5 (MGE5-NEURAL profile)                 │
├─────────────────────────────────────────────┤
│ Visual medium (screen / print / camera)     │
├─────────────────────────────────────────────┤
│ DVE/2 → ObservationGraph (perception only)  │
├─────────────────────────────────────────────┤
│ ReaderBackend (DSK-R) → ReaderOutput        │
└─────────────────────────────────────────────┘
```

## Crate placement

| Concern | Location |
|---------|----------|
| LSG, composer, readers, profiles, Gate capability types | `crates/doldskrift/src/neural/` |
| MGE/5 plan stubs | `crates/doldskrift-font/src/mge5.rs` |
| DVE/2 perceive stubs | `crates/doldskrift-vision/src/dve2.rs` |
| CLI | `crates/doldskrift-cli/src/neural_cli.rs` |
| Training experiments | `training/` (Python; not a second protocol) |

## Profiles

`ProfileKind::{Open, Neural, Protected}` — see [ADR-0007](../adr/ADR-0007-dsk3-neural-learned-machine-grammar.md) and [ADR-0006](../adr/ADR-0006-gate-protected-visual-objects.md).

## Foundation carrier path

Until camera DVE/2 exists, CI uses `ObservationGraph::from_latent_carrier(LatentGraph)` so reconstruction can be tested without pixels. Production perception must not depend on this field.
