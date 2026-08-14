<p align="center">
  <img src="assets/brand/logo-horizontal.svg" alt="Doldskrift" width="420" />
</p>

<h1 align="center">Doldskrift <img src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/svg/1f1f8-1f1ea.svg" alt="Sweden" width="28" height="28" /></h1>

<p align="center">
  <a href="https://github.com/doldskrift/doldskrift/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/doldskrift/doldskrift/ci.yml?branch=main&label=CI" alt="CI" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0%20%7C%20MIT-blue" alt="License" /></a>
  <img src="https://img.shields.io/badge/MSRV-1.75-orange" alt="MSRV" />
  <img src="https://img.shields.io/badge/rustc-stable-orange" alt="rustc" />
  <a href="https://doldskrift.github.io/doldskrift/"><img src="https://img.shields.io/badge/site-GitHub%20Pages-0F6B5C" alt="GitHub Pages site" /></a>
  <a href="https://thanks.dev/u/gh/theworker02"><img src="https://img.shields.io/badge/thanks.dev-theworker02-0F6B5C" alt="thanks.dev" /></a>
  <img src="https://img.shields.io/badge/protocol-DSK%2F1%20%7C%20DSK%2F2%20%7C%20DSK%2F3-0F6B5C" alt="Protocol" />
  <img src="https://img.shields.io/badge/crates.io-not%20published-lightgrey" alt="crates.io (not published)" />
</p>

<p align="center"><strong>Machine-native visual information.</strong><br/>
Encode structured information into a visual writing system designed for deterministic machine reconstruction.</p>

<p align="center">
  <strong>Name (Swedish):</strong> <em>dold</em> (hidden / concealed) + <em>skrift</em> (writing / script)<br/>
  → “hidden writing” / “concealed script” · <em>Maskinskriven betydelse — inte hemlighet.</em><br/>
  <sub>Machine-written meaning — not secrecy. Hidden ≠ encrypted.</sub>
</p>

> Badge / Pages URLs target the public GitHub org/repo `doldskrift/doldskrift`. This working tree may not have a git remote yet.

> **Open** and **Neural** are **not encryption**. Visual illegibility and learned grammars are not confidentiality.
> **Protected** *is* AEAD + Gate authorization when it ships — glyphs encode ciphertext. Today: refuse stubs only.
> Until then: encrypt with established cryptography first, then optionally represent the result with Open-mode Doldskrift.

---

## What is Doldskrift?

An independent open-source **platform** for machine-native visual information: protocol, glyphs, recognition, Lab CLI, and demos — one architecture with three profiles.

```bash
dold guide           # in-CLI onboarding
dold about           # etymology + profiles
dold museum          # protocol archaeology
```

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

| Name | Role |
|------|------|
| **DSK** | Protocol (DSK/1 stable, DSK/2 experimental, DSK/3 Neural + Protected) |
| **LSG** | Latent Structure Grammar (Neural intermediate) |
| **DSK-R** | Neural readers (DeterministicBaseline / Mock today) |
| **Gate** | Authorized runtime for Protected objects |
| **MGE** | Machine Glyph Engine |
| **DVE** | Doldskrift Vision Engine |
| **Forge** | Alphabet / glyph optimization |
| **Lens** | Flagship visual demonstration |
| **Lab** | Developer tooling (`dold`) |

The official logo **is** glyph `DSK_PROJECT_MARK` (`U+E1F0`) — a real machine glyph, not decoration.

---

## Profiles: Open · Neural · Protected

| Profile | What it is | Shipping? |
|---------|------------|-----------|
| **Open** | Deterministic codec + glyphs (DSK/1–2). Visual concealment, not secrecy. | Yes (prototype) |
| **Neural** | Learned LSG + compatible reader (DSK/3). Fail closed on unknown grammar. **Not encryption.** | Foundation stubs + DeterministicBaseline |
| **Protected** | AEAD ciphertext + Gate authorization. Glyphs encode ciphertext. | Refuse stubs only — **no AEAD yet** |

```text
DATA → DSK → MGE → VISUAL MEDIUM → DVE → DSK → DATA
```

---

## Demo: humans vs agents

![Humans see abstract glyphs; agents recover plaintext](assets/brand/human-vs-agent.svg)

| Reader | Experience |
|--------|------------|
| **Humans** | Structured abstract marks — geometry, not Latin |
| **Agents** | Deterministic decode: symbols → bytes → original data |

![Encode pipeline](assets/brand/encode-decode-pipeline.svg)

### Real specimen

Input:

```text
Deploy worker seven.
```

![Doldskrift rendering of “Deploy worker seven.”](assets/brand/readme-demo.svg)

Decode recovers the exact original string.

Modes:

![Visual, encoded, and session modes](assets/brand/modes-specimen.svg)

---

## Install · Try · Docs

