# Contributing to Doldskrift

Thanks for helping build a machine-native visual information system.

**Name note:** *Doldskrift* is Swedish (*dold* + *skrift* → “concealed script”). Concealment describes visual opacity, not cryptography.

## Principles

1. **Correctness over cleverness** — the codec must be boringly reliable.
2. **No fake security** — never call obfuscation encryption. Open / Neural ≠ confidentiality ([`SECURITY.md`](SECURITY.md)).
3. **Machine-first** — strangeness of glyphs is intentional (MGE/2+).
4. **Deterministic** — equal inputs produce equal outputs across platforms and languages.
5. **Interoperable** — Rust owns the truth; other languages speak via WASM/FFI and golden vectors.
6. **Accessible by design** — applications need a path to expose semantic text ([`ACCESSIBILITY.md`](ACCESSIBILITY.md)).
7. **Extensible without breakage** — experimental engines must not break DSK/1 parsers.
8. **Document decisions** — substantial design changes need a DEP ([`deps/DEP-0001-process.md`](deps/DEP-0001-process.md)) or ADR under `docs/adr/`.
9. **Honest profiles** — Neural is a learned representation track; Protected is AEAD + Gate. Do not conflate them. Product docs speak present tense about one platform; ADRs may still say “Phase V” historically.

## Current focus areas

| Area | What to know |
|------|----------------|
| **Open (DSK/1–2)** | Shipping codec / containers / demos — [`spec/DOLDSKRIFT-1.md`](spec/DOLDSKRIFT-1.md) |
| **MGE / fonts** | Glyph engine + families — [`deps/DEP-0002-machine-glyph-engine.md`](deps/DEP-0002-machine-glyph-engine.md) |
| **Neural (DSK/3)** | LSG, DeterministicBaseline / Mock — [`docs/neural/overview.md`](docs/neural/overview.md), [ADR-0007](docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md) |
| **Protected** | Refuse-path stubs only until AEAD — [ADR-0006](docs/adr/ADR-0006-gate-protected-visual-objects.md) |
| **Surfaces / site** | Unexpected Open demos + GitHub Pages (`site/`) |
| **Docs** | User guides under `docs/`; normative wire rules under `spec/` |

## Development

### Commit author

Release and project commits in this repository use the local git identity **theworker02** / `theworker02@users.noreply.github.com` (never CursorAgent). Agents should rely on repo-local `git config` or pass matching `-c user.name` / `-c user.email` overrides.

One-liners (also see root `Makefile` / npm scripts):

```bash
# Full Rust quality bar
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p doldskrift-cli -- conformance
cargo run -p doldskrift-cli -- doctor

# JS + site
npm install
npm test
npm run site:build
npm run links

# Or: make check / make test / make site:build
```

Font rebuild (after glyph-engine changes):

```bash
cargo run -p doldskrift-cli -- font build --out fonts/generated
```

Surfaces fixtures:

```bash
npm run surfaces:fixtures
# or: cargo run -p doldskrift-font --example gen_surfaces_fixtures
```

Golden vectors in `spec/vectors/` are authoritative. Both Rust and JavaScript MUST pass them.
Protocol constants: edit `spec/protocol-constants.json`, then sync `packages/doldskrift-js/src/protocol-constants.json` and related Rust/`types.ts` constants.

### Local install health

```bash
cargo install --path crates/doldskrift-cli
dold doctor
dold version --verbose
```

## Design proposals (DEPs) and ADRs

1. Open a draft under `deps/DEP-XXXX-….md` following DEP-0001 for protocol/engine changes.
2. Architecture / product decisions also use `docs/adr/`.
3. Link related spec sections and list vector impact.
4. Wait for maintainer acceptance before treating behavior as normative.

Small bugfixes and docs do not need DEPs.

## Pull requests

- Prefer small, focused changes.
- Add tests for codec, container, mapping, validate/inspect JSON, and (when touched) fingerprint/distance changes.
- Update `CHANGELOG.md` under **[0.5.0] — Unreleased**.
- Update docs when CLI flags, MIME types, or public APIs change (`docs/reference/cli.md` for CLI).
- Do not publish crates/npm packages unless maintainers authorize a release.
- Never weaken security wording to make the project sound like encryption.
- Keep the workspace at **five crates** (see [ADR-0003](docs/adr/ADR-0003-crate-consolidation.md)).

### Checklist for mapping / container PRs

- [ ] `cargo test --workspace` / `cargo clippy -D warnings`
- [ ] JS vector tests (`npm test`)
- [ ] `dold conformance` / `dold validate` on sample `.dsk`
- [ ] Spec prose matches algorithms
- [ ] Errors remain typed; no new panics on malformed input

### Checklist for font / MGE / surfaces PRs

- [ ] Determinism: same inputs → same geometry/fingerprint
- [ ] Project mark `U+E1F0` still generates as `DSK_PROJECT_MARK`
- [ ] Specimens / brand assets regenerated if mark geometry changed
- [ ] Surface fixtures regenerated if gallery SVGs changed

### Checklist for Neural / Protected PRs

- [ ] Neural path stays fail-closed on unknown grammar
- [ ] No fake encryption claims; Protected stubs remain honest about missing AEAD
- [ ] No multi-GB weights in default CI

## Dependency / SBOM notes

- `deny.toml` configures [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) (licenses, advisories, sources).
- CI runs `cargo-deny` with `continue-on-error: true` until the allowlist is fully hardened — treat failures as release blockers when cutting a tag.
- For a local SBOM-style inventory: `cargo tree` / `cargo deny check` from the repo root.

## Fuzzing

```bash
cargo fuzz run decoder
cargo fuzz run container
cargo fuzz run mapping
```

Arbitrary bytes may error; they must never panic uncontrollably.

## Code of conduct

See [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

Contributions are dual-licensed Apache-2.0 OR MIT unless otherwise stated.
