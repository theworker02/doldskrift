# CLI reference (`dold`)

Complete command surface for the local toolchain. Open / Neural modes are **not encryption**. Protected containers refuse plaintext decode until real AEAD ships.

Install: `cargo install --path crates/doldskrift-cli`  
Onboarding: `dold guide` · Health: `dold doctor` · `dold self-test`  
Catalog: `dold commands` (alias `topics`)  
Build metadata: `dold version --verbose`  
Shell completions: `dold completion powershell` (also `bash` / `zsh` / `fish` / `elvish`)  
Agent errors: set `DOLD_JSON_ERRORS=1` → stderr `doldskrift.error/1`

## Workflows

| Command | Purpose |
|---------|---------|
| `guide [topic]` | In-CLI onboarding (`quickstart`, `honesty`, `surfaces`, `agents`, `swedish`, `architecture`). |
| `demo [-o] [--text]` | One-shot Open product pack: `.dsk` + postcard SVG + logo mark + README. |
| `convert --from --to [-i] [-o] [--text] [--seed]` | Bridge `text`/`dsk` ↔ `postcard`/`ambient`/`seal` (Open only; Protected refuses). |
| `config show\|path\|init\|set` | Project defaults in `.doldskrift/config.json` (`doldskrift.config/1`). |
| `commands` / `topics` | Curated grouped listing of the full command surface. |
| `pipeline [--text] [-o] [--render] [--json]` | One-shot Open path: encode → validate → inspect → optional specimen SVG. |
| `batch encode <dir> [-o]` | Encode every `*.txt` in a directory to `.dsk`. |
| `batch validate <dir> [--json]` | Validate every `*.dsk` in a directory. |
| `self-test` | Built-in smoke: encode/decode/validate round-trip + Protected refuse + inspect schema. |
| `surfaces list` | Open visual surfaces with one-liners. |
| `schema [--json]` | Print inspect/validate/error/config JSON envelope names and field shapes. |
| `init [dir] [--sample]` | Scaffold `.doldskrift/` (+ config + optional sample). |

## Document lifecycle

| Command | Purpose |
|---------|---------|
| `encode` / `e` `[--mode visual\|encoded\|session\|protected] [--raw] [--seed] [--session-id]` | Text → `.dsk` (or raw PUA with `--raw`). `protected` writes a **stub** (not AEAD). |
| `decode` / `d` `[--seed]` | `.dsk` / raw → text. Protected → clear refusal. |
| `inspect [--json] [--reveal]` | Metadata only by default. `--json` emits `doldskrift.inspect/1` for agents. `--reveal` Open plaintext only; Protected refuses. |
| `validate [--json]` | Structure, version/mode, checksum, decode expectations (`doldskrift.validate/1`). |
| `fmt [--check] [-o]` | Canonical re-serialize of a `.dsk`. `--check` exits non-zero if dirty. |
| `verify` | Lighter checksum + re-serialize sanity check. |
| `explain [--reveal]` | Educational breakdown. |
| `pack` / `unpack` | Binary payloads in `.dsk`. |
| `open [--capability]` | Gate stub: Protected → `AccessDenied` (no AEAD yet). |
| `query <path> [-i file] [--json]` | Experimental JSON/semantic path query (stdin or `-i`). |

## Rendering & fonts

| Command | Purpose |
|---------|---------|
| `render [--density] [--debug] [--tty]` | Mode transform / debug SVG; `--tty` braille/block art. |
| `specimen [-o] [--mode] [--density]` | Glyph-strip SVG from text. |
| `logo [-o] [--seed]` | Export `DSK_PROJECT_MARK` (`U+E1F0`) SVG. |
| `brand [--out brand-kit] [--json]` | Brand kit: mark + tokens + sync checklist (`docs/brand.md`). |
| `font build [--out] [--seed]` | Generate MGE/2 font artifacts. |
| `font inspect [--catalog]` | Summarize glyph catalog JSON. |
| `forge [--seed] [--profile] [--json]` | Single-generation alphabet scorecard (measured values only). |