| | |
|--|--|
| **Install** | `cargo install --path crates/doldskrift-cli` then **`dold doctor`** |
| **Onboard** | `dold guide` · `dold demo -o ./dold-demo` |
| **Try (Open)** | encode → validate → inspect → decode · `dold convert` · [Lens](https://doldskrift.github.io/doldskrift/lens.html) / [Surfaces](https://doldskrift.github.io/doldskrift/surfaces.html) |
| **Neural** | `dold neural encode/render/reconstruct` · [Neural](https://doldskrift.github.io/doldskrift/neural.html) |
| **Docs** | [`docs/`](docs/) · [CLI](docs/reference/cli.md) · [Pages](https://doldskrift.github.io/doldskrift/) |

```bash
cargo install --path crates/doldskrift-cli
dold guide                  # product orientation
dold doctor                 # install health + constants sync
dold self-test              # encode/decode/validate + protected refuse
dold demo -o ./dold-demo    # .dsk + postcard + mark pack
dold commands               # curated catalog (alias: topics)
dold version --verbose

echo "Hello from Doldskrift" | dold encode > message.dsk   # alias: dold e
dold pipeline --text "Hello from Doldskrift" -o message.dsk
dold validate message.dsk
dold inspect --json message.dsk
dold decode message.dsk     # alias: dold d

dold convert --from text --to postcard --text "Deploy worker seven." -o card.svg
dold convert --from postcard --to dsk -i card.svg -o card.dsk

dold logo --out assets/brand/logo-mark.svg
dold config init

# Shell completions (optional)
dold completion powershell > _dold.ps1

# Agent-friendly errors
$env:DOLD_JSON_ERRORS = "1"   # PowerShell → stderr doldskrift.error/1
```

### End-to-end paths

1. **Open:** `encode` → `validate` → `.dsk` / MGE specimen → Lens or Surfaces → `decode`
2. **Neural:** `dold neural encode` → `render` (SVG carrier) → `reconstruct` (DeterministicBaseline)
3. **Protected:** `encode --mode protected` → `inspect` shows Readable: No → `decode` / `open` refuse (AEAD not shipping)

Interactive demos — **static GitHub Pages** (no separate hosted web app):

Live: [https://doldskrift.github.io/doldskrift/](https://doldskrift.github.io/doldskrift/)

```bash
npm install
npm run site:preview   # build site/dist + local static preview
```

---

## Unexpected surfaces

Open demos that still obey the honesty model (Open ≠ encryption; hashes ≠ signatures):

| Command | What it does |
|---------|----------------|
| `dold` (no args) | Tiny braille / project-mark fingerprint |
| `dold radio exchange "ping"` | Two-agent visual packet |
| `dold echo` / `--verify` | Self-describing document + SHA-256 glyph strip |
| `dold ambient` / `--decode` | Constellation SVG Open channel |
| `dold postcard` / `--read` | Machine postcard |
| `dold convert` | Bridge text/dsk ↔ postcard/ambient/seal |
| `dold museum` / `dold about` | Archaeology + Swedish etymology |
| `dold render --tty` | Braille / block art without a browser |

Site gallery: [Surfaces](https://doldskrift.github.io/doldskrift/surfaces.html). Full list: `dold surfaces list` / `dold guide surfaces`.

---

## Libraries

**Rust** (path dependency until crates.io; workspace `0.4.0` / Unreleased 0.5.0 work):

```toml
[dependencies]
doldskrift = { path = "crates/doldskrift" }
```

```rust
use doldskrift::{encode, decode, Mode};

let opaque = encode("Hello, agent!").unwrap();
assert_eq!(decode(&opaque).unwrap(), "Hello, agent!");
let _ = Mode::Encoded;
```

**JavaScript** — primary SDK `@doldskrift/core`; thin `@doldskrift/web` / `@doldskrift/react` adapters; generated WASM (`doldskrift-bindings`). Rust owns the protocol truth.

---

## Repository layout

```text
crates/          # ≤5 Rust crates (core, cli, vision, font, bindings)
packages/        # @doldskrift/core (+ thin web/react) · WASM bindings
site/            # GitHub Pages demos
docs/            # Product + protocol + neural docs map
spec/            # Normative DSK/1 + protocol-constants.json
training/        # Python experiments only (not a second protocol)
```

---

## Documentation

| Resource | Path |
|----------|------|
| Docs portal | [`docs/`](docs/) |
| CLI reference | [`docs/reference/cli.md`](docs/reference/cli.md) |
| Architecture | [`ARCHITECTURE.md`](ARCHITECTURE.md) |
| Roadmap | [`ROADMAP.md`](ROADMAP.md) |
| Security | [`SECURITY.md`](SECURITY.md) |
| Brand kit | [`docs/brand.md`](docs/brand.md) |
| Static site | [`site/`](site/) · [live](https://doldskrift.github.io/doldskrift/) |

---

## Status (honest)

| Component | Maturity |
|-----------|----------|
| DSK/1 codec + `.dsk` | Stable prototype |
| DSK/2 semantics | Experimental |
| DSK/3 Neural (LSG + readers) | Foundation stubs ([ADR-0007](docs/adr/ADR-0007-dsk3-neural-learned-machine-grammar.md)) |
| DSK/3 Protected + Gate | Design + refuse stubs ([ADR-0006](docs/adr/ADR-0006-gate-protected-visual-objects.md)) |
| MGE glyph engine | Candidate (evolving) |
| DVE vision | Experimental |
| WASM / CLI / Lens | Beta / shipping on Pages |

Open-mode visual illegibility is not a security boundary. See [`SECURITY.md`](SECURITY.md).

---

## Support / Funding

- **thanks.dev:** [https://thanks.dev/u/gh/theworker02](https://thanks.dev/u/gh/theworker02)
- **GitHub funding file:** [`.github/FUNDING.yml`](.github/FUNDING.yml)
- **Site:** [Funding page](https://doldskrift.github.io/doldskrift/funding.html)
- **CLI:** `dold funding` · `dold funding --json`

Bugs and docs: [`SUPPORT.md`](SUPPORT.md). Commercial collaboration: [Partners](https://doldskrift.github.io/doldskrift/partners.html).

---

## License

Apache-2.0 OR MIT — see [`LICENSE`](LICENSE).

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
