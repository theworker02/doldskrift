<p align="center">
  <img src="../assets/brand/logo-mark.svg" alt="Doldskrift" width="72" />
</p>

# Doldskrift documentation

Machine-native typography and encoding for software agents — **one platform**, three profiles (**Open · Neural · Protected**).

**Open** and **Neural** are **not encryption**. **Protected** will be AEAD + Gate — see [Security](security.md), [Neural overview](neural/overview.md), [Protected mode](product/protected-mode.md), and [`SECURITY.md`](../SECURITY.md).

## Map (start here)

| Doc | Audience |
|-----|----------|
| [Getting started](getting-started.md) | First encode/decode in five minutes |
| [Product](product/README.md) | What the company product is today |
| [Architecture](../ARCHITECTURE.md) | Layers + profiles (present-tense platform) |
| [CLI guide](guides/cli.md) · [`dold guide`](reference/cli.md) | Lab workflows |
| [FAQ](faq.md) | Common questions |
| [Brand](brand.md) | Logo is `DSK_PROJECT_MARK` (`U+E1F0`) |

## Concepts

- [Machine typography](concepts/machine-typography.md)
- [Glyphs (MGE)](concepts/glyphs.md)
- [Symbols & controls](concepts/symbols.md)
- [Mappings](concepts/mappings.md)
- [Sessions](concepts/sessions.md)

## Guides

- [Rust](guides/rust.md)
- [JavaScript](guides/javascript.md) — `@doldskrift/core` (+ thin web/react adapters)
- [Browser](guides/browser.md)
- [React](guides/react.md)
- [CLI (`dold`)](guides/cli.md)
- [Agents](guides/agents.md)
- [Font installation](guides/font-installation.md)

## Protocol (DSK)

- [Overview](protocol/overview.md) (DSK/1 open codec)
- [Framing](protocol/framing.md)
- [Containers](protocol/containers.md)
- [Discovery](protocol/discovery.md)
- [Negotiation](protocol/negotiation.md)
- [Streaming](protocol/streaming.md)
- [Neural (DSK/3 Neural)](neural/overview.md) · [ADR-0007](adr/ADR-0007-dsk3-neural-learned-machine-grammar.md)
- [Protected / Gate](product/protected-mode.md) · [Gate API stub](engineering/gate.md) · [ADR-0006](adr/ADR-0006-gate-protected-visual-objects.md)

Normative text (open): [`spec/DOLDSKRIFT-1.md`](../spec/DOLDSKRIFT-1.md). Constants: [`spec/protocol-constants.json`](../spec/protocol-constants.json).

## Neural profile

- [Overview](neural/overview.md)
- [Architecture](neural/architecture.md)
- [LSG](neural/lsg.md)
- [Reader](neural/reader.md)
- [Training](neural/training.md)
- [Evaluation](neural/evaluation.md)
- [Epochs](neural/epochs.md)
- [Limitations](neural/limitations.md)
- [Security](neural/security.md)
- [Reproducibility](neural/reproducibility.md)
- Templates: [model card](templates/model-card-neural.md) · [dataset card](templates/dataset-card-neural.md)

## Font / MGE

- [Architecture](font/architecture.md)
- [Generation](font/generation.md)
- [Recognition](font/recognition.md)
- [Specimen](font/specimen.md)

## Reference

- [Rust API](reference/rust-api.md)
- [JavaScript API](reference/js-api.md)
- [CLI reference](reference/cli.md)
- [Errors](reference/errors.md)
- [File format](reference/file-format.md)

## Research

- [Machine-readable writing](research/machine-readable-writing.md)
- [Human readability](research/human-readability.md)
- [Visual channel](research/visual-channel.md)
- [Future work](research/future-work.md)

## Product

- [Product docs](product/README.md)
- [Positioning](product/positioning.md)
- [Protected mode](product/protected-mode.md)
- [Use cases](product/use-cases.md)
- [Project brief](project-brief.md)

## Engineering

- [Build](engineering/build.md)
- [Testing](engineering/testing.md)
- [Dependency policy](engineering/dependencies.md)
- [Consolidation notes](engineering/consolidation-notes.md)
- [Gate API stub](engineering/gate.md)
- [ADRs](adr/README.md) (historical decisions; product docs speak present tense)

## Project

- [Brand](brand.md)
- [Accessibility](accessibility.md) · root [`ACCESSIBILITY.md`](../ACCESSIBILITY.md)
- [Security](security.md) · root [`SECURITY.md`](../SECURITY.md)
- [Whitepaper outline](../research/doldskrift-whitepaper.md)
- [Governance](../GOVERNANCE.md) · [Contributing](../CONTRIBUTING.md) · [DEPs](../deps/) · [Roadmap](../ROADMAP.md)
