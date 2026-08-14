# Governance

Doldskrift is an open research and engineering project. This document describes how decisions are made, how the protocol evolves, and how contributors participate.

## Principles

1. **Representation, not security theater.** Changes must not imply encryption, DRM, or confidentiality claims that the system does not provide. **Open** and **Neural** are representation; **Protected** is the encryption track (AEAD + Gate) and is not shipping real crypto yet.
2. **Determinism across languages.** Rust and JavaScript (and future implementations) MUST agree on golden vectors in `spec/vectors/`.
3. **Fail closed.** Unsupported versions/modes and malformed input produce typed errors; they must not panic or silently reinterpret data. Neural unknown grammar → `Incompatible` / `UnknownGrammar`.
4. **Accessibility is non-negotiable.** Features that worsen AT access must document mitigation paths (see [`ACCESSIBILITY.md`](ACCESSIBILITY.md)).
5. **Extensibility without breakage.** Experimental fields and research engines (MGE/4–5, DVE/2, Neural) MUST NOT break DSK/1 parsers when absent.
6. **Five-crate workspace.** Consolidation rules live in [ADR-0003](docs/adr/ADR-0003-crate-consolidation.md) — do not reintroduce shim crates.

## Roles

| Role | Responsibility |
|------|----------------|
| **Maintainers** | Merge PRs, cut releases, own SECURITY responses, accept/reject DEPs and ADRs |
| **Contributors** | Propose patches, DEPs, tests, docs; follow [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| **Implementers** | Build alternate language SDKs against the normative spec and vectors |

## Decision records

### DEPs (protocol / engine)

Substantial wire-format and engine changes go through a **Doldskrift Enhancement Proposal** in [`deps/`](deps/).

| ID | Title | Status |
|----|-------|--------|
| [DEP-0001](deps/DEP-0001-process.md) | DEP process | Accepted |
| [DEP-0002](deps/DEP-0002-machine-glyph-engine.md) | Machine Glyph Engine (MGE/2) | Accepted |
| [DEP-0003](deps/DEP-0003-glyph-redundancy.md) | Glyph Redundancy Encoding (GRE/1) | Accepted |

### ADRs (architecture / product)

| ID | Title | Status |
|----|-------|--------|
| [ADR-0006](docs/adr/ADR-0006-gate-protected-visual-objects.md) | Gate + Protected Visual Objects | Accepted (design; crypto not shipping) |
| [ADR-0007](docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md) | DSK/3 Neural / LSG | Accepted (foundation) |

See also [`docs/adr/README.md`](docs/adr/README.md).

## Spec authority

Normative protocol requirements live in:

- [`spec/DOLDSKRIFT-1.md`](spec/DOLDSKRIFT-1.md) — DSK/1 wire format, mapping, errors
- [`spec/encoding.md`](spec/encoding.md), [`spec/containers.md`](spec/containers.md), [`spec/glyphs.md`](spec/glyphs.md), [`spec/discovery.md`](spec/discovery.md)
- [`spec/protocol-constants.json`](spec/protocol-constants.json) — cross-language constants (see [`spec/README.md`](spec/README.md))
- [`spec/vectors/*.json`](spec/vectors/) — golden conformance inputs/outputs

When prose and vectors disagree, **vectors win** until the prose is corrected. When Rust and JS disagree on a vector, that is a **release blocker**.

## Versioning

| Layer | Scheme | Notes |
|-------|--------|-------|
| Protocol | `DSK/1`, `DSK/2` (experimental), `DSK/3` Neural + Protected | Major version byte in containers; fail closed on unknown |
| Profiles | Open · Neural · Protected | Product mental model; see README / site Product |
| Glyph engine | `MGE/2` (+ MGE/4 research, MGE/5 Neural stubs) | Independent of protocol major; `font_version` in headers |
| Fingerprints | `DSKG-1` | Structural fingerprint string format |
| Redundancy | `GRE/1` | Density-linked redundancy levels 1–3 |
| LSG / readers | Epoch-tagged (`LSG/1-E####`, DSK-R) | Neural compatibility matrices |
| Crates / npm | SemVer | Breaking API changes bump major; **not published** until maintainers cut a release |

Experimental MIME types (`application/vnd.doldskrift`, `text/doldskrift`) may change with notice in `CHANGELOG.md`.

## Releases

1. All workspace tests pass (`cargo test --workspace`, `npm test`, `npm run site:build`, `npm run links`).
2. `cargo fmt` / `cargo clippy -D warnings` clean.
3. `dold conformance` passes against `spec/vectors`.
4. `dold doctor` OK from a fresh install path.
5. `CHANGELOG.md` updated; `ROADMAP.md` honest about Neural / Protected maturity.
6. Prefer `cargo deny check` clean before tagging (CI currently advisory — see [`CONTRIBUTING.md`](CONTRIBUTING.md)).
7. Maintainers tag and publish; contributors do not publish crates/npm packages unilaterally.
8. Do not upload model weights to the default repository.

## Platform maturity (honest)

| Track | Status |
|-------|--------|
| Open | DSK/1 codec + demos — stable prototype |
| Neural foundation | Types, DeterministicBaseline/Mock CI path, MGE/5 SVG carrier — **not** trained Gemma |
| Protected | Metadata + refuse paths — **not** real AEAD |
| Public GitHub / crates.io / npm | Prep only — badges may point at the intended org |

## Conduct and security

- Community norms: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)
- Vulnerability reporting and threat model: [`SECURITY.md`](SECURITY.md)
- Do not file “Open-mode Doldskrift can be decoded” as a vulnerability — that is intended.

## License

Dual-licensed Apache-2.0 OR MIT. Contributions are accepted under the same terms unless otherwise stated in the PR.
