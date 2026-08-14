# Changelog

## [0.5.0] - 2026-08-14

Company-grade product polish: one present-tense platform architecture (Open · Neural · Protected), consolidated JS packages, Lab DX commands, brand/funding consistency. Neural remains **foundation** (no trained Gemma); Protected remains **refuse stubs** (no AEAD).

### Added

- **Product narrative merge:** README / ROADMAP / ARCHITECTURE / docs portal speak as one platform (not “bolted Phase V”); ADRs stay historical
- **`dold guide [topic]`** — in-CLI onboarding (`quickstart`, `honesty`, `surfaces`, `agents`, `swedish`, `architecture`)
- **`dold demo [-o] [--text]`** — one-shot Open product pack (`.dsk` + postcard + logo mark + README)
- **`dold convert --from --to`** — bridge `text`/`dsk` ↔ `postcard`/`ambient`/`seal` (Protected refuses)
- **`dold config show|path|init|set`** — `.doldskrift/config.json` (`doldskrift.config/1`)
- **Structured CLI errors:** `DOLD_JSON_ERRORS=1` → stderr `doldskrift.error/1` (code, message, hint)
- **`dold doctor` constants sync** — Rust ↔ `spec/protocol-constants.json` ↔ JS mirror when run from repo root
- **JS consolidation:** browser helpers + CSS in `@doldskrift/core`; `@doldskrift/web` is a thin re-export adapter
- **Funding.yml:** native `github: [theworker02]` + `thanks_dev: u/gh/theworker02`
- **Docs portal map** with logo; product / neural / consolidation notes reframed for the unified architecture
- **CLI workflows:** `dold commands` / `topics` (curated catalog); `dold pipeline`; `dold batch encode|validate`; `dold self-test`; `dold surfaces list`; `dold schema`; `dold init [--sample]`; encode/decode aliases `e`/`d`; category-oriented root `--help`
- **Funding:** [`.github/FUNDING.yml`](.github/FUNDING.yml) → [thanks.dev/u/gh/theworker02](https://thanks.dev/u/gh/theworker02); `dold funding [--json]`; site `funding.html` + footer Sponsor/thanks.dev; npm `"funding"` on JS packages; `@doldskrift/core` `fundingInfo()` / `THANKS_DEV_URL` / `BRAND_TOKENS`
- **`dold brand`** — brand kit export (mark SVG, tokens, sync checklist) via `doldskrift_font::brand_kit_manifest_json`
- **`doldskrift::runtime::VersionInfo`** / `version_info_json()`; CLI `dold version --verbose` prints core protocol/font/mge; WASM `wasm_version_info`
- **Vision:** `ScanEngine::SvgGeometry` (`--engine svg`/`dve2`); structured `confidence_buckets` on `ScanReport`; `scan --json` for full reports
- **Pages polish:** `social-card.svg` in `site/public/`; OG image meta; workflow verifies `funding.html` + install + logo-mark
- **`dold validate` / `dold fmt`** — structural validation (`doldskrift.validate/1`) and canonical `.dsk` rewrite (`--check`)
- **Richer `dold inspect --json`** — `doldskrift.inspect/1` with `profile`, `meta_keys`, `agent_hints`, Protected stub flags (still never embeds plaintext)
- **`dold completion`** — clap shell completions (bash/zsh/fish/powershell/elvish)
- **`dold version --verbose`** — target / profile / git / rustc build metadata
- **`dold doctor`** open-pipeline smoke (encode → validate → inspect JSON → fmt) + install-health framing
- Spec index [`spec/README.md`](spec/README.md); install page on site; Demo hub (Lens / Surfaces / Neural Lens)
- End-to-end examples polish (`examples/hello-world`, `agent-exchange`, `neural-roundtrip`)
- Root `Makefile` + npm `check` / `fixtures` scripts for one-liner quality bar
- **Lens rotation hardening:** rotate-within-cell draw + multi-angle deskew in `site/src/pixel-vision.js`; evidence panel reports strategy / mean deskew (modest rot can round-trip; large rot may fail honestly)
- **Surfaces fixture regen:** `cargo run -p doldskrift-font --example gen_surfaces_fixtures` / `npm run surfaces:fixtures`; CI smoke step; gallery cards for kaleidoscope / timeline / seal
- **`dold kaleidoscope`** — 6-fold Open mandala (decode via `--decode`)
- **`dold timeline`** — multi-epoch living strip (`--epochs`, scrub `--epoch` on decode)
- **`dold seal`** — wax-seal digest mark (`--verify`; integrity ≠ signature)
- **`dold compare`** — Open payload equality for two surface SVGs (`--json`)
- **`dold ambient --animate`** — SMIL star twinkle (carrier decode unchanged)
- **DVE/2 SVG geometry observation:** `perceive_svg_geometry` / `analyze_svg_geometry` (path/circle/line counts; still no camera)
- **Protocol constants JSON:** `spec/protocol-constants.json` (+ JS package mirror)
- **Agent exchange example:** `examples/agent-exchange/` + `doldskrift-font` example `agent_exchange`
- Neural Lens / Unknown Page: clearer DeterministicBaseline reconstruct messaging
- **Swedish brand identity:** README + site etymology (*dold* + *skrift* → concealed script), 🇸🇪 flag near title/chrome, tagline *Maskinskriven betydelse — inte hemlighet.*
- `dold about` — etymology, project mark fingerprint, Open / Neural / Protected profile list (`--json`)
- Museum etymology exhibit + profile pointer; mesh / flicker SVGs embed subtle `DSK_PROJECT_MARK`
- Brand mark regenerated via `dold logo` and synced to `site/public/` + `packages/*/logo.svg`
- README demo specimen regenerable via `cargo run -p doldskrift-font --example gen_readme_demo` (docs path fixed)
- README / package badges aligned to `ci.yml`, MSRV 1.75, license, GitHub Pages
- **DSK/3 Neural foundation:** `doldskrift::neural` (LSG types, SemanticComposer, ObservationGraph, ReaderBackend, profiles, Gate capability stubs)
- DeterministicBaseline / MockReader exact-match CI path (`tests/neural_pipeline.rs`)
- MGE/5 Neural plan stubs (`MGE5-NEURAL`) + **deterministic SVG carrier** (`render_plan_svg` / `dold neural render --format svg`)
- Tiny Neural golden JSON (`crates/doldskrift/tests/goldens/neural_e0001_hello.json`)
- CLI: `dold neural`, `dold dataset generate`, `dold reader *`, `dold evaluate baseline` (stubs / Mock path)
- **Protected refuse-path stubs (ADR-0006 m1–2):** `Mode::Protected`, `encode_protected_stub`, inspect `Readable: No`, `dold decode` refusal, `dold open` → `AccessDenied` (no AEAD)
- Docs: [ADR-0007](docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md), `docs/neural/*`, model/dataset card templates
- `training/` Python experiment tree (no second protocol implementation)
- Site: Lens wired to WASM codec + pixel-template vision; Black Page WASM; Neural Lens / Unknown Page / Compatibility honest scaffolds
- **Unexpected surfaces (shipping):**
  - `dold radio send|recv|exchange` — Agent Radio visual packet directories (`.dsk` + SVG + manifest)
  - `dold echo` / `--verify` — self-describing SVG with SHA-256 glyph strip (integrity, not a signature)
  - `dold ambient` / `--decode` — constellation SVG Open channel + deterministic aurora (honest: not steganography)
  - `dold flicker` — temporal animated SVG with embedded carrier
  - `dold live --epoch` — living document (MGE/4 variants change; Open + DeterministicBaseline recover)
  - `dold mesh` / `--html` — Neural constellation map; interactive HTML node inspect
  - `dold postcard` / `--read` — machine postcard (constellation + digest + glyph strip)
  - `dold duet` / `--decode` — interleaved A/B Open channels
  - `dold spectrogram` / `--decode` — symbol-id frequency-like strip
  - `dold notarize` / `--verify` — SHA-256 margin marks (integrity ≠ signature)
  - `dold handshake visual` — Open capability negotiation strip
  - `dold` with no args — braille / mark fingerprint delight
  - `dold museum` — protocol archaeology exhibit list
  - `dold render --tty` — braille / block terminal fallback
  - `dold scan --diff` — differential vision report + ASCII confidence histogram
- Site **Surfaces** gallery (`surfaces.html`) with ambient live sketch + downloadable fixtures
- `dold doctor` reports neural/protected/wasm/site brand readiness + new surfaces
- JS package constants synced: `FONT_VERSION=2`, `DSK_PROJECT_MARK`, handshake tokens, `PROTOCOL_VERSION_PROTECTED`

### Changed

- **ROADMAP / README / ARCHITECTURE:** present-tense unified platform; deprecate “bolted Phase V” product language (milestones remain under Neural / Protected tracks)
- **Museum eras** labeled by architecture layer/profile, not phase numbers
- **JS package roles** documented: core = SDK; web/react = adapters; wasm = generated Rust bridge
- Root `npm run build` includes `@doldskrift/web`
- CLI: flatten new surfaces into `ExtraSurfaceCommands` + Windows MSVC `/STACK:8MB` (`.cargo/config.toml`) so debug `dold --help` does not overflow
- Security / README language: Open · Neural · Protected (Neural ≠ encryption); etymology clarified as visual concealment, not crypto
- Removed leftover shim crate directories — workspace stays at **5 members**
- Brand mark shared across root README, crate/package READMEs, and `site/public/` (single source: `assets/brand/logo-mark.svg`)
- CLI reference and guides cover `guide` / `demo` / `convert` / `config` / JSON errors
- Clippy `-D warnings` clean across workspace (incl. benches); `dold query` args reordered for clap (`path` then `-i`)

### Deferred (honest)

- Trained Gemma/LoRA inference, full MGE/5 geometry, camera DVE/2, multi-GB checkpoints, published OCR bake-off numbers
- Real AEAD/Gate crypto (`protected-crypto` feature not started)
- npm publish (Rust crates on crates.io as of 0.5.0; JS packages remain unpublished)
- Enforcing cargo-deny as a hard CI gate (currently advisory)
- Replacing `site/src/doldskrift.js` with WASM / `@doldskrift/core` only

## [0.4.0] — Unreleased

### Added

- Phase IV product definition: four-layer architecture (Semantic / Protocol / Visual / Recognition)
- Platform naming: DSK, MGE, DVE, Forge, Lens, Lab
- Five-crate workspace (`doldskrift`, `cli`, `vision`, `font`, `bindings`)
- Site: **Lens**, **Black Page**, Product, Partners; restrained primary nav
- README 4.0 with encode/decode visuals and honest status table
- ROADMAP, SUPPORT, TRADEMARKS, project brief, privacy, dependency policy, ADRs
- Automated internal link checker (`tools/check-links.mjs`) in CI
- Experimental DSK/2 semantic value modules
- MGE/4 research modules (variants, signatures, familiarity/repetition proxies)
- Forge scorecard (`dold forge`) — measured values only
- DVE/1 `VisionEngine` + feature-map visual round-trip test
- CLI: `doctor`, `query`, `forge`
- `deny.toml` (+ optional cargo-deny CI)
- Phase V design docs: [ADR-0006](docs/adr/ADR-0006-gate-protected-visual-objects.md) (Gate + Protected Visual Objects), [Protected mode](docs/product/protected-mode.md), [Gate stub](docs/engineering/gate.md) — **docs only; no AEAD implementation**

### Changed

- Architecture and branding aligned to “machine-native visual information platform”
- Navigation prioritized: Product · Demo · Docs · Research
- Legacy core/codec/document/protocol crates removed from workspace members
- Security / product copy distinguishes **Open** (not encryption) vs **Protected** DSK/3 (encryption + Gate; not shipping)

## [0.2.0] — 2026-08-12

### Added

- **MGE/2** machine glyph engine: zone anatomy (N/E/S/W/C), DSKG-1 fingerprints, GRE/1 redundancy, integrity markers, `glyph_distance`, recognition confidence
- `DSK_PROJECT_MARK` (`U+E1F0`) — logo is a valid writing-system glyph
- Font families: DoldskriftText, DoldskriftMono, DoldskriftMicro; density low/normal/high
- CLI: `inspect` (boxed + `--json`), `explain`, `scan`, `pack`/`unpack`, `bench vision`, `render --debug` / `--density`
- Structural vision pipeline + measured synthetic benchmarks
- Brand assets under `assets/brand/`
- Documentation tree, ARCHITECTURE, GOVERNANCE, DEPs
- Machine Reader demonstration, Glyph Explorer
- GitHub issue templates, Dependabot, PR template

### Changed

- Workspace version → 0.2.0
- Font grammar version → 2 (aligned with MGE/2)
- Containers self-describe `glyph_engine` / `payload_type` in header metadata

## [0.1.0] — 2026-08-12

### Added

- Initial DOLDSKRIFT/1 codec, CLI, fonts, JS SDK, site, golden vectors