## Vision

| Command | Purpose |
|---------|---------|
| `scan --engine structural\|svg [--diff] [--json]` | Vision pipeline; `svg`/`dve2` = DVE/2 SVG geometry; `--json` includes `confidence_buckets`. |
| `vision <image>` | Alias of structural scan. |
| `bench vision [--samples]` | Measured synthetic recognition bench (not published OCR bake-off). |

## Neural (foundation)

| Command | Purpose |
|---------|---------|
| `neural encode` | Semantic → LatentGraph plan (Mock/Deterministic path). |
| `neural render [--format svg]` | MGE/5 deterministic SVG carrier (not a trained alphabet). |
| `neural reconstruct` | DeterministicBaseline / Mock reader. |
| `neural inspect` | Plan / observation metadata. |
| `dataset generate` | Synthetic Neural fixtures (JSON + SVG). |
| `reader list\|info\|install\|verify` | Reader stubs — **no** multi-GB weight auto-download. |
| `evaluate baseline` | Exact-match harness stub. |

Neural ≠ encryption. Wrong/unknown grammar fails closed.

## Protocol & identity

| Command | Purpose |
|---------|---------|
| `conformance [--vectors]` | Golden vectors under `spec/vectors/`. |
| `handshake offer\|negotiate\|visual` | Open capability negotiation (+ visual strip). |
| `about [--json]` | Swedish etymology + Open / Neural / Protected profiles. |
| `funding [--json]` | Sponsor / [thanks.dev](https://thanks.dev/u/gh/theworker02) URLs (`doldskrift.funding/1`). |
| `brand [--out DIR] [--json]` | Export brand kit (mark SVG, tokens, sync checklist). |
| `museum [--json]` | Protocol archaeology exhibit list. |
| `doctor` | **Install health story** — toolchain, neural/protected refuse paths, brand/WASM/surfaces, open encode→validate→fmt. |
| `version [--verbose]` | SemVer; verbose adds core protocol/font/mge + target/profile/git/rustc. |
| `completion <shell>` | Generate shell completions. |
| *(no args)* | Quiet-mode braille / project-mark fingerprint. |

## Unexpected surfaces (Open honesty model)

| Command | Purpose |
|---------|---------|
| `radio send\|recv\|exchange` | Agent Radio visual packet directories. |
| `echo` / `--verify` | Self-describing SVG + SHA-256 glyph strip (integrity ≠ signature). |
| `ambient` [`--animate`] / `--decode` | Constellation SVG Open channel. |
| `flicker` | Temporal animated SVG with embedded carrier. |
| `live --epoch` | Living document (MGE/4 variants); Open + baseline recover. |
| `mesh` / `--html` | Neural constellation map / interactive HTML. |
| `postcard` / `--read` | Machine postcard (constellation + digest + glyph strip). |
| `duet` / `--decode` | Interleaved A/B Open channels. |
| `spectrogram` / `--decode` | Symbol-id frequency-like strip. |
| `notarize` / `--verify` | SHA-256 margin marks (integrity ≠ signature). |
| `kaleidoscope` / `--decode` | Six-fold Open mandala. |
| `timeline` / `--epochs` / `--epoch` | Multi-epoch living strip. |
| `seal` / `--verify` | Wax-seal digest mark (integrity ≠ signature). |
| `compare a.svg b.svg [--json]` | Open payload equality across surface SVGs. |

Hashes and digests are **integrity** aids, not signatures. See [`SECURITY.md`](../../SECURITY.md).

## Agent-oriented JSON

```bash
dold inspect --json message.dsk   # schema: doldskrift.inspect/1
dold validate --json message.dsk  # schema: doldskrift.validate/1
dold about --json
dold museum --json
```

`inspect --json` includes `profile`, `meta_keys`, `agent_hints`, and never embeds decoded plaintext.
